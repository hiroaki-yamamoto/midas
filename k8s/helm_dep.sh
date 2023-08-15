#!/bin/sh
# -*- coding: utf-8 -*-

set -e

WORKDIR=`dirname $0`

if [ ! -x `which helm` ]; then
  echo "Install Helm"
  exit 1
fi

if [ ! -x `which yq` ]; then
  echo "Install yq: https://github.com/mikefarah/yq"
  exit 1
fi

addRepo() {
  local name=$1
  local repo=$2
  echo "Adding '$2' as '$1'"
  helm repo add $1 $2
}

helmInstall() {
  echo "Installing $2 as $1"
  if [ -n "$3" ]; then
    echo "Helm Exra ARGS: ${@:3}"
  fi
  helm upgrade -i $@
}

NAMESPACE_PATH="${WORKDIR}/./namespace.yml"
MIDAS_NAMESPACE=`yq '.metadata.name' $NAMESPACE_PATH`

echo "Refreshing the release menu in the repo"
helm repo update

# addRepo 'nats' 'https://nats-io.github.io/k8s/helm/charts/'
# helmInstall broker nats/nats \
#   -n $MIDAS_NAMESPACE --create-namespace \
#   -f $WORKDIR/./broker/config.yml
# kubectl apply -f $WORKDIR/./broker/svc.yml -n midas

# addRepo 'mongodb' 'https://mongodb.github.io/helm-charts'
# helmInstall mongodb mongodb/mongodb \
#   -n $MIDAS_NAMESPACE --create-namespace
