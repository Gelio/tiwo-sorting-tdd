#!/bin/bash
set -euxo pipefail

for f in "*.md"; do
  echo "Processing $f"
  pandoc $f -o $(basename -s ".md" $f).pdf
done
