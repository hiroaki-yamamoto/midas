#!/bin/sh
# -*- coding: utf-8 -*-

set -e

DEPS=`realpath ../deps`

export \
  PKG_CONFIG_ALLOW_CROSS=1 \
  OPENSSL_STATIC=true \
  OPENSSL_DIR=$DEPS/openssl \
  OPENSSL_LIB_DIR=$DEPS/openssl/lib64 \
  CARGO_INCREMENTAL=false

cargo build --target x86_64-unknown-linux-musl $@
