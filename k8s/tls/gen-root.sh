#!/bin/sh
# -*- coding: utf-8 -*-

set -e

genRootKeys() {
  DEST=$1

  openssl ecparam \
    -name prime256v1 \
    -genkey \
    -out $DEST/root.key
  # openssl genrsa -des3 -out $DEST/root.key 2048

  openssl req \
    -new -x509 \
    -nodes \
    -key $DEST/root.key \
    -subj '/C=JP/ST=Tokyo/L=Tokyo/O=AAAA Midas Root Authority/OU=IT/CN=localhost' \
    -days 3650 \
    -out $DEST/root-ca.pem

  echo "Generated root keys. Add root-ca.pem to your browser when accessing the app."
}
