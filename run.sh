#!/bin/sh
# -*- coding: utf-8 -*-

set -e

go mod download -x
go build -o /bin/app ./services/${SERVICE}/main.go
exec app $@
