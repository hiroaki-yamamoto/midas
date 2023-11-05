#!/bin/sh
# -*- coding: utf-8 -*-

DIR=`dirname $0`
DIR=`realpath $DIR`

files=`find $DIR/schema -type f -name "*.schema.yml" -and \
  -not -name "template.schema.yml"`

mkdir -p build

cat << EOF > build.ninja
rule yq
  command = $DIR/../deps/yq/bin/yq -M -o json \$in > \$out
  description = YAML -> JSON Schema

rule frontend
  command = npx quicktype --no-ignore-json-refs --src \$in -o \$out \
    -l ts --acronym-style original \
    -s schema
  description = JSON Schema -> Frontend

rule backend
  command = npx quicktype --no-ignore-json-refs --src \$in -o \$out -l rust \
    --density dense \
    --visibility public \
    --derive-{debug,clone,partial-eq} \
    -s schema
  description = JSON Schema -> Rust
EOF

for schema_in in $files; do
  JSON_SCHEMA=$DIR/build/`basename $schema_in .schema.yml`.json
  FRONTEND=$DIR/../frontend/src/rpc/$(basename $JSON_SCHEMA .json).ts
  BACKEND=$DIR/../backend/libs/rpc/src/$(basename $JSON_SCHEMA .json).rs

cat << EOF >> build.ninja
build $JSON_SCHEMA: yq $schema_in
build $FRONTEND: frontend $JSON_SCHEMA
build $BACKEND: backend $JSON_SCHEMA
EOF

  # echo "JSONS -> Frontend: $schema_in"
  # npx quicktype \
  #   --src "$JSON_SCHEMA" \
  #   -o "$DIR/../frontend/src/rpc/$(basename $JSON_SCHEMA .json).ts" \
  #   -l ts --acronym-style original \
  #   -s schema

  # echo "JSONS -> Backend: $schema_in"
  # npx quicktype \
  #   --src "$JSON_SCHEMA" \
  #   -o "$DIR/../backend/libs/rpc/src/$(basename $JSON_SCHEMA .json).rs" \
  #   -l rust --density dense --visibility public --derive-{debug,clone,partial-eq} \
  #   -s schema
done
