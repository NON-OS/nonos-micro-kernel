#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT"

report_dir=${NONOS_AUDIT_OUT:-target/release-audit}
mkdir -p "$report_dir"
report="$report_dir/release-profile-audit.txt"
: > "$report"

fail=0
run_check() {
  name=$1
  shift
  out="$report_dir/$name.out"
  if "$@" > "$out" 2>&1; then
    cat "$out" | tee -a "$report"
  else
    fail=1
    cat "$out" | tee -a "$report"
  fi
}

{
  printf '%s\n' "NØNOS release-profile audit"
  printf 'commit: %s\n' "$(git rev-parse HEAD 2>/dev/null || printf unknown)"
  printf 'source_date_epoch: %s\n' "${SOURCE_DATE_EPOCH:-unset}"
  printf '\n'
} | tee -a "$report"

run_check config scripts/release_audit/config_checks.sh
run_check key_material scripts/release_audit/key_material_checks.sh
run_check source_strings scripts/release_audit/source_string_checks.sh "$report_dir"
run_check artifacts scripts/release_audit/artifact_checks.sh "$report_dir" "$@"

if [ "$fail" -eq 0 ]; then
  printf '\n%s\n' "release-profile audit: PASS" | tee -a "$report"
else
  printf '\n%s\n' "release-profile audit: FAIL" | tee -a "$report"
fi

exit "$fail"
