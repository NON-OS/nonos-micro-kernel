#!/usr/bin/env bash
# Replays the bootloader's TPM NV monotonic counter command sequence against a
# software TPM 2.0 and asserts the counter is defined, increments, and reads
# back monotonically. This proves the command encoding in
# nonos-bootloader/src/security/tpm_nv/consts.rs independently of UEFI firmware,
# which is useful because the QEMU-bundled OVMF ships no TCG2 protocol.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CONSTS="$ROOT/nonos-bootloader/src/security/tpm_nv/consts.rs"

for tool in swtpm xxd nc; do
    command -v "$tool" >/dev/null || { echo "gap: $tool not installed"; exit 1; }
done

D="$(mktemp -d)"
cleanup() { pkill -f "swtpm socket --tpmstate dir=$D" 2>/dev/null || true; rm -rf "$D"; }
trap cleanup EXIT

swtpm socket --tpmstate "dir=$D" --server "type=unixio,path=$D/data" \
    --ctrl "type=unixio,path=$D/ctrl" --tpm2 --flags not-need-init,startup-clear &
sleep 1

bytes() {
    awk "/$1: \\[u8/{f=1;next} f&&/];/{f=0} f" "$CONSTS" \
        | grep -oE '0x[0-9A-Fa-f]{2}' | sed 's/0x//' | tr -d '\n'
}
DEFINE="$(bytes DEFINE_COUNTER_CMD)"
INCR="$(bytes INCREMENT_CMD)"
READ="$(bytes READ_CMD)"

send() { printf '%s' "$1" | xxd -r -p | nc -U -w2 "$D/data" | xxd -p | tr -d '\n'; }
rc() { echo "${1:12:8}"; }
val() { echo "${1:32:16}"; }

check_rc() { [ "$(rc "$2")" = "00000000" ] || { echo "gap: $1 rc=0x$(rc "$2")"; exit 1; }; }

R="$(send "$DEFINE")"; check_rc define "$R"
R="$(send "$INCR")";   check_rc increment1 "$R"
R1="$(send "$READ")";  check_rc read1 "$R1"
R="$(send "$INCR")";   check_rc increment2 "$R"
R2="$(send "$READ")";  check_rc read2 "$R2"

C1=$((16#$(val "$R1")))
C2=$((16#$(val "$R2")))
echo "tpm-counter evidence: v1=$C1 v2=$C2"
[ "$C2" -gt "$C1" ] || { echo "gap: counter not monotonic"; exit 1; }
echo "PASS: TPM NV monotonic counter encoding verified against swtpm"
