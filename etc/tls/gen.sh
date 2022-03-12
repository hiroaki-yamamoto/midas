#!/bin/sh
# -*- coding: utf-8 -*-

set -e

DEST=`dirname $0`/browser

mkdir -p $DEST

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
  -out $DEST/root-ca.pem

echo "Generated root-ca.pem. Add it to your browser when accessing the app."

openssl ecparam \
  -name prime256v1 \
  -genkey \
  -out $DEST/localhost.key
# openssl genrsa -des3 -out $DEST/localhost.key 2048

openssl req \
  -new \
  -key $DEST/localhost.key \
  -subj '/C=JP/ST=Tokyo/L=Tokyo/O=AAAA Midas Certificate Signing Request/OU=IT/CN=localhost' \
  -out $DEST/localhost.csr

openssl x509 \
  -req \
  -in $DEST/localhost.csr \
  -CA $DEST/root-ca.pem \
  -CAkey $DEST/root.key \
  -CAcreateserial \
  -out $DEST/localhost.crt \
  -days 730 \
  -sha256 \
  -extfile localhost.cfg
