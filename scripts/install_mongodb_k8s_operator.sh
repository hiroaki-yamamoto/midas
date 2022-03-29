#!/bin/sh
# -*- coding: utf-8 -*-

set -e

if [ ! -x `which yq` ]; then
  echo "Install yq: https://github.com/mikefarah/yq"
  exit 1
fi

BASEDIR=`dirname $0`
WORKDIR=`realpath $BASEDIR`
VERSION='0.7.3'
NAME="mongodb-kubernetes-operator-$VERSION"
ARCHIVE_NAME="$NAME.tar.gz"

source ./curl.sh

curl -o $WORKDIR/$ARCHIVE_NAME \
  "https://github.com/mongodb/mongodb-kubernetes-operator/archive/refs/tags/v$VERSION.tar.gz"

tar xzvf $WORKDIR/$ARCHIVE_NAME
rm $WORKDIR/$ARCHIVE_NAME
kubectl apply -f mongodb_namespace.yml \
  --dry-run=client -o yaml
cd $WORKDIR/$NAME
kubectl apply -f config/crd/bases/mongodbcommunity.mongodb.com_mongodbcommunity.yaml \
  --dry-run=client -o yaml
kubectl apply -k config/rbac/ --namespace mongodb \
  --dry-run=client -o yaml
kubectl apply -f config/manager/manager.yaml --namespace mongodb \
  --dry-run=client -o yaml
