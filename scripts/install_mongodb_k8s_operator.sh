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
MIDAS_NAMESPACE=`yq '.metadata.name' $WORKDIR/../k8s/namespace.yml`

source ./curl.sh

curl -o $WORKDIR/$ARCHIVE_NAME \
  "https://github.com/mongodb/mongodb-kubernetes-operator/archive/refs/tags/v$VERSION.tar.gz"

tar xzvf $WORKDIR/$ARCHIVE_NAME
rm $WORKDIR/$ARCHIVE_NAME
kubectl apply -f ${WORKDIR}/../k8s/namespace.yml
cd $WORKDIR/$NAME
kubectl apply -f config/crd/bases/mongodbcommunity.mongodb.com_mongodbcommunity.yaml
kubectl apply -k config/rbac/ --namespace $MIDAS_NAMESPACE
kubectl apply -f config/manager/manager.yaml --namespace $MIDAS_NAMESPACE
cd -
rm -rf $WORKDIR/$NAME

echo "Setup Done."
