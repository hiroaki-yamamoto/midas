#!/bin/env sh
# -*- coding: utf-8 -*-

DIR=$(realpath $(dirname "$0"))
SCHEMA_DIR=$DIR/schema
NAME=${1//_/ }
NAME=${NAME^}

if [ -z "$1" ]; then
  echo "Usage: $0 <schema-name>"
  exit 1
fi

$DIR/../deps/yq/bin/yq \
  ".title = \"$NAME schema\" | .\$id = \"/$1\" " \
  $SCHEMA_DIR/template.schema.yml \
> $SCHEMA_DIR/$1.schema.yml
