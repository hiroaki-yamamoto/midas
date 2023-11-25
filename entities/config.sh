#!/bin/sh
# -*- coding: utf-8 -*-

WORKDIR=`dirname $0`

cp header.ninja build.ninja

for f in `find . -type f -name '*.yml'`; do
  name=`basename $f .yml`
cat << EOF >> build.ninja

build $WORKDIR/../backend/libs/rpc/src/$name.rs: backend $f
build $WORKDIR/../frontend/src/app/rpc/$name.ts: frontend $f
EOF
done
