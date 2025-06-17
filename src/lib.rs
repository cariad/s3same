#[doc = include_str!("../README.md")]
use aws_sdk_s3::operation::get_object_attributes::GetObjectAttributesError;
use log::{debug, error};
use std::io::Error;
use std::path::Path;

mod base64;

#[derive(Debug, PartialEq)]
/// A computed checksum.
pub enum Checksum {
    /// A CRC-64/NVME checksum.
    Crc64Nvme(u64),
}

/// Checksum type.
pub enum ChecksumType {
    Crc64Nvme,
}

pub fn checksum_type(checksum: &Checksum) -> ChecksumType {
    match checksum {
        Checksum::Crc64Nvme(_) => ChecksumType::Crc64Nvme,
    }
}

/// Gets a file's checksum.
///
/// ```rust
/// let checksum = s3same::file_checksum(
///     s3same::ChecksumType::Crc64Nvme,
///     std::path::Path::new("LICENSE"),
/// ).unwrap();
///
/// assert_eq!(checksum, s3same::Checksum::Crc64Nvme(15304087587844128139));
/// ```
pub fn file_checksum(t: ChecksumType, path: &Path) -> Result<Checksum, Error> {
    match t {
        ChecksumType::Crc64Nvme => {
            let mut digest = crc64fast_nvme::Digest::new();
            digest.write(std::fs::read(path)?.as_slice());
            Ok(Checksum::Crc64Nvme(digest.sum64()))
        }
    }
}

/// Gets an AWS S3 object's checksum.
///
/// ```rust
/// use aws_config::BehaviorVersion;
/// use s3same::Checksum;
/// use tokio::runtime::Runtime;
///
/// async fn get_checksum_async(uri: &s3uri::S3Uri) -> Checksum {
///     let config = aws_config::defaults(BehaviorVersion::latest())
///         .load()
///         .await;
///
///     let client = aws_sdk_s3::Client::new(&config);
///
///     s3same::object_checksum(&client, uri)
///         .await
///         .unwrap()
///         .unwrap()
/// }
///
/// let uri = s3uri::from_bucket("s3same")
///     .join("CRC64NVME")
///     .join("hello.txt");
///
/// let checksum = Runtime::new()
///     .unwrap()
///     .block_on(get_checksum_async(&uri));
///
/// assert_eq!(checksum, Checksum::Crc64Nvme(1905147139147649465));
/// ```
pub async fn object_checksum(
    client: &aws_sdk_s3::Client,
    uri: &s3uri::S3Uri,
) -> Result<Option<Checksum>, String> {
    let response = client
        .get_object_attributes()
        .bucket(&uri.bucket)
        .key(&uri.key)
        .object_attributes(aws_sdk_s3::types::ObjectAttributes::Checksum)
        .send()
        .await;

    match response {
        Ok(attributes) => match attributes.checksum {
            Some(checksum_attribute) => match checksum_attribute.checksum_crc64_nvme {
                Some(checksum_crc64_nvme) => match base64::to_u64(&checksum_crc64_nvme) {
                    Ok(checksum) => Ok(Some(Checksum::Crc64Nvme(checksum))),
                    Err(e) => Err(format!("{:#?}", e)),
                },
                None => Ok(None),
            },
            None => Ok(None),
        },
        Err(err) => match err.into_service_error() {
            GetObjectAttributesError::NoSuchKey(_) => Err(format!("{} does not exist", uri)),
            e => Err(format!("{:#?}", e)),
        },
    }
}

/// Checks if a local file and S3 object have the same content.
///
/// ```rust
/// use aws_config::BehaviorVersion;
/// use std::path::Path;
/// use s3same::Checksum;
/// use tokio::runtime::Runtime;
///
/// async fn are_same_async(local: &Path, remote: &s3uri::S3Uri) -> bool {
///     let config = aws_config::defaults(BehaviorVersion::latest())
///         .load()
///         .await;
///
///     let client = aws_sdk_s3::Client::new(&config);
///
///     s3same::are_same(&client, local, remote)
///         .await
///         .unwrap()
/// }
///
/// let local = Path::new("test_files")
///     .join("hello.txt");
///
/// let remote = s3uri::from_bucket("s3same")
///     .join("CRC64NVME")
///     .join("hello.txt");
///
/// let are_same = Runtime::new()
///     .unwrap()
///     .block_on(are_same_async(&local, &remote));
///
/// assert!(are_same);
/// ```
pub async fn are_same(
    client: &aws_sdk_s3::Client,
    local: &Path,
    remote: &s3uri::S3Uri,
) -> Result<bool, String> {
    match object_checksum(client, remote).await {
        Ok(opt_checksum) => match opt_checksum {
            Some(remote_checksum) => {
                match file_checksum(checksum_type(&remote_checksum), local) {
                    Ok(local_checksum) => {
                        let same = remote_checksum == local_checksum;
                        debug!("{local:?}={local_checksum:?}; {remote}={remote_checksum:?}; same={same}");
                        Ok(same)
                    }
                    Err(e) => {
                        error!("file_checksum failed ({e})");
                        Err(e.to_string())
                    }
                }
            }
            None => {
                debug!("{remote} has no checksum");
                Ok(false)
            }
        },
        Err(e) => {
            error!("object_checksum failed ({e})");
            Err(e)
        }
    }
}
