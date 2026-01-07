use data_encoding::BASE32_NOPAD;
use std::time::{SystemTime, UNIX_EPOCH};
use totp_lite::{totp_custom, Sha1};

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
    let code = totp_custom::<Sha1>(30, 6, &secret_bytes, timestamp);

    Ok(code)
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

    #[test]
    fn test_compare_with_oathtool() {
        use totp_lite::{totp_custom, Sha1};
        use data_encoding::BASE32_NOPAD;

        // Secret: GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ (RFC 6238 test vector)
        let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
        let secret_bytes = BASE32_NOPAD.decode(secret.as_bytes()).unwrap();
        // Time: 2009-02-13 23:31:30 UTC
        let timestamp = 1234567890;

        let code = totp_custom::<Sha1>(30, 6, &secret_bytes, timestamp);
        assert_eq!(code, "005924", "Expected 005924, got {}", code);
    }
}
