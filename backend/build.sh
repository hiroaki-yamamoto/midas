#!/bin/sh
# -*- coding: utf-8 -*-

set -e

export \
  PKG_CONFIG_ALLOW_CROSS=1 \
  OPENSSL_STATIC=true \
  OPENSSL_DIR=${HOME}/opt/openssl-musl/musl \
  CARGO_INCREMENTAL=false

cargo build --target x86_64-unknown-linux-musl
