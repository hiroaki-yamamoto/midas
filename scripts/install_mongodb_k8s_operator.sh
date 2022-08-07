#!/bin/sh
# -*- coding: utf-8 -*-

set -e

if [ ! -x `which yq` ]; then
  echo "Install yq: https://github.com/mikefarah/yq"
  exit 1
fi

BASEDIR=`dirname $0`
WORKDIR=`realpath $BASEDIR`
VERSION='0.7.5'
NAME="mongodb-kubernetes-operator-$VERSION"
ARCHIVE_NAME="$NAME.tar.gz"
NAMESPACE_PATH="${WORKDIR}/../k8s/database/namespace.yml"
MIDAS_NAMESPACE=`yq '.metadata.name' $NAMESPACE_PATH`

source ./curl.sh

curl -o $WORKDIR/$ARCHIVE_NAME \
  "https://github.com/mongodb/mongodb-kubernetes-operator/archive/refs/tags/v$VERSION.tar.gz"

tar xzvf $WORKDIR/$ARCHIVE_NAME
rm $WORKDIR/$ARCHIVE_NAME
kubectl apply -f $NAMESPACE_PATH
cd $WORKDIR/$NAME
kubectl apply -f config/crd/bases/mongodbcommunity.mongodb.com_mongodbcommunity.yaml
kubectl apply -k config/rbac/ --namespace $MIDAS_NAMESPACE
kubectl apply -f config/manager/manager.yaml --namespace $MIDAS_NAMESPACE
cd -
rm -rf $WORKDIR/$NAME

echo "Setup Done."
