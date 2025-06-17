use thiserror::Error;

#[derive(Debug, Error)]
pub enum AreSameError {
    #[error("Failed to get file checksum: {0}")]
    FileChecksumError(#[from] std::io::Error),

    #[error("Failed to get object checksum: {0}")]
    ObjectChecksumError(#[from] ObjectChecksumError),
}

#[derive(Debug, Error)]
pub enum ObjectChecksumError {
    #[error("Failed to get object attributes: {0}")]
    GetObjectAttributesError(
        #[from] aws_sdk_s3::operation::get_object_attributes::GetObjectAttributesError,
    ),

    #[error("Object not found: {0}")]
    ObjectNotFound(s3uri::S3Uri),

    #[error("Failed to convert to u64: {0}")]
    ToU64Error(#[from] ToU64Error),

    #[error("URI has no key: {0}")]
    UriHasNoKey(s3uri::S3Uri),
}

#[derive(Debug, Error)]
pub enum ToU64Error {
    #[error("Failed to convert to u64: {0}")]
    ConversionError(String),

    #[error("Failed to decode string: {0}")]
    DecodeError(#[from] base64::DecodeError),
}
