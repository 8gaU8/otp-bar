use std::time::{SystemTime, UNIX_EPOCH};
use data_encoding::BASE32_NOPAD;
use totp_lite::{totp, Sha1};

/// Generate a TOTP code from a base32-encoded secret
pub fn generate_otp(secret: &str) -> Result<String, String> {
    // Decode the base32 secret
    let secret_bytes = BASE32_NOPAD
        .decode(secret.to_uppercase().as_bytes())
        .map_err(|e| format!("Failed to decode base32 secret: {}", e))?;
    
    // Get current Unix timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Failed to get system time: {}", e))?
        .as_secs();
    
    // Generate TOTP code (30 second period, 6 digits)
    let code = totp::<Sha1>(&secret_bytes, timestamp, 30);
    
    Ok(format!("{:06}", code))
}

/// Calculate the remaining time in seconds for the current OTP period
/// OTP typically refreshes every 30 seconds based on Unix time
/// Returns remaining time in seconds (1-30, where 30 means start of new period)
pub fn get_otp_remaining_time() -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    
    let period = 30;
    let time_in_period = now % period;
    let remaining_time = period - time_in_period;
    
    remaining_time
}

/// Check if the OTP is in the warning period (last 10 seconds)
/// Returns true if remaining time is <= 10 seconds
pub fn is_otp_in_warning_period() -> bool {
    get_otp_remaining_time() <= 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_otp_remaining_time() {
        let remaining = get_otp_remaining_time();
        assert!(remaining > 0 && remaining <= 30);
    }

    #[test]
    fn test_warning_period() {
        // This test just verifies the function runs without panicking
        let _is_warning = is_otp_in_warning_period();
    }
}
