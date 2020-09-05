#!/bin/sh
# -*- coding: utf-8 -*-

set -e

exec ./target/x86_64-unknown-linux-musl/debug/${SERVICE} $@
