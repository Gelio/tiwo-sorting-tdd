#!/bin/bash
set -euxo pipefail

TMP_REPORT=/tmp/tiwo-report
REPORT_NAME="TiWO-InsertionSort-GrzegorzRozdzialik"

ORIG_PWD=$PWD
echo "Regenerate reports"
cd reports
./generate.sh
cd $ORIG_PWD

rm -rf $TMP_REPORT
mkdir -p "$TMP_REPORT/$REPORT_NAME"
cp reports/assignment-2-full-implementation.pdf "$TMP_REPORT/$REPORT_NAME/$REPORT_NAME.pdf"
cp -r src README.md Cargo.toml "$TMP_REPORT/$REPORT_NAME"
cd $TMP_REPORT

zip -r "$ORIG_PWD/$REPORT_NAME.zip" $REPORT_NAME
cd $ORIG_PWD
rm -rf $TMP_REPORT
