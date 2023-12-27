#!/bin/sh
# -*- coding: utf-8 -*-

set -e

source ./curl.sh
export CC="musl-gcc -fPIE -pie -O2"

BASEDIR=`dirname $0`
VERSION='1.2.3'
DOWNLOAD_URL="https://github.com/hiroaki-yamamoto/egen/releases/download/v${VERSION}/egen"
FILENAME=`basename $DOWNLOAD_URL`
EXTRACTED_NAME=`basename ${FILENAME} .tar.gz`
WORK_DIR=`realpath $BASEDIR`

rm -rf $WORK_DIR/../deps/egen
curl $DOWNLOAD_URL -o $WORK_DIR/$FILENAME
chmod u+x $WORK_DIR/$FILENAME
mkdir -p $WORK_DIR/../deps/egen/bin
mv $WORK_DIR/$FILENAME $WORK_DIR/../deps/egen/bin
