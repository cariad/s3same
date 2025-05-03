#!/bin/bash

set -euo pipefail

source .env

mkdir -p test_files

download () {
  destination="test_files/${1}"

  if [ -f "${destination}" ]; then
    echo "Skipped ${destination}; already downloaded."
    return
  fi

  aws s3 cp "s3://${S3SAME_TEST_BUCKET}/CRC64NVME/${1}" "${destination}"
}

download "${S3SAME_TEST_0_FILENAME}"
download "${S3SAME_TEST_1_FILENAME}"
download "${S3SAME_TEST_2_FILENAME}"
