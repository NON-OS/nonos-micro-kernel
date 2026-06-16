#!/usr/bin/env bash
#
# nonos-package: turn an unmodified crates.io package into a signed NONOS
# capsule the running system can load at the live GUI, with no kernel
# rebuild. Two modes:
#
#   library:  scaffold a tiny harness that only calls the crate, then build
#             it against the std platform layer.
#   binary:   build the crate's own upstream binary (its real main, no
#             harness) and package that.
#
# Usage:
#   toolchain/nonos-package.sh <crate> [version]
#   toolchain/nonos-package.sh <crate> [version] --bin <binname>
#
# Examples:
#   toolchain/nonos-package.sh serde_json 1
#   toolchain/nonos-package.sh ripgrep 14.1.1 --bin rg
#
# Output: dist/<bin>/{capsule.elf,cert.bin,manifest.bin,trailer.bin}
# ready for the runtime loader (capsule_load) to verify and spawn.
set -euo pipefail

die() { printf 'nonos-package: %s\n' "$1" >&2; exit 1; }

CRATE=${1:?usage: nonos-package.sh <crate> [version] [--bin <binname>]}
VERSION="*"
MODE=lib
BINNAME="$CRATE"
shift
while [ $# -gt 0 ]; do
    case "$1" in
        --bin) MODE=bin; shift; [ $# -gt 0 ] && { BINNAME="$1"; shift; } ;;
        *) VERSION="$1"; shift ;;
    esac
done

REPO=$(git rev-parse --show-toplevel 2>/dev/null) || die "run inside the kernel repo"
cd "$REPO"

TARGET=userland/x86_64-nonos-user.json
RTOBJ="${NONOS_RT_OBJ:-$REPO/target/nonos_rt.o}"
SIGN=nonos-sign/target/release/capsule-sign
TC="${NONOS_TOOLCHAIN:-nightly-2026-01-16}"
[ -x "$SIGN" ] || die "capsule-sign not built (cargo build --release -p capsule-sign)"
[ -f "$RTOBJ" ] || die "nonos-rt object missing at $RTOBJ (build from toolchain/nonos-rt)"

