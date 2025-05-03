use base64::Engine;

/// Converts a string to u64.
pub fn to_u64(s: &str) -> Result<u64, String> {
    match base64::engine::general_purpose::STANDARD.decode(s) {
        Ok(decoded) => match decoded.try_into() {
            Ok(bytes) => Ok(u64::from_be_bytes(bytes)),
            Err(_) => Err("failed to convert to bytes".to_owned()),
        },
        Err(e) => Err(e.to_string()),
    }
}
