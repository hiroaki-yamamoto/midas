#!/bin/sh
# -*- coding: utf-8 -*-

set -e

source ./curl.sh

BASEDIR=`dirname $0`
VERSION='v4.35.2'
DOWNLOAD_URL="https://github.com/mikefarah/yq/releases/download/${VERSION}/yq_linux_amd64"
WORK_DIR=`realpath $BASEDIR`

rm -rf $WORK_DIR/../deps/yq
mkdir -p $WORK_DIR/../deps/yq/bin

curl $DOWNLOAD_URL -o $WORK_DIR/../deps/yq/bin/yq
curl https://raw.githubusercontent.com/mikefarah/yq/master/LICENSE \
  -o $WORK_DIR/../deps/yq/LICENSE.txt
chmod u+x $WORK_DIR/../deps/yq/bin/yq

cd $WORK_DIR/../deps
tar cJvf $WORK_DIR/../.github/yq.txz yq
cd -
