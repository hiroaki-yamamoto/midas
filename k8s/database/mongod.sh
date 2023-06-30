#!/bin/sh
# -*- coding: utf-8 -*-

SCRIPT_DIR=`realpath $(dirname $0)`

docker stop mongodb
docker rm mongodb

docker run -d \
  --name mongodb \
  --restart unless-stopped \
  --env MONGO_INITDB_ROOT_USERNAME='admin' \
  --env MONGO_INITDB_ROOT_PASSWORD='admin' \
  --env MONGO_INITDB_DATABASE='midas' \
  --network minikube \
  -h mongodb \
  -v $SCRIPT_DIR/../../db:/data/db \
  -v $SCRIPT_DIR/scripts:/docker-entrypoint-initdb.d:ro \
  -- mongo:latest
