#!/bin/sh
# -*- coding: utf-8 -*-

set -e

genSvcKeys() {
  DEST=$1

  # openssl ecparam \
  #   -name prime256v1 \
  #   -genkey \
  #   -out $DEST/svc.key
  openssl genrsa -out $DEST/svc.key 4096

  openssl req \
    -new \
    -nodes \
    -key $DEST/svc.key \
    -subj '/C=JP/ST=Tokyo/L=Tokyo/O=AAAA Midas Signning Service/OU=IT/CN=localhost' \
    -out $DEST/svc.csr

  openssl x509 \
    -req \
    -in $DEST/svc.csr \
    -CA $DEST/root-ca.pem \
    -CAkey $DEST/root.key \
    -CAcreateserial \
    -out $DEST/svc.crt \
    -days 730 \
    -sha256 \
    -extfile svc.cfg

  echo "Generated service certificates"
}
