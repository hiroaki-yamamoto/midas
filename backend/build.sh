#!/bin/sh
# -*- coding: utf-8 -*-

set -e
cargo build --target x86_64-unknown-linux-musl $@
