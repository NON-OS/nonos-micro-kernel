#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT"

out=${NONOS_ZERO_STATE_OUT:-target/zero-state-forensic}
mkdir -p "$out"
marker="NONOS_SECRET_MARKER_$(date +%s)_$$"
before="$out/before.sha256"
after="$out/after.sha256"
scan="$out/marker-scan.txt"
report="$out/report.json"

images=${NONOS_ZERO_STATE_IMAGES:-"target/qemu-virtio-blk.img target/qemu-OVMF_VARS.fd target/esp/EFI/nonos/kernel.bin target/kernel_attested.bin"}

hash_images() {
  label=$1
  : > "$label"
  for img in $images; do
    if [ -f "$img" ]; then
      shasum -a 256 "$img" >> "$label"
    fi
  done
}

hash_images "$before"

if [ -z "${NONOS_ZERO_STATE_BOOT_CMD:-}" ]; then
  cat > "$report" <<EOF
{"status":"gap","marker":"$marker","reason":"NONOS_ZERO_STATE_BOOT_CMD is unset","expected":"set NONOS_ZERO_STATE_BOOT_CMD to a bounded QEMU boot command that injects or logs the marker"}
EOF
  echo "GAP: set NONOS_ZERO_STATE_BOOT_CMD to run the forensic boot harness"
  exit 2
fi

NONOS_SECRET_MARKER=$marker sh -c "$NONOS_ZERO_STATE_BOOT_CMD"

hash_images "$after"
: > "$scan"
found=0
for img in $images; do
  if [ -f "$img" ] && grep -a -F "$marker" "$img" >/dev/null 2>&1; then
    echo "$img" >> "$scan"
    found=1
  fi
done

if [ "$found" -eq 0 ]; then
  status=pass
else
  status=fail
fi

cat > "$report" <<EOF
{"status":"$status","marker":"$marker","before":"$before","after":"$after","scan":"$scan","images":"$images"}
EOF

if [ "$found" -eq 0 ]; then
  echo "zero-state forensic scan: PASS"
else
  echo "zero-state forensic scan: FAIL; marker found in:"
  cat "$scan"
fi
exit "$found"
