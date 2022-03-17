#!/bin/sh
# -*- coding: utf-8 -*-

set -e

genProxyKeys() {
  ROOT=$1
  DEST=$2

  openssl ecparam \
    -name prime256v1 \
    -genkey \
    -out $DEST/tls.key

  openssl req \
    -new \
    -key $DEST/tls.key \
    -subj '/C=JP/ST=Tokyo/L=Tokyo/O=AAAA Midas Certificate Signing Request/OU=IT/CN=localhost' \
    -out $DEST/tls.csr

  openssl x509 \
    -req \
    -in $DEST/tls.csr \
    -CA $ROOT/root-ca.pem \
    -CAkey $ROOT/root.key \
    -CAcreateserial \
    -out $DEST/tls.crt \
    -days 730 \
    -sha256 \
    -extfile proxy.cfg

  echo "Generated browser certificates"
}
