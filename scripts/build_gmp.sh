#!/bin/sh
# -*- coding: utf-8 -*-

set -e

source ./curl.sh
export CC="musl-gcc -fPIE -pie -O2"

BASEDIR=`dirname $0`
DOWNLOAD_URL='https://gmplib.org/download/gmp/gmp-6.3.0.tar.xz'
FILENAME=`basename $DOWNLOAD_URL`
EXTRACTED_NAME=`basename ${FILENAME} .tar.xz`
WORK_DIR=`realpath $BASEDIR`
PREFIX="$WORK_DIR/../deps/gmp"

curl $DOWNLOAD_URL -o $WORK_DIR/$FILENAME
tar xJvf $WORK_DIR/$FILENAME -C $WORK_DIR
rm -rf $PREFIX
cd $WORK_DIR/$EXTRACTED_NAME

./configure \
  --prefix=$PREFIX \
  --enable-shared=no \
  --enable-static=yes
make -j`nproc`
make -j`nproc` check
make -j`nproc` install
cd -
rm -rf $WORK_DIR/$EXTRACTED_NAME $WORK_DIR/$FILENAME

cd $WORK_DIR/../deps
tar cJvf $WORK_DIR/../.github/`basename $PREFIX`.txz `basename $PREFIX`
cd -
