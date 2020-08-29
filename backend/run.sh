#!/bin/sh
# -*- coding: utf-8 -*-

set -e

LIBRARY_PATH=/usr/local/rustup/toolchains/1.46.0-x86_64-unknown-linux-musl/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained:${LIBRARY_PATH}
CPATH=/usr/lib/gcc/x86_64-alpine-linux-musl/9.3.0/include:${CPATH}
export LIBRARY_PATH CPATH

cargo build -p ${SERVICE}
exec ./target/debug/${SERVICE} $@
