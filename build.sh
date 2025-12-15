#!/bin/bash

# 1. Kill the running instance (if any)
flatpak kill dev.n3shemmy3.Screener > /dev/null 2>&1 || true

# 2. Build and Install
# If this fails, the script exits immediately (exit 1)
flatpak-builder --force-clean --user --install build-dir dev.n3shemmy3.Screener.json || exit 1

# 3. Run the new version
flatpak run dev.n3shemmy3.Screener