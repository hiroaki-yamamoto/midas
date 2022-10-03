#!/usr/bin/sh
# -*- coding: utf-8 -*-

set -e

FRONT_OUT=`dirname $0`/../frontend/src/app/rpc
PROTOS=`find . -type f -name '*.proto'`

frontend() {
  mkdir -p $FRONT_OUT
  PATH=$PATH:`realpath $(dirname $0)/../deps/protoc/bin` ../deps/protoc/bin/protoc \
    --grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:$FRONT_OUT \
    --js_out=import_style=commonjs:$FRONT_OUT \
    -I . $PROTOS
  echo "Done (Frontend)"
}

clean() {
  rm -rf \
    `find $FRONT_OUT -type f -name *_pb.js` \
    `find $FRONT_OUT -type f -name *_pb.d.ts` \
    `find $FRONT_OUT -type f -name *_pb.ts`
}

case $1 in
  clean)
    clean
  ;;
  *)
    frontend
    ;;
esac
