#!/usr/bin/env sh
# -*- coding: utf-8 -*-

yarn install
exec yarn run start -- --host 0.0.0.0 --port 50000
