use crate::errors::ToU64Error;
use base64::Engine;

/// Converts a string to u64.
pub fn to_u64(s: &str) -> Result<u64, ToU64Error> {
    match base64::engine::general_purpose::STANDARD.decode(s) {
        Ok(decoded) => match decoded.try_into() {
            Ok(bytes) => Ok(u64::from_be_bytes(bytes)),
            Err(_) => Err(ToU64Error::ConversionError(
                "failed to convert to bytes".to_string(),
            )),
        },
        Err(e) => Err(e.into()),
    }
}
