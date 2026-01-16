use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use tauri::{
    menu::{Menu, MenuBuilder, MenuItemBuilder, MenuItemKind, PredefinedMenuItem},
    tray::TrayIconBuilder,
    ActivationPolicy, AppHandle, Manager, Wry,
};
use tauri_plugin_dialog::FilePath;
use tauri_plugin_opener::OpenerExt;

mod config;
mod otp;
mod qr;

use config::Config;
use otp::{generate_otp, get_otp_remaining_time, is_otp_in_warning_period};

struct MenuState(Mutex<Menu<Wry>>);

fn get_config_dir() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    let config_dir = home.join(".config/otp-bar");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Could not create config directory");
    }
    config_dir
}

fn get_config_file_path() -> PathBuf {
    get_config_dir().join("config.toml")
}

fn list_token_ids() -> Vec<String> {
    let config_path = get_config_file_path();
    Config::load(&config_path)
        .map(|config| config.list_token_names())
        .unwrap_or_default()
}

fn read_token(id: &str) -> Result<String, String> {
    let config_path = get_config_file_path();
    let config = Config::load(&config_path)?;

    config
        .get_token(id)
        .cloned()
        .ok_or_else(|| format!("Token '{}' not found", id))
}

fn write_token(user_name: &str, token: &str) -> Result<(), String> {
    let config_path = get_config_file_path();
    let mut config = Config::load(&config_path).unwrap_or_else(|e| {
        eprintln!("Warning: Failed to load config ({}), using default", e);
        Config::default()
    });

    config.add_token(user_name.to_string(), token.to_string());
    config.save(&config_path)?;

    Ok(())
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

    if let Some(file_path) = app
        .dialog()
        .file()
        .add_filter("Images", &["png", "jpg", "jpeg"])
        .blocking_pick_file()
    {
        let file_path_str = match file_path {
            FilePath::Path(p) => p.to_string_lossy().to_string(),
            _ => return Err("Only file paths are supported".to_string()),
        };
        let tokens = qr::parse_qr_and_extract_tokens(&file_path_str)?;

        for token_data in tokens {
            write_token(&token_data.name, &token_data.secret)?;
        }

        // Restart the application
        reload_menu(&app);
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

fn get_otp_text(id: &String, otp: &String) -> String {
    format!("{}: {}", otp, id)
}

fn create_menu(app: &AppHandle, token_ids: &[String]) -> Result<Menu<tauri::Wry>, String> {
    let menu = MenuBuilder::new(app);

    // Configure item
    let configure_item = MenuItemBuilder::new("Load QR code")
        .id("configure")
        .build(app)
        .map_err(|e| format!("Failed to create configure menu item: {}", e))?;

    let restart_item = MenuItemBuilder::new("Apply config")
        .id("reload")
        .build(app)
        .map_err(|e| format!("Failed to create restart menu item: {}", e))?;

    let edit_config_item = MenuItemBuilder::new("Edit config")
        .id("edit_config")
        .build(app)
        .map_err(|e| format!("Failed to create edit config menu item: {}", e))?;

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
        .item(&edit_config_item)
        .item(&restart_item)
        .item(&quit_item)
        .item(&separator)
        .item(&timer_item)
        .item(&separator);

    // Add token items
    for id in token_ids {
        let token = read_token(id).unwrap_or_default();
        let otp = generate_otp(&token).unwrap_or_else(|_| "ERROR".to_string());
        let text = get_otp_text(&id, &otp);

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
        tokio::time::sleep(Duration::from_millis(500)).await;

        let current_remaining_time = get_otp_remaining_time();

        // Get current menu from state
        let menu_handle = {
            let state = app.state::<MenuState>();
            let menu = state.0.lock().unwrap();
            menu.clone()
        };

        // Update timer display
        let timer_text = get_timer_display_text();
        if let Some(menu_item) = menu_handle.get("timer") {
            if let MenuItemKind::MenuItem(item) = menu_item {
                let _ = item.set_text(timer_text);
            }
        }

        // Update OTP codes when period resets
        if current_remaining_time > previous_remaining_time {
            println!("OTP period reset detected, updating all OTP codes");

            let token_ids = list_token_ids();
            for id in &token_ids {
                if let Some(menu_item) = menu_handle.get(id) {
                    if let Ok(token) = read_token(id) {
                        if let Ok(otp) = generate_otp(&token) {
                            let text = get_otp_text(&id, &otp);
                            if let MenuItemKind::MenuItem(item) = menu_item {
                                let _ = item.set_text(text);
                            }
                        }
                    }
                }
            }
        }

        previous_remaining_time = current_remaining_time;
    }
}

fn reload_menu(app: &AppHandle) {
    match list_token_ids() {
        token_ids => {
            match create_menu(app, &token_ids) {
                Ok(new_menu) => {
                    if let Some(tray) = app.tray_by_id("main") {
                        if let Err(e) = tray.set_menu::<Menu<Wry>>(Some(new_menu.clone())) {
                            eprintln!("Failed to update tray menu: {}", e);
                        } else {
                            // Update state
                            let state = app.state::<MenuState>();
                            *state.0.lock().unwrap() = new_menu;
                            println!("Menu updated successfully");
                        }
                    } else {
                        eprintln!("Main tray icon not found");
                    }
                }
                Err(e) => eprintln!("Failed to create new menu: {}", e),
            }
        }
    }
}

fn get_token_id_from_config(config: &Config, requested_id: Option<&str>) -> Result<String, String> {
    match requested_id {
        Some(id) => {
            // Verify the token exists
            if config.get_token(id).is_some() {
                Ok(id.to_string())
            } else {
                Err(format!("Token '{}' not found", id))
            }
        }
        None => {
            // Get the highest priority token (first in the list)
            config
                .list_token_names()
                .first()
                .cloned()
                .ok_or_else(|| "No tokens configured".to_string())
        }
    }
}

fn handle_cli_show(token_id: Option<&str>) -> Result<(), String> {
    let config_path = get_config_file_path();
    let config = Config::load(&config_path)?;

    let token_id = get_token_id_from_config(&config, token_id)?;
    let secret = config
        .get_token(&token_id)
        .ok_or_else(|| format!("Token '{}' not found", token_id))?;

    println!("Showing OTP for: {}", token_id);
    println!("Press Ctrl+C to stop\n");

    loop {
        let otp_code = generate_otp(secret)?;
        let remaining_time = get_otp_remaining_time();

        // Clear the line and print OTP with remaining time
        print!("\r{} ({}s remaining)  ", otp_code, remaining_time);
        io::stdout().flush().unwrap();

        // Sleep for 500ms before updating
        thread::sleep(Duration::from_millis(500));
    }
}

fn handle_cli_clip(token_id: Option<&str>) -> Result<(), String> {
    let config_path = get_config_file_path();
    let config = Config::load(&config_path)?;

    let token_id = get_token_id_from_config(&config, token_id)?;
    let secret = config
        .get_token(&token_id)
        .ok_or_else(|| format!("Token '{}' not found", token_id))?;

    let otp_code = generate_otp(secret)?;

    // Use clipboard plugin via Tauri's clipboard manager
    // For CLI mode, we'll use a simple approach - just print the code
    // and let the user copy it manually, or we could use a system command
    
    // For macOS, we can use pbcopy
    #[cfg(target_os = "macos")]
    {
        use std::process::{Command, Stdio};
        let mut child = Command::new("pbcopy")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn pbcopy: {}", e))?;
        
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin
                .write_all(otp_code.as_bytes())
                .map_err(|e| format!("Failed to write to pbcopy: {}", e))?;
        }
        
        child
            .wait()
            .map_err(|e| format!("Failed to wait for pbcopy: {}", e))?;
    }

    // For other platforms, just print
    #[cfg(not(target_os = "macos"))]
    {
        println!("OTP code (copy manually): {}", otp_code);
    }

    println!("Copied OTP for '{}' to clipboard: {}", token_id, otp_code);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .plugin(tauri_plugin_clipboard_manager::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_cli::init())
    .setup(|app| {
        // Handle CLI arguments
        if let Ok(matches) = tauri_plugin_cli::CliExt::cli(app.handle()).matches() {
            // Check if any CLI subcommand was used
            if let Some(show_matches) = matches.subcommand.as_ref().and_then(|s| {
                if s.name == "show" { Some(s) } else { None }
            }) {
                let token_id = show_matches.matches.args.get("token_id")
                    .and_then(|v| v.value.as_str());
                
                if let Err(e) = handle_cli_show(token_id) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
                std::process::exit(0);
            }

            if let Some(clip_matches) = matches.subcommand.as_ref().and_then(|s| {
                if s.name == "clip" { Some(s) } else { None }
            }) {
                let token_id = clip_matches.matches.args.get("token_id")
                    .and_then(|v| v.value.as_str());
                
                if let Err(e) = handle_cli_clip(token_id) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
                std::process::exit(0);
            }
        }

        // If no CLI subcommand, run GUI mode
        // Dockアイコンを非表示に
        #[cfg(target_os = "macos")]
        app.set_activation_policy(ActivationPolicy::Accessory);

        // Create initial menu
        let token_ids = list_token_ids();
        let menu = create_menu(app.handle(), &token_ids).expect("Failed to create menu");

        // Manage menu state
        app.manage(MenuState(Mutex::new(menu.clone())));

        let _tray = TrayIconBuilder::with_id("main")
            .menu(&menu)
            .icon(
                app.default_window_icon()
                    .expect("Failed to get default window icon for tray; ensure a window icon is configured in the Tauri bundle")
                    .clone(),
            )
            .on_menu_event(move |app: &AppHandle, event: tauri::menu::MenuEvent| {
                let item_id = event.id().as_ref();

                if item_id == "configure" {
                    let app_clone = app.clone();
                    tauri::async_runtime::spawn(async move {
                        if let Err(e) = handle_configure(app_clone).await {
                            eprintln!("Configuration error: {}", e);
                        }
                    });
                } else if item_id == "reload" {
                    println!("Reloading config...");
                    // let app_clone = app.clone();
                    // Reload config and update menu
                    reload_menu(app);
                }else if item_id == "edit_config" {
                    let config_path = get_config_file_path();
                    let config_path_str = config_path.to_string_lossy().to_string();
                    app.opener().open_path(config_path_str, None::<&str>)
                        .map_err(|e| eprintln!("Failed to open config file: {}", e)).ok();

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
