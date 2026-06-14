#!/usr/bin/env bash
# Apply the NONOS std platform layer (PAL) into the pinned rust-src so that
# `-Zbuild-std=std` produces NONOS binaries from unmodified `use std::...`
# code. Keyed on `target_vendor = "nonos"` (the capsule target sets it), so
# capsules that build core+alloc are unaffected. Idempotent; re-run after a
# `rustup` update. Usage: RUSTUP_TOOLCHAIN=nightly-2026-01-16 apply.sh
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
TC="${RUSTUP_TOOLCHAIN:-nightly-2026-01-16}"
SR="$(RUSTUP_TOOLCHAIN="$TC" rustc --print sysroot)"
STD="$SR/lib/rustlib/src/rust/library/std"
SYS="$STD/src/sys"

# 1. copy the platform modules
cp "$HERE/sys/alloc/nonos.rs"            "$SYS/alloc/nonos.rs"
cp "$HERE/sys/io/error/nonos.rs"         "$SYS/io/error/nonos.rs"
cp "$HERE/sys/random/nonos.rs"           "$SYS/random/nonos.rs"
cp "$HERE/sys/stdio/nonos.rs"            "$SYS/stdio/nonos.rs"
cp "$HERE/sys/args/nonos.rs"             "$SYS/args/nonos.rs"
cp "$HERE/sys/fs/nonos.rs"               "$SYS/fs/nonos.rs"
mkdir -p "$SYS/pal/nonos" "$SYS/net/connection"
cp "$HERE/sys/pal/nonos/mod.rs"          "$SYS/pal/nonos/mod.rs"
cp "$HERE/sys/pal/nonos/os.rs"           "$SYS/pal/nonos/os.rs"
cp "$HERE/sys/pal/nonos/time.rs"         "$SYS/pal/nonos/time.rs"
cp "$HERE/sys/net/connection/nonos.rs"   "$SYS/net/connection/nonos.rs"
mkdir -p "$SYS/thread"
cp "$HERE/sys/thread/nonos.rs"           "$SYS/thread/nonos.rs"

# 2. patch the cfg_select selectors + build.rs (idempotent)
python3 - "$SYS" "$STD" <<'PY'
import sys
sysdir, stddir = sys.argv[1], sys.argv[2]

def insert_before(path, anchor, block, guard):
    with open(path) as f:
        s = f.read()
    if guard in s:
        return
    i = s.index(anchor)
    with open(path, "w") as f:
        f.write(s[:i] + block + s[i:])

ARM = '    target_vendor = "nonos" => {\n        mod nonos;\n        pub use nonos::*;\n    }\n'
ARM_BARE = '    target_vendor = "nonos" => {\n        mod nonos;\n    }\n'
ARM_FILL = '    target_vendor = "nonos" => {\n        pub use nonos::fill_bytes;\n        mod nonos;\n    }\n'
ARM_IMP = '    target_vendor = "nonos" => {\n        mod nonos;\n        use nonos as imp;\n    }\n'
ARM_PAL = '    target_vendor = "nonos" => {\n        mod nonos;\n        pub use self::nonos::*;\n    }\n'

insert_before(f"{sysdir}/alloc/mod.rs", '    any(\n        target_family = "unix",', ARM_BARE, 'mod nonos')
insert_before(f"{sysdir}/io/error/mod.rs", '    target_os = "hermit" => {', ARM, 'target_vendor = "nonos"')
insert_before(f"{sysdir}/random/mod.rs", '    // Tier 1\n', ARM_FILL, 'target_vendor = "nonos"')
insert_before(f"{sysdir}/stdio/mod.rs", '    any(target_family = "unix"', ARM, 'target_vendor = "nonos"')
insert_before(f"{sysdir}/fs/mod.rs", '    any(target_family = "unix", target_os = "wasi") => {', ARM_IMP, 'target_vendor = "nonos"')
insert_before(f"{sysdir}/pal/mod.rs", '    unix => {', ARM_PAL, 'target_vendor = "nonos"')
insert_before(f"{sysdir}/net/connection/mod.rs", '    any(\n        all(target_family = "unix", not(target_os = "l4re")),', ARM, 'target_vendor = "nonos"')

ARM_THREAD = ('    target_vendor = "nonos" => {\n        mod nonos;\n'
    '        pub use nonos::{\n            Thread, available_parallelism, current_os_id, set_name, sleep, yield_now,\n'
    '            DEFAULT_MIN_STACK_SIZE,\n        };\n    }\n')
insert_before(f"{sysdir}/thread/mod.rs", '    target_os = "hermit" => {', ARM_THREAD, 'target_vendor = "nonos"')

# args: arm + add nonos to the gated `mod common`
insert_before(f"{sysdir}/args/mod.rs",
    '    any(\n        all(target_family = "unix", not(any(target_os = "espidf"',
    ARM, 'target_vendor = "nonos"')
with open(f"{sysdir}/args/mod.rs") as f:
    s = f.read()
if 'target_os = "xous",\n    target_vendor = "nonos",\n))]\nmod common;' not in s:
    s = s.replace('    target_os = "xous",\n))]\nmod common;',
                  '    target_os = "xous",\n    target_vendor = "nonos",\n))]\nmod common;')
    with open(f"{sysdir}/args/mod.rs", "w") as f:
        f.write(s)

# thread_local: route nonos to no_threads + a no-op destructor guard
with open(f"{sysdir}/thread_local/mod.rs") as f:
    s = f.read()
s = s.replace('        target_os = "vexos",\n    ) => {\n        mod no_threads;',
              '        target_os = "vexos",\n        target_vendor = "nonos",\n    ) => {\n        mod no_threads;')
s = s.replace('            target_os = "vexos",\n        ) => {\n            pub(crate) fn enable() {',
              '            target_os = "vexos",\n            target_vendor = "nonos",\n        ) => {\n            pub(crate) fn enable() {')
with open(f"{sysdir}/thread_local/mod.rs", "w") as f:
    f.write(s)

# Cargo.toml: make dlmalloc a dependency for nonos (the real heap)
with open(f"{stddir}/Cargo.toml") as f:
    s = f.read()
gate = 'all(target_family = "wasm", target_os = "unknown"), target_os = "xous", target_os = "vexos", '
if gate in s and 'target_vendor = "nonos", all(target_vendor = "fortanix"' not in s:
    s = s.replace(gate, gate + 'target_vendor = "nonos", ')
    with open(f"{stddir}/Cargo.toml", "w") as f:
        f.write(s)

# build.rs: mark nonos a known (non-restricted) platform
with open(f"{stddir}/build.rs") as f:
    s = f.read()
if 'target_vendor == "nonos"' not in s:
    s = s.replace('|| target_os == "vexos"\n',
                  '|| target_os == "vexos"\n        || target_vendor == "nonos"\n', 1)
    with open(f"{stddir}/build.rs", "w") as f:
        f.write(s)
PY

echo "NONOS std PAL applied to $STD"
echo "modules: alloc, io_error, random, stdio, args, fs, pal(time), net(connection)"
