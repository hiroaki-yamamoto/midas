#!/bin/sh
# -*- coding: utf-8 -*-

curl() {
  `which curl` -fLC - --retry 3 --retry-delay 3 $@
}
