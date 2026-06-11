#!/bin/sh
set -eu

report_dir=$1
shift
artifacts="$report_dir/artifacts-to-scan.txt"
hits="$report_dir/artifact-dangerous-strings.txt"
dangerous='GDB on|panic shell|debug root console|unverified spawn|allow-all|skip verification|phone-home|telemetry endpoint|automatic update|serial command backdoor|QEMU-only bypass|embedded private key'

: > "$artifacts"
: > "$hits"

if [ "$#" -gt 0 ]; then
  if [ -f "$1" ]; then
    printf '%s\n' "$1" >> "$artifacts"
  else
    printf 'GAP: artifact not found at %s; source/config checks only\n' "$1"
  fi
else
  for candidate in \
    target/nonos-kernel.x86_64 \
    target/x86_64-nonos/release/nonos-kernel \
    target/kernel_signed.bin \
    target/kernel_attested.bin \
    ci-reports/build/nonos-kernel.x86_64
  do
    if [ -f "$candidate" ]; then
      printf '%s\n' "$candidate" >> "$artifacts"
    fi
  done
  find target/release-* -maxdepth 1 -type f \( -name '*.elf' -o -name '*.efi' -o -name '*.iso' -o -name '*.img' \) \
    >> "$artifacts" 2>/dev/null || true
fi

if [ ! -s "$artifacts" ]; then
  printf 'GAP: no release artifacts discovered; source/config checks only\n'
  exit 0
fi

printf '%s\n' "artifact scan list:"
sed 's/^/  /' "$artifacts"

if ! command -v strings >/dev/null 2>&1; then
  printf '%s\n' "GAP: strings(1) unavailable; artifact string scan skipped"
  exit 0
fi

while IFS= read -r artifact; do
  strings "$artifact" | grep -E "$dangerous" | sed "s|^|$artifact: |" >> "$hits" 2>/dev/null || true
done < "$artifacts"

if [ -s "$hits" ]; then
  printf '%s\n' "FAIL: release artifact contains dangerous debug/bypass strings"
  exit 1
fi

printf '%s\n' "PASS: release artifacts did not expose dangerous debug/bypass strings"
