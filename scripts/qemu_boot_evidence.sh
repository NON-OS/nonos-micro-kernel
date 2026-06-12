#!/bin/sh
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT"
out=${NONOS_BOOT_EVIDENCE_OUT:-target/boot-evidence}
timeout_secs=${NONOS_BOOT_EVIDENCE_TIMEOUT:-180}
log=${NONOS_BOOT_EVIDENCE_LOG:-$out/qemu-serial.log}
report=$out/report.json
boot_cmd=${NONOS_BOOT_EVIDENCE_CMD:-}
mkdir -p "$out" "$(dirname "$log")"
rm -f "$log"
if [ -z "$boot_cmd" ]; then
  boot_cmd="make QEMU_NET_MODE=off QEMU_SERIAL_LOG=$log nonos-mk-run-serial-log"
fi
set +e
if command -v timeout >/dev/null 2>&1; then
  timeout "$timeout_secs" sh -c "$boot_cmd"
elif command -v gtimeout >/dev/null 2>&1; then
  gtimeout "$timeout_secs" sh -c "$boot_cmd"
else
  perl -e 'alarm shift; exec @ARGV or die "exec failed: $!"' "$timeout_secs" sh -c "$boot_cmd"
fi
rc=$?
set -e
if [ "$rc" -ne 0 ] && [ "$rc" -ne 124 ] && [ "$rc" -ne 142 ]; then
  printf '{"status":"gap","reason":"boot command failed","exit":%s,"log":"%s"}\n' "$rc" "$log" > "$report"
  printf '%s\n' "qemu boot evidence: GAP boot command failed"
  exit 2
fi

if [ ! -s "$log" ] && { [ "$rc" -eq 124 ] || [ "$rc" -eq 142 ]; }; then
  printf '{"status":"gap","reason":"timeout before serial log","exit":%s,"log":"%s"}\n' "$rc" "$log" > "$report"
  printf '%s\n' "qemu boot evidence: GAP timeout before serial log"
  exit 2
fi
if [ ! -s "$log" ]; then
  printf '{"status":"gap","reason":"serial log missing or empty","log":"%s"}\n' "$log" > "$report"
  printf '%s\n' "qemu boot evidence: GAP serial log missing or empty"
  exit 2
fi

if grep -a -E '\[FATAL\]|\[PANIC\]' "$log" >/dev/null 2>&1; then
  printf '{"status":"fail","reason":"fatal marker","log":"%s"}\n' "$log" > "$report"
  printf '%s\n' "qemu boot evidence: FAIL fatal marker"
  exit 1
fi

if grep -a -F "[INIT] Capsules spawned" "$log" >/dev/null 2>&1; then
  printf '{"status":"pass","marker":"[INIT] Capsules spawned","log":"%s"}\n' "$log" > "$report"
  printf '%s\n' "qemu boot evidence: PASS capsules spawned"
  exit 0
fi

printf '{"status":"gap","reason":"ready marker missing","log":"%s"}\n' "$log" > "$report"
printf '%s\n' "qemu boot evidence: GAP ready marker missing"
exit 2
