#!/bin/sh
# -*- coding: utf-8 -*-

set -e

export CC="musl-gcc -fPIE -pie -O2"

BASEDIR=`dirname $0`
DOWNLOAD_URL='https://www.openssl.org/source/openssl-3.0.0.tar.gz'
FILENAME=`basename $DOWNLOAD_URL`
EXTRACTED_NAME=`basename ${FILENAME} .tar.gz`
WORK_DIR=`realpath $BASEDIR`

rm -rf $WORK_DIR/../deps/openssl
curl $DOWNLOAD_URL -o $WORK_DIR/$FILENAME
tar xzvf $WORK_DIR/$FILENAME -C $WORK_DIR
cd $WORK_DIR/$EXTRACTED_NAME;

./Configure no-shared \
  --prefix=$WORK_DIR/../deps/openssl \
  --openssldir=$WORK_DIR/../deps/openssl \
  linux-x86_64
make depend
make -j`nproc`
make -j`nproc` install
cp LICENSE.txt $WORK_DIR/../deps/openssl/LICENSE.txt
cd -
rm -rf $WORK_DIR/$EXTRACTED_NAME $WORK_DIR/$FILENAME

cd $WORK_DIR/../deps
tar cJvf $WORK_DIR/../.circleci/openssl.txz openssl
cd -