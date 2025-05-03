# s3same: Compare AWS S3 objects in Rust

`s3same` is a Rust library for comparing AWS S3 objects.

## Installation

```bash
cargo add s3same
```

## Examples

### Check if a local file and S3 object have the same content

`s3same::are_same` checks if a local file and S3 object have the same content by comparing their checksums.

```rust
use aws_config::BehaviorVersion;
use std::path::Path;
use s3same::Checksum;
use tokio::runtime::Runtime;

async fn are_same_async(local: &Path, remote: &s3uri::S3Uri) -> bool {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    let client = aws_sdk_s3::Client::new(&config);

    s3same::are_same(&client, local, remote)
        .await
        .unwrap()
}

let local = Path::new("test_files")
    .join("hello.txt");

let remote = s3uri::from_bucket("s3same")
    .join("CRC64NVME")
    .join("hello.txt");

let are_same = Runtime::new()
    .unwrap()
    .block_on(are_same_async(&local, &remote));

assert!(are_same);
```

### Get a local file's checksum

`s3same::file_checksum` gets a local file's checksum.

```rust
let checksum = s3same::file_checksum(
    s3same::ChecksumType::Crc64Nvme,
    std::path::Path::new("LICENSE"),
).unwrap();

assert_eq!(
    checksum,
    s3same::Checksum::Crc64Nvme(15304087587844128139),
);
```

### Get an S3 object's checksum

`s3same::object_checksum` gets an S3 object's checksum.

```rust
use aws_config::BehaviorVersion;
use s3same::Checksum;
use tokio::runtime::Runtime;

async fn get_checksum_async(uri: &s3uri::S3Uri) -> Checksum {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;

    let client = aws_sdk_s3::Client::new(&config);

    s3same::object_checksum(&client, uri)
        .await
        .unwrap()
        .unwrap()
}

let uri = s3uri::from_bucket("s3same")
    .join("CRC64NVME")
    .join("hello.txt");

let checksum = Runtime::new()
    .unwrap()
    .block_on(get_checksum_async(&uri));

assert_eq!(
    checksum,
    Checksum::Crc64Nvme(1905147139147649465),
);
```

## Support

Please submit all your questions, feature requests and bug reports at [github.com/cariad/s3same/issues](https://github.com/cariad/s3same/issues). Thank you!

## License

The library is [open-source](https://github.com/cariad/s3same) and published under the [MIT License](https://github.com/cariad/s3same/blob/main/LICENSE).

## Author

Hello! 👋 I'm Cariad Eccleston. You can find me at [cariad.earth](https://www.cariad.earth), [github.com/cariad](https://github.com/cariad), [linkedin.com/in/cariad](https://linkedin.com/in/cariad) and [@cariad.earth](https://bsky.app/profile/cariad.earth) on Bluesky.
