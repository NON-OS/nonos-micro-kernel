#!/bin/sh
set -eu

report_dir=$1
hits="$report_dir/dangerous-source-hits.txt"

if grep -R -I -n -E 'allow[_-]?all.*cap|skip.*verif|unverified.*spawn|panic shell|debug root console|phone-home|telemetry endpoint|automatic update endpoint' \
  src userland nonos-bootloader/src Cargo.toml Makefile > "$hits" 2>/dev/null; then
  printf 'WARN: dangerous-string review hits recorded in %s\n' "$hits"
else
  printf '%s\n' "PASS: no dangerous release-control strings found in active source paths"
fi
