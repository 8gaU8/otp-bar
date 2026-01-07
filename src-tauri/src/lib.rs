use tauri::{
    ActivationPolicy, AppHandle, Manager, 
    menu::{Menu, MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};

mod otp;
mod qr;

use otp::{generate_otp, get_otp_remaining_time, is_otp_in_warning_period};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(rename = "oathtoolExecutablePath")]
    oathtool_executable_path: Option<String>,
}

fn get_config_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".config/otp-bar")
}

fn get_config_file() -> PathBuf {
    get_config_dir().join("config.json")
}

fn list_token_ids() -> Vec<String> {
    let config_dir = get_config_dir();
    let mut token_ids = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&config_dir) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                if file_name != "config.json" {
                    token_ids.push(file_name);
                }
            }
        }
    }
    
    token_ids.sort();
    token_ids
}

fn read_token(id: &str) -> Result<String, String> {
    let config_dir = get_config_dir();
    let token_path = config_dir.join(id);
    
    fs::read_to_string(&token_path)
        .map(|s| s.trim().to_string())
        .map_err(|e| format!("Failed to read token: {}", e))
}

fn write_token_file(user_name: &str, token: &str) -> Result<(), String> {
    let config_dir = get_config_dir();
    let file_path = config_dir.join(user_name);
    
    if file_path.exists() {
        return Ok(()); // Skip if already exists
    }
    
    fs::write(&file_path, token)
        .map_err(|e| format!("Failed to write token file: {}", e))
}

fn get_timer_display_text() -> String {
    let remaining_time = get_otp_remaining_time();
    if is_otp_in_warning_period() {
        format!("⚠️ Time: {}s", remaining_time)
    } else {
        format!("⏱️ Time: {}s", remaining_time)
    }
}

async fn handle_configure(app: AppHandle) -> Result<(), String> {
    use tauri_plugin_dialog::DialogExt;
    
    if let Some(file_path) = app.dialog()
        .file()
        .add_filter("Images", &["png", "jpg", "jpeg"])
        .blocking_pick_file()
    {
        let file_path_str = file_path.to_string_lossy().to_string();
        let tokens = qr::parse_qr_and_extract_tokens(&file_path_str)?;
        
        for token_data in tokens {
            write_token_file(&token_data.name, &token_data.secret)?;
        }
        
        // Restart the application
        app.restart();
    }
    
    Ok(())
}

async fn copy_otp_to_clipboard(app: AppHandle, id: String) -> Result<(), String> {
    let token = read_token(&id)?;
    let otp = generate_otp(&token)?;
    
    use tauri_plugin_clipboard_manager::ClipboardExt;
    app.clipboard()
        .write_text(otp)
        .map_err(|e| format!("Failed to write to clipboard: {}", e))?;
    
    Ok(())
}

fn create_menu(app: &AppHandle, token_ids: &[String]) -> Result<Menu<tauri::Wry>, String> {
    let menu = MenuBuilder::new(app);
    
    // Configure item
    let configure_item = MenuItemBuilder::new("Configure (restart automatically)")
        .id("configure")
        .build(app)
        .map_err(|e| format!("Failed to create configure menu item: {}", e))?;
    
    // Quit item
    let quit_item = PredefinedMenuItem::quit(app, Some("Quit"))
        .map_err(|e| format!("Failed to create quit menu item: {}", e))?;
    
    // Separator
    let separator = PredefinedMenuItem::separator(app)
        .map_err(|e| format!("Failed to create separator: {}", e))?;
    
    // Timer item
    let timer_text = get_timer_display_text();
    let timer_item = MenuItemBuilder::new(timer_text)
        .id("timer")
        .enabled(false)
        .build(app)
        .map_err(|e| format!("Failed to create timer menu item: {}", e))?;
    
    let mut menu = menu
        .item(&configure_item)
        .item(&quit_item)
        .item(&separator)
        .item(&timer_item)
        .item(&separator);
    
    // Add token items
    for id in token_ids {
        let token = read_token(id).unwrap_or_default();
        let otp = generate_otp(&token).unwrap_or_else(|_| "ERROR".to_string());
        let text = format!("{}: {}", id, otp);
        
        let item = MenuItemBuilder::new(text)
            .id(id)
            .build(app)
            .map_err(|e| format!("Failed to create menu item: {}", e))?;
        
        menu = menu.item(&item);
    }
    
    menu.build()
        .map_err(|e| format!("Failed to build menu: {}", e))
}

async fn update_menu_periodically(app: AppHandle) {
    let mut previous_remaining_time = get_otp_remaining_time();
    
    loop {
        thread::sleep(Duration::from_millis(500));
        
        let current_remaining_time = get_otp_remaining_time();
        
        // Update timer display
        let timer_text = get_timer_display_text();
        if let Some(tray) = app.tray_by_id("main") {
            if let Some(menu) = tray.menu() {
                if let Some(timer_item) = menu.get("timer") {
                    let _ = timer_item.as_menuitem().map(|item| item.set_text(timer_text));
                }
            }
        }
        
        // Update OTP codes when period resets
        if current_remaining_time > previous_remaining_time {
            println!("OTP period reset detected, updating all OTP codes");
            
            let token_ids = list_token_ids();
            if let Some(tray) = app.tray_by_id("main") {
                if let Some(menu) = tray.menu() {
                    for id in &token_ids {
                        if let Some(menu_item) = menu.get(id) {
                            if let Ok(token) = read_token(id) {
                                if let Ok(otp) = generate_otp(&token) {
                                    let text = format!("{}: {}", id, otp);
                                    let _ = menu_item.as_menuitem().map(|item| item.set_text(text));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        previous_remaining_time = current_remaining_time;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Dockアイコンを非表示に
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);
            
            // Create tray icon
            let token_ids = list_token_ids();
            let menu = create_menu(app.handle(), &token_ids)
                .expect("Failed to create menu");
            
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(move |app, event| {
                    let item_id = event.id().as_ref();
                    
                    if item_id == "configure" {
                        let app_clone = app.clone();
                        tauri::async_runtime::spawn(async move {
                            if let Err(e) = handle_configure(app_clone).await {
                                eprintln!("Configuration error: {}", e);
                            }
                        });
                    } else if item_id != "timer" {
                        // It's a token ID
                        let id = item_id.to_string();
                        let app_clone = app.clone();
                        tauri::async_runtime::spawn(async move {
                            if let Err(e) = copy_otp_to_clipboard(app_clone, id).await {
                                eprintln!("Failed to copy OTP: {}", e);
                            }
                        });
                    }
                })
                .build(app)
                .expect("Failed to create tray icon");
            
            // Start periodic update task
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update_menu_periodically(app_handle).await;
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
