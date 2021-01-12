#!/bin/sh
# -*- coding: utf-8 -*-

set -e

DEST=`dirname $0`/svc

mkdir -p $DEST

openssl ecparam \
  -name prime256v1 \
  -genkey \
  -out $DEST/root.key
# openssl genrsa -out $DEST/root.key 4096

openssl req \
  -new -x509 \
  -nodes \
  -key $DEST/root.key \
  -subj '/C=JP/ST=Tokyo/L=Tokyo/O=AAAA Midas Root Authority/OU=IT/CN=localhost' \
  -out $DEST/root-ca.pem

openssl ecparam \
  -name prime256v1 \
  -genkey \
  -out $DEST/devel.key
# openssl genrsa -out $DEST/devel.key 4096

openssl req \
  -new \
  -nodes \
  -key $DEST/devel.key \
  -subj '/C=JP/ST=Tokyo/L=Tokyo/O=AAAA Midas Signning Service/OU=IT/CN=localhost' \
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

dd if=/dev/urandom of=$DEST/devel.pfxpass bs=1024 count=1

openssl pkcs12 \
  -export \
  -nodes \
  -out $DEST/devel.pfx \
  -inkey $DEST/devel.key \
  -in $DEST/devel.crt \
  -certfile $DEST/root-ca.pem \
  -password file:$DEST/devel.pfxpass
