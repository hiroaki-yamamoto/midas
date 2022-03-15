#!/bin/sh
# -*- coding: utf-8 -*-

set -e

genBrowserKeys() {
  DEST=$1

  openssl ecparam \
    -name prime256v1 \
    -genkey \
    -out certs/browser.key

  openssl req \
    -new \
    -key $DEST/browser.key \
    -subj '/C=JP/ST=Tokyo/L=Tokyo/O=AAAA Midas Certificate Signing Request/OU=IT/CN=localhost' \
    -out $DEST/browser.csr

  openssl x509 \
    -req \
    -in $DEST/browser.csr \
    -CA $DEST/root-ca.pem \
    -CAkey $DEST/root.key \
    -CAcreateserial \
    -out $DEST/browser.crt \
    -days 730 \
    -sha256 \
    -extfile browser.cfg

  echo "Generated browser certificates"
}
