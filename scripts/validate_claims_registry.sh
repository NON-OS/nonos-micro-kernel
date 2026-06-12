#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
CLAIMS=${1:-"$ROOT/docs/engineering/claims.toml"}

if [ ! -f "$CLAIMS" ]; then
  echo "FAIL: claims registry missing: $CLAIMS" >&2
  exit 1
fi

required='CAP-MONO-001 SPAWN-FAIL-001 IPC-INTEGRITY-001 ZERO-STATE-001 NO-TELEMETRY-001 HW-BROKER-001 DMA-ISO-001 BUILD-DET-001 RELEASE-SAFE-001 CLAIM-DISCIPLINE-001'

fail=0
for id in $required; do
  if grep -F "id = \"$id\"" "$CLAIMS" >/dev/null 2>&1; then
    :
  else
    echo "FAIL: missing claim $id" >&2
    fail=1
  fi
done

if grep -nE 'status = "(proven|partially_proven|unproven|out_of_scope)"' "$CLAIMS" >/dev/null 2>&1; then
  :
else
  echo "FAIL: no valid claim status entries found" >&2
  fail=1
fi

bad_status=$(grep -nE 'status = "' "$CLAIMS" | grep -vE 'status = "(proven|partially_proven|unproven|out_of_scope)"' || true)
if [ -n "$bad_status" ]; then
  echo "$bad_status" >&2
  echo "FAIL: invalid claim status" >&2
  fail=1
fi

for field in id claim scope code_paths tests artifacts status falsifier last_verified_commit; do
  if grep -F "$field =" "$CLAIMS" >/dev/null 2>&1; then
    :
  else
    echo "FAIL: claims registry missing field '$field'" >&2
    fail=1
  fi
done

if [ "$fail" -eq 0 ]; then
  echo "claims registry: PASS"
else
  echo "claims registry: FAIL" >&2
fi
exit "$fail"
