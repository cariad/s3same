# s3same: Compare AWS S3 objects in Rust

## Installation

```bash
cargo add s3same
```

## Examples

### Check if a local file is of an expected size

`s3uri::from_uri` parses a string URI and returns a `s3uri::S3Uri` struct with the bucket name and key set.

```rust
use s3same::LocalPath;

let result = std::path::Path::new("LICENSE")
    .is_size(1073)
    .unwrap();

assert!(result);
```

## Author

Hello! 👋 I'm Cariad Eccleston. You can find me at [cariad.earth](https://www.cariad.earth), [github.com/cariad](https://github.com/cariad), [linkedin.com/in/cariad](https://linkedin.com/in/cariad) and [@cariad.earth](https://bsky.app/profile/cariad.earth) on Bluesky.
