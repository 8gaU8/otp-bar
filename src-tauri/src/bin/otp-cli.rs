use arboard::Clipboard;
use clap::{Parser, Subcommand};
use otp_bar_lib::{config::Config, get_config_path, otp};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "otp-cli")]
#[command(about = "OTP command line interface", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display OTP with remaining time (continuously until Ctrl+C)
    Show {
        /// Token ID (optional - uses highest priority token if not specified)
        token_id: Option<String>,
    },
    /// Copy OTP to clipboard and exit
    Clip {
        /// Token ID (optional - uses highest priority token if not specified)
        token_id: Option<String>,
    },
}

fn get_token_id(config: &Config, requested_id: Option<String>) -> Result<String, String> {
    match requested_id {
        Some(id) => {
            // Verify the token exists
            if config.get_token(&id).is_some() {
                Ok(id)
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

fn load_token_secret(token_id: Option<String>) -> Result<(String, String), String> {
    let config_path = get_config_path();
    let config = Config::load(&config_path)?;

    let token_id = get_token_id(&config, token_id)?;
    let secret = config
        .get_token(&token_id)
        .ok_or_else(|| format!("Token '{}' not found", token_id))?
        .clone();

    Ok((token_id, secret))
}

fn show_otp(token_id: Option<String>) -> Result<(), String> {
    let (token_id, secret) = load_token_secret(token_id)?;

    println!("Showing OTP for: {}", token_id);
    println!("Press Ctrl+C to stop\n");

    loop {
        let otp_code = otp::generate_otp(&secret)?;
        let remaining_time = otp::get_otp_remaining_time();

        // Clear the line and print OTP with remaining time
        print!("\r{} ({}s remaining)  ", otp_code, remaining_time);
        io::stdout().flush().unwrap();

        // Sleep for 500ms before updating
        thread::sleep(Duration::from_millis(500));
    }
}

fn clip_otp(token_id: Option<String>) -> Result<(), String> {
    let (token_id, secret) = load_token_secret(token_id)?;

    let otp_code = otp::generate_otp(&secret)?;

    let mut clipboard = Clipboard::new().map_err(|e| format!("Failed to access clipboard: {}", e))?;
    clipboard
        .set_text(&otp_code)
        .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;

    println!("Copied OTP for '{}' to clipboard: {}", token_id, otp_code);

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Show { token_id } => show_otp(token_id),
        Commands::Clip { token_id } => clip_otp(token_id),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
