#!/bin/sh
# -*- coding: utf-8 -*-

set -e

source ./curl.sh

BASEDIR=`dirname $0`
protoc() {
  local VERSION='3.20.1'
  local FILENAME="protoc-${VERSION}-linux-x86_64.zip"
  local REPO="https://github.com/protocolbuffers/protobuf"
  local DOWNLOAD_URL="${REPO}/releases/download/v$VERSION/${FILENAME}"
  local EXTRACTED_NAME="protoc"
  local WORK_DIR=`realpath "${BASEDIR}/../deps/${EXTRACTED_NAME}"`

  rm -rf $WORK_DIR
  mkdir -p $WORK_DIR
  cd $WORK_DIR
  echo "$DOWNLOAD_URL"
  curl $DOWNLOAD_URL -o $FILENAME
  bsdtar xvf $FILENAME
  rm $FILENAME
  cd -

  echo "Done."
}

grpc() {
  local VERSION='1.4.0'
  local FILENAME="protoc-gen-grpc-web-$VERSION-linux-x86_64"
  local OUT_FILE='protoc-gen-grpc-web'
  local REPO=https://github.com/grpc/grpc-web
  local URL="$REPO/releases/download/$VERSION/$FILENAME"

  local WORK_DIR=`realpath "${BASEDIR}/../deps/protoc"`
  cd $WORK_DIR
  curl $URL -o bin/$OUT_FILE
  chmod u+x bin/$OUT_FILE
  cd -
  echo "Done."
}

protoc
grpc
