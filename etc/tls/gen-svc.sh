#!/bin/sh
# -*- coding: utf-8 -*-

set -e

DEST=`dirname $0`/svc

mkdir -p $DEST
openssl ecparam \
  -name brainpoolP512t1 \
  -genkey \
  -out $DEST/root.key
openssl req \
  -new -x509 \
  -nodes \
  -key $DEST/root.key \
  -subj '/C=JP/ST=Tokyo/L=Tokyo/O=AAAA Midas Root Authority/OU=IT/CN=localhost' \
  -out $DEST/root-ca.pem

openssl ecparam \
  -name brainpoolP512t1 \
  -genkey \
  -out $DEST/devel.key

openssl req \
  -new \
  -key $DEST/devel.key \
  -subj '/C=JP/ST=Tokyo/L=Tokyo/O=AAAA Midas Service Signing Request/OU=IT/CN=localhost' \
  -out $DEST/devel.csr

openssl x509 \
  -req \
  -in $DEST/devel.csr \
  -CA $DEST/root-ca.pem \
  -CAkey $DEST/root.key \
  -CAcreateserial \
  -out $DEST/devel.crt \
  -days 730 \
  -sha256 \
  -extfile svc.cfg
