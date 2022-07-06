#!/bin/sh
# -*- coding: utf-8 -*-

set -e

BASEDIR=`dirname $0`
WORKDIR=`realpath $BASEDIR`

openssl() {
  $WORKDIR/../../deps/openssl/bin/openssl $@
}
