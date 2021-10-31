#!/usr/bin/env sh
# -*- coding: utf-8 -*-

yarn install
exec node index.js ${@}
