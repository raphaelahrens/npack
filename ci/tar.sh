#!/usr/bin/env bash

set -ex

VERSION=$1
TARGET=$2

td=$(mktemp -d)
out_dir=$(pwd)
name="${PROJECT_NAME}-${VERSION}-${TARGET}"

cp target/release/pack "$td/"
cp README.md "$td/"

pushd $td
tar czf "$out_dir/$name.tar.gz" *
popd
rm -r $td