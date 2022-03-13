#!/bin/sh
# -*- coding: utf-8 -*-

set -e

HERE=`dirname $0`
CERTS=$HERE/certs
mkdir -p $CERTS

if [ -f  $CERTS/root.key -a -f $CERTS/root-ca.pem ];then
  echo "Skipped root key generation. Files exist."
else
  source $HERE/gen-root.sh
  genRootKeys $CERTS
fi

if [ -f  $CERTS/svc.key -a -f $CERTS/svc.csr -a -f $CERTS/svc.crt ];then
  echo "Skipped service certificate generation. Files exist."
else
  source $HERE/gen-svc.sh
  genSvcKeys $CERTS
fi

if [ -f  $CERTS/browser.key -a -f $CERTS/browser.csr -a -f $CERTS/browser.crt ];then
  echo "Skipped browser certificate generation. Files exist."
else
  source $HERE/gen-browser.sh
  genBrowserKeys $CERTS
fi
