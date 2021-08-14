#!/bin/bash

set -v

./build.sh

dest="../ivanceras.github.io/performance-test-sauron"

rm -rf "$dest"

mkdir -p "$dest"

cp -r client/index.html client/pkg "$dest/"

## Remove the ignore file on the pkg directory
rm $dest/pkg/.gitignore
