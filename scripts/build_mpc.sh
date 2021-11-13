#!/bin/sh
# -*- coding: utf-8 -*-

set -e

export CC="musl-gcc -fPIE -pie -O2"

BASEDIR=`dirname $0`
DOWNLOAD_URL='http://www.multiprecision.org/downloads/mpc-1.2.0.tar.gz'
FILENAME=`basename $DOWNLOAD_URL`
EXTRACTED_NAME=`basename ${FILENAME} .tar.gz`
WORK_DIR=`realpath $BASEDIR`
PREFIX="$WORK_DIR/../deps/mpc"

curl $DOWNLOAD_URL -o $WORK_DIR/$FILENAME
tar xzvf $WORK_DIR/$FILENAME -C $WORK_DIR
rm -rf $PREFIX
cd $WORK_DIR/$EXTRACTED_NAME

./configure \
  --prefix=$PREFIX \
  --enable-shared=no \
  --enable-static=yes \
  --with-gmp=`dirname $PREFIX`/gmp \
  --with-mpfr=`dirname $PREFIX`/mpfr
make -j`nproc`
make -j`nproc` check
make -j`nproc` install
cd -
rm -rf $WORK_DIR/$EXTRACTED_NAME $WORK_DIR/$FILENAME

cd $WORK_DIR/../deps
tar cJvf $WORK_DIR/../.circleci/`basename $PREFIX`.txz `basename $PREFIX`
cd -
