#!/usr/bin/env bash
# Graft the NONOS relibc platform backend into the gitignored vendored relibc
# (third_party/redox/src-repos/relibc). The backend is authored as tracked
# source here; this copies it in and idempotently patches the vendored platform
# selector, a Sync errno static (kernel has no per-thread TLS), and config.mk
# (cc/ld block + an `override CARGOFLAGS` carrying the json target spec) so
# `make TARGET=x86_64-unknown-nonos libs` builds the nonos arm. Re-run after a
# relibc re-import or `git clean`. Idempotent.
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
REPO="$(cd "$HERE/../.." && pwd)"
RELIBC="$REPO/third_party/redox/src-repos/relibc"
CC_NONOS="$REPO/toolchain/nonos-c/cc-nonos"

# 1. copy the platform backend
mkdir -p "$RELIBC/src/platform/nonos"
cp "$HERE/platform/nonos/mod.rs"      "$RELIBC/src/platform/nonos/mod.rs"
cp "$HERE/platform/nonos/lowlevel.rs" "$RELIBC/src/platform/nonos/lowlevel.rs"
cp "$HERE/platform/nonos/socket.rs"   "$RELIBC/src/platform/nonos/socket.rs"
cp "$HERE/platform/nonos/signal.rs"   "$RELIBC/src/platform/nonos/signal.rs"
cp "$HERE/platform/nonos/epoll.rs"    "$RELIBC/src/platform/nonos/epoll.rs"
cp "$HERE/platform/nonos/ptrace.rs"   "$RELIBC/src/platform/nonos/ptrace.rs"

# 2. patch the platform selector + config.mk (idempotent)
python3 - "$RELIBC" "$CC_NONOS" <<'PY'
import re, sys
relibc, cc = sys.argv[1], sys.argv[2]
tgtdir = cc.rsplit("/", 1)[0]

modrs = f"{relibc}/src/platform/mod.rs"
with open(modrs) as f:
    s = f.read()
orig = s
if '#[path = "nonos/mod.rs"]' not in s:
    # vendored-relibc-version-specific anchor — re-check on a relibc re-import
    anchor = '\npub use self::rlb::{Line, RawLineBuffer};'
    arm = ('\n#[cfg(target_os = "nonos")]\n#[path = "nonos/mod.rs"]\n'
           'pub(crate) mod sys;\n')
    i = s.index(anchor)
    s = s[:i] + arm + s[i:]
errno_block = (
    '#[cfg(not(target_os = "nonos"))]\n'
    '#[thread_local]\n'
    'pub static ERRNO: Cell<c_int> = Cell::new(0);\n'
    '#[cfg(target_os = "nonos")]\n'
    'pub struct SyncErrno(Cell<c_int>);\n'
    '#[cfg(target_os = "nonos")]\n'
    'unsafe impl Sync for SyncErrno {}\n'
    '#[cfg(target_os = "nonos")]\n'
    'impl SyncErrno {\n'
    '    pub fn get(&self) -> c_int { self.0.get() }\n'
    '    pub fn set(&self, v: c_int) { self.0.set(v) }\n'
    '    pub fn as_ptr(&self) -> *mut c_int { self.0.as_ptr() }\n'
    '}\n'
    '#[cfg(target_os = "nonos")]\n'
    'pub static ERRNO: SyncErrno = SyncErrno(Cell::new(0));'
)
if 'struct SyncErrno' not in s:
    s = re.sub(
        r'(#\[cfg_attr\(not\(target_os = "nonos"\), thread_local\)\]|#\[thread_local\])'
        r'\npub static ERRNO: Cell<c_int> = Cell::new\(0\);',
        lambda _m: errno_block, s, count=1)
if s != orig:
    with open(modrs, "w") as f:
        f.write(s)

cfgmk = f"{relibc}/config.mk"
with open(cfgmk) as f:
    s = f.read()
block = ("\nifeq ($(TARGET),x86_64-unknown-nonos)\n"
         f"\texport CC={cc}\n"
         "\texport LD=ld.lld\n"
         "\texport AR=llvm-ar\n"
         "\texport NM=llvm-nm\n"
         "\texport CPPFLAGS=\n"
         "\texport CARGO_TEST=\n"
         "\toverride CARGOFLAGS := -Z build-std=core,alloc,compiler_builtins "
         f"-Z json-target-spec --target={tgtdir}/x86_64-unknown-nonos.json\n"
         "endif\n")
s = re.sub(r"\nifeq \(\$\(TARGET\),x86_64-unknown-nonos\).*?endif\n", "", s, flags=re.S)
with open(cfgmk, "w") as f:
    f.write(s.rstrip("\n") + "\n" + block)
PY

echo "NONOS relibc backend grafted into $RELIBC"
echo "  src/platform/nonos/{mod,lowlevel,socket,signal,epoll,ptrace}.rs"
echo "  cfg arm + Sync-errno patch + config.mk TARGET block (override CARGOFLAGS)"
