#!/bin/sh
# -*- coding: utf-8 -*-

set -e

source ./curl.sh
export CC="musl-gcc -fPIE -pie -O2"

BASEDIR=`dirname $0`
DOWNLOAD_URL='https://www.mpfr.org/mpfr-current/mpfr-4.2.0.tar.xz'
FILENAME=`basename $DOWNLOAD_URL`
EXTRACTED_NAME=`basename ${FILENAME} .tar.xz`
WORK_DIR=`realpath $BASEDIR`
PREFIX="$WORK_DIR/../deps/mpfr"

curl $DOWNLOAD_URL -o $WORK_DIR/$FILENAME
tar xJvf $WORK_DIR/$FILENAME -C $WORK_DIR
rm -rf $PREFIX
cd $WORK_DIR/$EXTRACTED_NAME

./configure \
  --prefix=$PREFIX \
  --enable-shared=no \
  --enable-static=yes \
  --with-gmp=`dirname $PREFIX`/gmp
make -j`nproc`
make -j`nproc` check
make -j`nproc` install
cd -
rm -rf $WORK_DIR/$EXTRACTED_NAME $WORK_DIR/$FILENAME

cd $WORK_DIR/../deps
tar cJvf $WORK_DIR/../.circleci/`basename $PREFIX`.txz `basename $PREFIX`
cd -
