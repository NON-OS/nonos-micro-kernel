#!/bin/sh
set -eu

fail=0

if grep -F 'nonos-production = ["nonos-zk-enforce"]' Cargo.toml >/dev/null 2>&1; then
  printf '%s\n' "PASS: nonos-production implies nonos-zk-enforce"
else
  fail=1
  printf '%s\n' "FAIL: nonos-production must imply nonos-zk-enforce"
fi

if grep -F 'feature = "nonos-production", feature = "nonos-dev-unverified-capsules"' src/lib.rs >/dev/null 2>&1; then
  printf '%s\n' "PASS: production/dev-unverified feature mutex is compile-time enforced"
else
  fail=1
  printf '%s\n' "FAIL: missing compile_error! mutex for production and dev-unverified capsules"
fi

if sed -n '/^default = /p' Cargo.toml | grep -F 'nonos-dev-unverified-capsules' >/dev/null 2>&1; then
  fail=1
  printf '%s\n' "FAIL: default features include nonos-dev-unverified-capsules"
else
  printf '%s\n' "PASS: default features do not enable the unverified spawn escape hatch"
fi

exit "$fail"
