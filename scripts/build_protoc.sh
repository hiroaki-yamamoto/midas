#!/bin/sh
# -*- coding: utf-8 -*-

set -e

source ./curl.sh

VERSION='21.5'

BASEDIR=`dirname $0`
FILENAME="protoc-${VERSION}-linux-x86_64.zip"
REPO="https://github.com/protocolbuffers/protobuf"
DOWNLOAD_URL="${REPO}/releases/download/v$VERSION/${FILENAME}"
EXTRACTED_NAME="protoc"
WORK_DIR=`realpath "${BASEDIR}/../deps/${EXTRACTED_NAME}"`

rm -rf $WORK_DIR
mkdir -p $WORK_DIR
cd $WORK_DIR
echo "$DOWNLOAD_URL"
curl $DOWNLOAD_URL -o $FILENAME
bsdtar xvf $FILENAME
rm $FILENAME
cd -

echo "Done."
