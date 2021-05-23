#!/bin/bash
set -euxo pipefail

for f in $(ls *.md); do
  echo "Processing $f"
  pandoc $f -o $(basename -s ".md" $f).pdf
done
