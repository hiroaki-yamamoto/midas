#!/bin/sh
# -*- coding: utf-8 -*-

set -e

source ./openssl.sh

HERE=`realpath $(dirname $0)`
CERTS=$HERE/certs
ROOT_DIR=$CERTS/root
SVC_DIR=$CERTS/svc
PROXY_DIR=$CERTS/proxy
mkdir -p $CERTS $ROOT_DIR $SVC_DIR $PROXY_DIR

if [ -f  $ROOT_DIR/root.key -a -f $ROOT_DIR/root-ca.pem ];then
  echo "Skipped root key generation. Files exist."
else
  source $HERE/gen-root.sh
  genRootKeys $ROOT_DIR
fi

if [ -f  $SVC_DIR/tls.key -a -f $SVC_DIR/tls.csr -a -f $SVC_DIR/tls.crt ];then
  echo "Skipped service certificate generation. Files exist."
else
  source $HERE/gen-svc.sh
  genSvcKeys $ROOT_DIR $SVC_DIR
fi

if [ -f  $PROXY_DIR/tls.key -a -f $PROXY_DIR/tls.csr -a -f $PROXY_DIR/tls.crt ];then
  echo "Skipped proxy certificate generation. Files exist."
else
  source $HERE/gen-proxy.sh
  genProxyKeys $ROOT_DIR $PROXY_DIR
fi
