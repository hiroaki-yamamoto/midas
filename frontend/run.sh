#!/usr/bin/env sh
# -*- coding: utf-8 -*-

pnpm i
exec pnpm run start -- \
  --host 0.0.0.0 \
  --port 50000 \
  --disable-host-check \
  --hmr true
