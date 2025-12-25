#!/bin/sh
set -eu # error download
export DIST="$1"
export SOURCE_ROOT="$2"

cd "$SOURCE_ROOT"
mkdir "$DIST"/.cargo
cargo vendor | sed 's/^directory = ".*"/directory = "vendor"/g' > $DIST/.cargo/config.toml
# Move vendor into dist tarball directory
mv vendor "$DIST"