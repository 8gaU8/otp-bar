use data_encoding::BASE32_NOPAD;
use rqrr;

#[derive(Debug, Clone)]
pub struct TokenData {
    pub name: String,
    pub secret: String,
}

/// Parse a QR code image and extract OTP tokens
pub fn parse_qr_and_extract_tokens(image_path: &str) -> Result<Vec<TokenData>, String> {
    // Load the image
    let img = image::open(image_path).map_err(|e| format!("Failed to open image: {}", e))?;

    // Convert to luma (grayscale)
    let img = img.to_luma8();

    // Prepare the image for rqrr
    let mut img_for_qr = rqrr::PreparedImage::prepare(img);

    // Find and decode QR codes
    let grids = img_for_qr.detect_grids();

    if grids.is_empty() {
        return Err("No QR code found in image".to_string());
    }

    let mut all_tokens = Vec::new();

    for grid in grids {
        let (_meta, content) = grid
            .decode()
            .map_err(|e| format!("Failed to decode QR code: {:?}", e))?;

        println!("QR Code content: {}", content);

        // Try to parse as otpauth-migration URL
        if content.starts_with("otpauth-migration://") {
            let tokens = parse_migration_url(&content)?;
            all_tokens.extend(tokens);
        } else if content.starts_with("otpauth://") {
            // Single OTP URL
            let token = parse_otpauth_url(&content)?;
            all_tokens.push(token);
        } else {
            return Err("QR code does not contain OTP data".to_string());
        }
    }

    if all_tokens.is_empty() {
        return Err("No tokens found in QR code".to_string());
    }

    Ok(all_tokens)
}

fn parse_otpauth_url(url: &str) -> Result<TokenData, String> {
    // Parse otpauth://totp/AccountName?secret=BASE32SECRET&issuer=Issuer
    let url = url::Url::parse(url).map_err(|e| format!("Failed to parse URL: {}", e))?;

    let path = url.path().trim_start_matches('/');
    let name = urlencoding::decode(path)
        .map_err(|e| format!("Failed to decode name: {}", e))?
        .to_string();

    let secret = url
        .query_pairs()
        .find(|(key, _)| key == "secret")
        .map(|(_, value)| value.to_string())
        .ok_or_else(|| "No secret found in URL".to_string())?;

    Ok(TokenData { name, secret })
}

fn parse_migration_url(url: &str) -> Result<Vec<TokenData>, String> {
    use base64::Engine;

    // Parse the URL
    let parsed_url =
        url::Url::parse(url).map_err(|e| format!("Failed to parse migration URL: {}", e))?;

    // Get the data parameter
    let data_param = parsed_url
        .query_pairs()
        .find(|(key, _)| key == "data")
        .map(|(_, value)| value.to_string())
        .ok_or_else(|| "No data parameter in migration URL".to_string())?;

    // Decode the base64 data
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(data_param.as_bytes())
        .map_err(|e| format!("Failed to decode base64: {}", e))?;

    // Parse the protobuf data
    parse_migration_payload(&decoded)
}

// Simple protobuf parser for Google Authenticator migration format
fn parse_migration_payload(data: &[u8]) -> Result<Vec<TokenData>, String> {
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < data.len() {
        if data[i] == 0x0a {
            // Field 1 (OtpParameters)
            i += 1;
            let (length, bytes_read) = decode_varint(&data[i..])?;
            i += bytes_read;

            if i + length > data.len() {
                return Err("Invalid protobuf data".to_string());
            }

            let param_data = &data[i..i + length];
            if let Ok(token) = parse_otp_parameter(param_data) {
                tokens.push(token);
            }
            i += length;
        } else {
            // Skip unknown fields
            i += 1;
            if i < data.len() {
                let wire_type = data[i - 1] & 0x07;
                match wire_type {
                    0 => {
                        // Varint
                        let (_, bytes_read) = decode_varint(&data[i..])?;
                        i += bytes_read;
                    }
                    2 => {
                        // Length-delimited
                        let (length, bytes_read) = decode_varint(&data[i..])?;
                        if i + bytes_read + length > data.len() {
                            return Err("Invalid protobuf data".to_string());
                        }
                        i += bytes_read + length;
                    }
                    _ => i += 1,
                }
            }
        }
    }

    Ok(tokens)
}

fn parse_otp_parameter(data: &[u8]) -> Result<TokenData, String> {
    let mut secret_bytes = None;
    let mut name = None;
    let mut i = 0;

    while i < data.len() {
        let field = data[i];
        i += 1;

        match field {
            0x0a => {
                // Field 1: secret (bytes)
                let (length, bytes_read) = decode_varint(&data[i..])?;
                i += bytes_read;
                if i + length > data.len() {
                    return Err("Invalid protobuf data: field extends beyond buffer".to_string());
                }
                secret_bytes = Some(data[i..i + length].to_vec());
                i += length;
            }
            0x12 => {
                // Field 2: name (string)
                let (length, bytes_read) = decode_varint(&data[i..])?;
                i += bytes_read;
                if i + length > data.len() {
                    return Err("Invalid protobuf data: field extends beyond buffer".to_string());
                }
                name = Some(String::from_utf8_lossy(&data[i..i + length]).to_string());
                i += length;
            }
            _ => {
                // Skip unknown field
                let wire_type = field & 0x07;
                match wire_type {
                    0 => {
                        // Varint
                        let (_, bytes_read) = decode_varint(&data[i..])?;
                        i += bytes_read;
                    }
                    2 => {
                        // Length-delimited
                        if i < data.len() {
                            let (length, bytes_read) = decode_varint(&data[i..])?;
                            if i + bytes_read + length > data.len() {
                                return Err("Invalid protobuf data: field extends beyond buffer".to_string());
                            }
                            i += bytes_read + length;
                        }
                    }
                    _ => {
                        if i < data.len() {
                            i += 1;
                        }
                    }
                }
            }
        }
    }

    let secret_bytes = secret_bytes.ok_or_else(|| "No secret found".to_string())?;
    let secret = BASE32_NOPAD.encode(&secret_bytes);
    let name = name.unwrap_or_else(|| "Unknown".to_string());

    Ok(TokenData { name, secret })
}

fn decode_varint(data: &[u8]) -> Result<(usize, usize), String> {
    let mut result = 0usize;
    let mut shift = 0;
    let mut i = 0;

    while i < data.len() {
        let byte = data[i];
        i += 1;

        result |= ((byte & 0x7f) as usize) << shift;

        if byte & 0x80 == 0 {
            return Ok((result, i));
        }

        shift += 7;

        if shift > 63 {
            return Err("Varint too long".to_string());
        }
    }

    Err("Incomplete varint".to_string())
}
