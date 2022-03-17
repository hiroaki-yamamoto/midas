#!/bin/sh
# -*- coding: utf-8 -*-

set -e

genSvcKeys() {
  ROOT=$1
  DEST=$2

  openssl genrsa -out $DEST/tls.key 4096

  openssl req \
    -new \
    -nodes \
    -key $DEST/tls.key \
    -subj '/C=JP/ST=Tokyo/L=Tokyo/O=AAAA Midas Signning Service/OU=IT/CN=localhost' \
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
    -extfile svc.cfg

  echo "Generated service certificates"
}
