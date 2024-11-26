#!/bin/bash

# VERSION specified release that script will download to extract protobuf files.
VERSION="v1.8.24"

set -x

# Download source code.
wget https://github.com/XTLS/Xray-core/archive/refs/tags/$VERSION.tar.gz -O xray-core.tar.gz
trap 'rm xray-core.tar.gz' EXIT

# Extract zip archive to a temporary directory.
TEMP_DIR=$(mktemp -d)
tar --extract \
    --file xray-core.tar.gz \
    --strip-components=1 \
    --directory $TEMP_DIR

function sync_crate() {
    local temp_dir=$1

    rm -rf vendor/github.com/XTLS/Xray-core/

    rsync -avm \
        --include='*/' \
        --include-from=rsync.txt \
        --exclude='*' \
        $temp_dir/ \
        vendor/github.com/XTLS/Xray-core/
}

sync_crate $TEMP_DIR
