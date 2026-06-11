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

out=${NONOS_NO_TELEMETRY_OUT:-target/no-telemetry}
timeout_secs=${NONOS_NO_TELEMETRY_TIMEOUT:-120}
pcap=${NONOS_NO_TELEMETRY_PCAP:-$out/qemu-net.pcap}
report=$out/report.json
boot_cmd=${NONOS_NO_TELEMETRY_BOOT_CMD:-}

mkdir -p "$out" "$(dirname "$pcap")"
rm -f "$pcap"

if [ -z "$boot_cmd" ]; then
  boot_cmd="make QEMU_NET_CAPTURE=$pcap nonos-mk-run-serial-net"
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
  printf '{"status":"gap","reason":"boot command failed","exit":%s,"pcap":"%s"}\n' "$rc" "$pcap" > "$report"
  printf '%s\n' "no-telemetry capture: GAP boot command failed"
  exit 2
fi

if [ ! -f "$pcap" ]; then
  printf '{"status":"gap","reason":"pcap not produced","pcap":"%s"}\n' "$pcap" > "$report"
  printf '%s\n' "no-telemetry capture: GAP pcap not produced"
  exit 2
fi

bytes=$(wc -c < "$pcap" | tr -d ' ')

if [ "$bytes" -le 24 ]; then
  printf '{"status":"pass","pcap":"%s","bytes":%s}\n' "$pcap" "$bytes" > "$report"
  printf '%s\n' "no-telemetry capture: PASS no packets captured"
  exit 0
fi

printf '{"status":"fail","pcap":"%s","bytes":%s}\n' "$pcap" "$bytes" > "$report"
printf 'no-telemetry capture: FAIL captured %s bytes; review %s\n' "$bytes" "$pcap"
exit 1
