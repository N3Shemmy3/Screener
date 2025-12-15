#!/bin/bash
set -e

BUILD_DIR="build"
PREFIX="$HOME/.local"
BINARY_NAME="screener"

if [ ! -d "$BUILD_DIR" ]; then
    meson setup "$BUILD_DIR" --prefix="$PREFIX" --buildtype=debug
fi

meson compile -C "$BUILD_DIR"
meson install -C "$BUILD_DIR"

"$PREFIX/bin/$BINARY_NAME"