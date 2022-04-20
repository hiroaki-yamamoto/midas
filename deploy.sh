#!/usr/bin/env sh
# -*- coding: utf-8 -*-

set -e

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

deploy() {
  skaffold run --build-concurrency=0
}

echo "Building Backend"
buildBackend
echo "Building Frontend"
buildFrontend
echo "Deploying"
deploy
echo "Done"
