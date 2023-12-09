#!/usr/bin/env sh
# -*- coding: utf-8 -*-

set -e

buildFrontendEntities() {
  cd entities
  ./config.sh
  ninja
  cd -
}

buildBackend() {
  cd backend
  ./build.sh
  cd -
}

buildFrontend() {
  cd frontend
  pnpm run build
  cd -
}

buildOldFrontend() {
  cd frontend.old
  pnpm run build
  cd -
}

deploy() {
  skaffold run --build-concurrency=0
}

echo "Generating Entities code to typescript"
buildFrontendEntities
echo "Building Backend"
buildBackend
echo "Building Frontend"
buildFrontend
echo "Building Old Frontend"
buildOldFrontend
echo "Deploying"
deploy
echo "Done"
