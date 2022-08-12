#!/usr/bin/env sh
# -*- coding: utf-8 -*-

set -e

buildFrontendProto() {
  cd proto
  ./build.sh
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

deploy() {
  skaffold run --build-concurrency=0
}

# echo "Generating proto code to typescript"
# buildFrontendProto
echo "Building Backend"
buildBackend
echo "Building Frontend"
buildFrontend
echo "Deploying"
deploy
echo "Done"
