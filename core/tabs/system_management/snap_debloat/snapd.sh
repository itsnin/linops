#!/usr/bin/env bash
set -euo pipefail

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

if command_exists snap; then
    echo "==> removing snapd"
    apt-get remove --purge -y snapd
    echo "==> cleaning orphaned deps"
    apt-get autoremove -y
    echo "==> holding snapd so it cannot be reinstalled"
    apt-mark hold snapd
    echo "snapd removed and held"
else
    echo "snapd is not installed"
fi
