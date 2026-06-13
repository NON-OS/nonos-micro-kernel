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

out=${NONOS_QEMU_NET_AUDIT_OUT:-target/qemu-network-audit}
mkdir -p "$out"
default_cmd="$out/default-serial.mkdryrun"
nat_cmd="$out/nat-serial.mkdryrun"
net_cmd="$out/net-serial.mkdryrun"
nat_capture_cmd="$out/nat-capture-serial.mkdryrun"
capture_cmd="$out/net-capture-serial.mkdryrun"

make -n nonos-mk-run-serial > "$default_cmd"
make -n nonos-mk-run-serial-nat > "$nat_cmd"
make -n nonos-mk-run-serial-net > "$net_cmd"
make -n QEMU_NET_CAPTURE="$out/qemu-nat.pcap" nonos-mk-run-serial-nat > "$nat_capture_cmd"
make -n QEMU_NET_CAPTURE="$out/qemu-net.pcap" nonos-mk-run-serial-net > "$capture_cmd"

fail=0

if grep -E 'virtio-net-pci|hostfwd=tcp' "$default_cmd" >/dev/null 2>&1; then
  printf '%s\n' "FAIL: default serial QEMU boot attaches network authority"
  fail=1
else
  printf '%s\n' "PASS: default serial QEMU boot has no NIC or hostfwd"
fi

if grep -E 'virtio-net-pci' "$nat_cmd" >/dev/null 2>&1 && \
   grep -E -- '-netdev user,id=net0' "$nat_cmd" >/dev/null 2>&1 && \
   ! grep -E 'hostfwd=tcp' "$nat_cmd" >/dev/null 2>&1; then
  printf '%s\n' "PASS: explicit NAT serial boot attaches outbound-only NIC"
else
  printf '%s\n' "FAIL: explicit NAT serial boot is missing outbound-only NIC"
  fail=1
fi

if grep -E 'virtio-net-pci' "$net_cmd" >/dev/null 2>&1 && \
   grep -E 'hostfwd=tcp::[0-9]+-:22' "$net_cmd" >/dev/null 2>&1 && \
   grep -E 'hostfwd=tcp::[0-9]+-:80' "$net_cmd" >/dev/null 2>&1; then
  printf '%s\n' "PASS: explicit network serial boot attaches hostfwd NIC"
else
  printf '%s\n' "FAIL: explicit network serial boot is missing hostfwd NIC"
  fail=1
fi

if grep -E 'filter-dump' "$nat_capture_cmd" >/dev/null 2>&1 && \
   grep -F "file=$out/qemu-nat.pcap" "$nat_capture_cmd" >/dev/null 2>&1; then
  printf '%s\n' "PASS: explicit NAT capture serial boot attaches filter-dump"
else
  printf '%s\n' "FAIL: explicit NAT capture serial boot is missing filter-dump"
  fail=1
fi

if grep -E 'filter-dump' "$capture_cmd" >/dev/null 2>&1 && \
   grep -F "file=$out/qemu-net.pcap" "$capture_cmd" >/dev/null 2>&1; then
  printf '%s\n' "PASS: explicit network capture serial boot attaches filter-dump"
else
  printf '%s\n' "FAIL: explicit network capture serial boot is missing filter-dump"
  fail=1
fi

exit "$fail"
