#!/bin/sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT"

out=${NONOS_REPRO_OUT:-target/repro-build}
cmd=${NONOS_REPRO_BUILD_CMD:-"make nonos-mk-verify-fast"}
artifacts=${NONOS_REPRO_ARTIFACTS:-"target/nonos-kernel.x86_64 target/kernel_signed.bin target/attestation-receipt.txt"}
epoch=${SOURCE_DATE_EPOCH:-$(git log -1 --format=%ct 2>/dev/null || date +%s)}
commit=$(git rev-parse HEAD 2>/dev/null || printf unknown)

rm -rf "$out"
mkdir -p "$out"

if [ -n "$(git status --porcelain 2>/dev/null)" ] && [ "${NONOS_REPRO_ALLOW_DIRTY:-0}" != "1" ]; then
  echo "FAIL: git tree is dirty; set NONOS_REPRO_ALLOW_DIRTY=1 for development-only repro probes" >&2
  exit 1
fi

for lane in A B; do
  work="$out/work-$lane"
  git worktree add --detach "$work" "$commit" >/dev/null
  (
    cd "$work"
    export SOURCE_DATE_EPOCH=$epoch
    export CARGO_INCREMENTAL=0
    sh -c "$cmd" > "$ROOT/$out/build-$lane.log" 2>&1
    : > "$ROOT/$out/hashes-$lane.txt"
    for artifact in $artifacts; do
      if [ -f "$artifact" ]; then
        shasum -a 256 "$artifact" >> "$ROOT/$out/hashes-$lane.txt"
      else
        echo "MISSING  $artifact" >> "$ROOT/$out/hashes-$lane.txt"
      fi
    done
  )
done

if cmp -s "$out/hashes-A.txt" "$out/hashes-B.txt"; then
  status=pass
else
  status=fail
fi

rustc_v=$(rustc --version 2>/dev/null || printf unknown)
cargo_v=$(cargo --version 2>/dev/null || printf unknown)

cat > "$out/report.json" <<EOF
{"status":"$status","commit":"$commit","source_date_epoch":"$epoch","command":"$cmd","artifacts":"$artifacts","rustc":"$rustc_v","cargo":"$cargo_v","hashes_a":"$out/hashes-A.txt","hashes_b":"$out/hashes-B.txt"}
EOF

if [ "$status" = pass ]; then
  echo "repro build: PASS"
  exit 0
fi

echo "repro build: FAIL"
diff -u "$out/hashes-A.txt" "$out/hashes-B.txt" || true
exit 1
