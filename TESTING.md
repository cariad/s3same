# Testing

The library's unit tests require some files with known names and checksums to exist locally.

To set up these test files:

1. Update [.env](.env) to reference an S3 bucket with objects you have permission to read and their expected checksums.
1. Update [.download_test_files.sh](.download_test_files.sh) to ensure all of your test files will be downloaded.
1. Run:

    ```bash
    ./download_test_files.sh
    ```

After this initial setup, run the unit tests with:

```bash
cargo test
```
