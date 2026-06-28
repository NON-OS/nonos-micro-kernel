#!/usr/bin/env bash
# Graft the NONOS relibc platform backend into the gitignored vendored relibc
# (third_party/redox/src-repos/relibc). The backend is authored as tracked
# source here; this copies it in and idempotently patches the vendored platform
# selector + config.mk so `make TARGET=x86_64-unknown-nonos headers` builds the
# nonos arm. Re-run after a relibc re-import or `git clean`. Idempotent.
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
REPO="$(cd "$HERE/../.." && pwd)"
RELIBC="$REPO/third_party/redox/src-repos/relibc"
CC_NONOS="$REPO/toolchain/nonos-c/cc-nonos"

# 1. copy the platform backend
mkdir -p "$RELIBC/src/platform/nonos"
cp "$HERE/platform/nonos/mod.rs"      "$RELIBC/src/platform/nonos/mod.rs"
cp "$HERE/platform/nonos/lowlevel.rs" "$RELIBC/src/platform/nonos/lowlevel.rs"

# 2. patch the platform selector + config.mk (idempotent)
python3 - "$RELIBC" "$CC_NONOS" <<'PY'
import sys
relibc, cc = sys.argv[1], sys.argv[2]

modrs = f"{relibc}/src/platform/mod.rs"
with open(modrs) as f:
    s = f.read()
if 'target_os = "nonos"' not in s:
    # vendored-relibc-version-specific anchor — re-check on a relibc re-import
    anchor = '\npub use self::rlb::{Line, RawLineBuffer};'
    arm = ('\n#[cfg(target_os = "nonos")]\n#[path = "nonos/mod.rs"]\n'
           'pub(crate) mod sys;\n')
    i = s.index(anchor)
    with open(modrs, "w") as f:
        f.write(s[:i] + arm + s[i:])

cfgmk = f"{relibc}/config.mk"
with open(cfgmk) as f:
    s = f.read()
if 'x86_64-unknown-nonos' not in s:
    block = ("\nifeq ($(TARGET),x86_64-unknown-nonos)\n"
             f"\texport CC={cc}\n"
             "\texport LD=ld.lld\n"
             "\texport AR=llvm-ar\n"
             "\texport NM=llvm-nm\n"
             "\texport CPPFLAGS=\n"
             "\texport CARGO_TEST=\n"
             "endif\n")
    with open(cfgmk, "w") as f:
        f.write(s.rstrip("\n") + "\n" + block)
PY

echo "NONOS relibc backend grafted into $RELIBC"
echo "  src/platform/nonos/{mod.rs,lowlevel.rs}; cfg arm + config.mk TARGET block"
