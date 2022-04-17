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
  skaffold build -q | skaffold deploy --build-artifacts -
}

echo "Building Backend"
buildBackend
echo "Building Frontend"
buildFrontend
echo "Deploying"
deploy
echo "Done"
