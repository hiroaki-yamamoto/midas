#!/usr/bin/env sh
# -*- coding: utf-8 -*-

CERT_DIR=`dirname $0`/certs/proxy

kubectl delete -n midas secret proxy

kubectl create secret tls proxy \
  -n midas \
  --key $CERT_DIR/tls.key \
  --cert $CERT_DIR/tls.crt
