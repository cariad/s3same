use s3same::{Checksum, ChecksumType};
use std::collections::HashMap;
use std::env;
use std::path::Path;

fn make_bucket() -> s3uri::S3Uri {
    let bucket = env::var_os("S3SAME_TEST_BUCKET").unwrap();
    s3uri::from_bucket(bucket.to_str().unwrap())
}

async fn make_client() -> aws_sdk_s3::Client {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;

    aws_sdk_s3::Client::new(&config)
}

fn make_crc64nvme_test_cases() -> HashMap<String, u64> {
    dotenv::dotenv().ok();

    let expr = r"S3SAME_TEST_(?<group>.+)_(?<key>.+)";
    let expectations = envish::group_by(expr).unwrap();
    let mut test_cases = HashMap::new();

    for expectation in expectations {
        test_cases.insert(
            expectation.1["FILENAME"].to_owned().into_string().unwrap(),
            expectation.1["CRC64NVME"]
                .to_owned()
                .into_string()
                .unwrap()
                .parse()
                .unwrap(),
        );
    }

    assert!(test_cases.len() > 0, "no test cases found");

    test_cases
}

#[tokio::test]
async fn test_are_same() {
    let client = make_client().await;
    let crc64nvme_prefix = make_bucket().join("CRC64NVME");
    let test_files = Path::new("test_files");

    for case in make_crc64nvme_test_cases() {
        let local = test_files.join(&case.0);
        let remote = crc64nvme_prefix.join(&case.0);
        let are_same = s3same::are_same(&client, &local, &remote).await.unwrap();
        assert!(are_same);
    }
}

#[tokio::test]
async fn test_are_same_false() {
    let cases = make_crc64nvme_test_cases();
    let mut names = cases.keys();

    let name_0 = names.next().unwrap();
    let local = Path::new("test_files").join(&name_0);

    let name_1 = names.next().unwrap();
    let remote = make_bucket().join("CRC64NVME").join(&name_1);

    let client = make_client().await;

    let are_same = s3same::are_same(&client, &local, &remote).await.unwrap();
    assert!(!are_same);
}

#[test]
fn test_file_checksum() {
    let test_files = Path::new("test_files");

    for test_case in make_crc64nvme_test_cases() {
        let path = test_files.join(test_case.0);
        let checksum = s3same::file_checksum(ChecksumType::Crc64Nvme, &path).unwrap();
        let expect = Checksum::Crc64Nvme(test_case.1);
        assert_eq!(checksum, expect);
    }
}

#[tokio::test]
async fn test_object_checksum() {
    let client = make_client().await;
    let crc64nvme_prefix = make_bucket().join("CRC64NVME");

    for case in make_crc64nvme_test_cases() {
        let uri = crc64nvme_prefix.join(&case.0);

        let checksum = s3same::object_checksum(&client, &uri)
            .await
            .unwrap()
            .unwrap();

        let expect = Checksum::Crc64Nvme(case.1);
        assert_eq!(checksum, expect);
    }
}