SLUG=${CRATE//_/-}
HANDLE=app.${CRATE}
DIR=userland/capsule_${CRATE}
mkdir -p "$DIR"

if [ "$MODE" = bin ]; then
    # Build the crate's own binary for the target, unmodified, linking the
    # start shim. The result is the capsule ELF; no harness is written.
    UP=target/upstream-${CRATE}
    echo "[nonos-package] building upstream binary ${CRATE} (${BINNAME})"
    RUSTUP_TOOLCHAIN="$TC" RUSTFLAGS="-Clink-arg=${RTOBJ}" \
        cargo install "$CRATE" $([ "$VERSION" != "*" ] && printf -- "--version %s" "$VERSION") \
        --target "$TARGET" \
        -Zbuild-std=std,panic_abort -Zbuild-std-features=compiler-builtins-mem \
        --root "$UP" --no-track --force --bin "$BINNAME"
    PREBUILT="${UP}/bin/${BINNAME}"
    [ -f "$PREBUILT" ] || die "upstream build produced no ${PREBUILT}"
    cat > "$DIR/Capsule.mk" <<EOF
# ${CRATE} ${VERSION}: unmodified crates.io binary, packaged as a
# runtime-loadable NONOS capsule. Built from the crate's own main.
CAPSULE_SLUG               := ${SLUG}
CAPSULE_HANDLE             := ${HANDLE}
CAPSULE_DOMAIN             := crates.io
CAPSULE_DIR                := ${DIR}
CAPSULE_BIN_NAME           := ${BINNAME}
CAPSULE_NAMESPACE          := systems.nonos.${CRATE}
CAPSULE_SERVICE_ENDPOINT   := service:0:${HANDLE}
CAPSULE_REPLY_ENDPOINT     := reply:0:endpoint.${HANDLE}.reply
CAPSULE_REQUIRED_CAPS      := 0x19
CAPSULE_CAPS_CEILING       := 0x19
CAPSULE_PREBUILT_BIN       := ${PREBUILT}
CAPSULE_METADATA           := crates.io ${CRATE} ${VERSION} publisher
include nonos-mk/capsule.mk
EOF
else
    mkdir -p "$DIR/src" "$DIR/.cargo"
    cat > "$DIR/Cargo.toml" <<EOF
[package]
name = "nonos_capsule_${CRATE}"
version = "0.1.0"
edition = "2021"
publish = false

[[bin]]
name = "${CRATE}"
path = "src/main.rs"

[dependencies]
${CRATE} = "${VERSION}"

[profile.release]
panic = "abort"
opt-level = 2
strip = true
EOF
    cat > "$DIR/.cargo/config.toml" <<EOF
[target.x86_64-nonos-user]
rustflags = ["-Clink-arg=${RTOBJ}"]
EOF
    # The harness only invokes the crate so its execution is observable; it
    # holds no logic of its own. Replace it for a real entry point.
    [ -f "$DIR/src/main.rs" ] || cat > "$DIR/src/main.rs" <<EOF
fn main() {
    println!("nonos: ${CRATE} capsule live");
}
EOF
    cat > "$DIR/Capsule.mk" <<EOF
# ${CRATE} ${VERSION}: unmodified crates.io library, exercised by a harness
# and packaged as a runtime-loadable NONOS capsule via the std PAL.
CAPSULE_BUILD_STD          := std,panic_abort
CAPSULE_SLUG               := ${SLUG}
CAPSULE_HANDLE             := ${HANDLE}
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_DIR                := ${DIR}
CAPSULE_BIN_NAME           := ${CRATE}
CAPSULE_NAMESPACE          := systems.nonos.${CRATE}
CAPSULE_SERVICE_ENDPOINT   := service:0:${HANDLE}
CAPSULE_REPLY_ENDPOINT     := reply:0:endpoint.${HANDLE}.reply
CAPSULE_REQUIRED_CAPS      := 0x19
CAPSULE_METADATA           := crates.io ${CRATE} ${VERSION} publisher
include nonos-mk/capsule.mk
EOF
    BINNAME="$CRATE"
fi

grep -qF "include ${DIR}/Capsule.mk" Makefile \
    || die "add 'include ${DIR}/Capsule.mk' to the Makefile capsule include block, then re-run"

# Publisher keypair, keyed on the capsule bin name (seed local, pub in the
# trust dir), generated only if missing.
for alg in ed25519 mldsa65; do
    if [ ! -f ".keys/${BINNAME}_publisher_${alg}.seed" ]; then
        "$SIGN" keygen --alg "$alg" --out ".keys/${BINNAME}_publisher_${alg}"
        mv -f ".keys/${BINNAME}_publisher_${alg}.pub" \
              "nonos-data/trust/keys/${BINNAME}_publisher_${alg}.pub"
    fi
done

make "nonos-mk-${SLUG}-sign"

OUT=dist/${BINNAME}
mkdir -p "$OUT"
cp "nonos-data/trust/capsules/${BINNAME}.nonos_id_cert.bin" "$OUT/cert.bin"
cp "nonos-data/trust/capsules/${BINNAME}.manifest.bin"      "$OUT/manifest.bin"
cp "nonos-data/trust/capsules/${BINNAME}.zk_trailer.bin"    "$OUT/trailer.bin"
if [ "$MODE" = bin ]; then
    cp "$PREBUILT" "$OUT/capsule.elf"
else
    cp "${DIR}/target/x86_64-nonos-user/release/${CRATE}" "$OUT/capsule.elf"
fi

printf '\npackaged %s (%s mode) -> %s\n' "$CRATE" "$MODE" "$OUT"
ls -l "$OUT" | awk 'NR>1{print "  "$5"  "$NF}'
printf 'seed %s.* into the VFS store, then `install %s` from the terminal.\n' "$BINNAME" "$BINNAME"
