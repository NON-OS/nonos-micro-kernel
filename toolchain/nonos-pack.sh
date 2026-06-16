#!/usr/bin/env bash
# nonos-pack: build an unmodified std Rust crate into a NONOS capsule
# executable. It compiles the crate against the NONOS std platform layer
# (-Zbuild-std=std) and links the nonos-rt startup object, producing a
# static-PIE NONOS ELF. Signing, attestation, and publishing into the
# trust policy are the next stage (the per-slug nonos-mk-<slug>-sign
# targets); a capsule must be in the baked ZK policy to be accepted, so
# install is a deliberate publish step, not arbitrary self-install.
#
# Usage: toolchain/nonos-pack.sh <crate-dir>
set -euo pipefail

CRATE="${1:?usage: nonos-pack.sh <crate-dir>}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TC="${RUSTUP_TOOLCHAIN:-nightly-2026-01-16}"
TGT="$ROOT/userland/x86_64-nonos-user.json"
RT="$ROOT/toolchain/nonos-rt"
RTOBJ="${NONOS_RT_OBJ:-$ROOT/target/nonos_rt.o}"
FLAGS="-Zbuild-std=std,panic_abort -Zbuild-std-features=compiler-builtins-mem"
export PATH="$HOME/.cargo/bin:$PATH"
export RUSTUP_TOOLCHAIN="$TC"

if [ -x "$ROOT/toolchain/nonos-std/apply.sh" ]; then
    echo "[nonos-pack] applying NONOS std platform layer"
    "$ROOT/toolchain/nonos-std/apply.sh" >/dev/null
fi

if [ -f "$RT/Cargo.toml" ]; then
    echo "[nonos-pack] building startup object (nonos-rt _start)"
    cargo rustc --manifest-path "$RT/Cargo.toml" --release --target "$TGT" \
        -Zbuild-std=core -- --emit "obj=$RTOBJ" >/dev/null
elif [ ! -f "$RTOBJ" ]; then
    echo "[nonos-pack] error: nonos-rt source missing and no prebuilt $RTOBJ"; exit 1
else
    echo "[nonos-pack] using existing startup object $RTOBJ"
fi

echo "[nonos-pack] building crate against std: $CRATE"
( cd "$CRATE" && RUSTFLAGS="-Clink-arg=$RTOBJ" cargo build --release --target "$TGT" $FLAGS )

DIR="$CRATE/target/x86_64-nonos-user/release"
ELF="$(find "$DIR" -maxdepth 1 -type f -perm -u+x ! -name '*.d' ! -name '*.rlib' 2>/dev/null | head -1)"
[ -n "$ELF" ] || { echo "[nonos-pack] error: no executable produced"; exit 1; }

echo "[nonos-pack] built: $ELF"
file "$ELF"
echo "[nonos-pack] next: sign + attest + publish into the trust policy to install."
