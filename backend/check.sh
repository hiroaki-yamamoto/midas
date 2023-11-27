#!/bin/sh
# -*- coding: utf-8 -*-

set -e
cargo check --target x86_64-unknown-linux-musl $@
