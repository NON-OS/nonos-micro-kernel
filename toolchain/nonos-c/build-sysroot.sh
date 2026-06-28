#!/usr/bin/env bash
# Assemble the NONOS C sysroot from the vendored relibc build output:
# sysroot/lib/{libc.a,crt0.o,crti.o,crtn.o} + sysroot/include (cbindgen +
# hand-written headers). Static-link only — libc.so/ld.so are deliberately
# not built (the nonos capsule link is fully static), so this builds the
# specific targets rather than `all`. Re-run after a relibc change. Idempotent.
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
REPO="$(cd "$HERE/../.." && pwd)"
RELIBC="$REPO/third_party/redox/src-repos/relibc"
OUT="$HERE/sysroot"
B="$RELIBC/target/x86_64-unknown-nonos/release"

export PATH="/usr/local/opt/llvm/bin:/usr/local/opt/lld/bin:/usr/local/opt/findutils/libexec/gnubin:$PATH"

"$REPO/toolchain/nonos-relibc/apply.sh"

make -C "$RELIBC" TARGET=x86_64-unknown-nonos NONOS_C_DIR="$HERE" USE_RUST_LIBM=1 \
	headers "$B/libc.a" "$B/crt0.o" "$B/crti.o" "$B/crtn.o"

rm -rf "$OUT"
mkdir -p "$OUT/lib" "$OUT/include"
cp "$B/libc.a" "$B/crt0.o" "$B/crti.o" "$B/crtn.o" "$OUT/lib/"
cp -r "$RELIBC/target/x86_64-unknown-nonos/include/." "$OUT/include/"
echo "[sysroot] assembled at $OUT"
