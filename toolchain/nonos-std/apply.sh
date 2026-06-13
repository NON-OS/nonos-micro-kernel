#!/usr/bin/env bash
# Apply the NONOS std platform layer (PAL) into the pinned rust-src so that
# `-Zbuild-std=std` produces NONOS binaries from unmodified `use std::...` code.
#
# The PAL is keyed on `target_vendor = "nonos"` (the capsule target sets it),
# so existing capsules that build core+alloc are unaffected. Re-run after any
# `rustup` update of the toolchain. Idempotent.
#
# Usage: RUSTUP_TOOLCHAIN=nightly-2026-01-16 toolchain/nonos-std/apply.sh
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
TC="${RUSTUP_TOOLCHAIN:-nightly-2026-01-16}"
SR="$(RUSTUP_TOOLCHAIN="$TC" rustc --print sysroot)"
STD="$SR/lib/rustlib/src/rust/library/std"
SYS="$STD/src/sys"

cp "$HERE/sys/alloc/nonos.rs"    "$SYS/alloc/nonos.rs"
cp "$HERE/sys/io/error/nonos.rs" "$SYS/io/error/nonos.rs"
cp "$HERE/sys/random/nonos.rs"   "$SYS/random/nonos.rs"

python3 - "$SYS" "$STD" <<'PY'
import sys
sysdir, stddir = sys.argv[1], sys.argv[2]

def edit(path, anchor, insert, guard):
    with open(path) as f:
        s = f.read()
    if guard in s:
        return
    i = s.index(anchor)
    s = s[:i] + insert + s[i:]
    with open(path, "w") as f:
        f.write(s)

# 1. allocator
edit(f"{sysdir}/alloc/mod.rs",
     '    any(\n        target_family = "unix",',
     '    target_vendor = "nonos" => {\n        mod nonos;\n    }\n',
     'mod nonos;')

# 2. io error decoding
edit(f"{sysdir}/io/error/mod.rs",
     '    target_os = "hermit" => {',
     '    target_vendor = "nonos" => {\n        mod nonos;\n        pub use nonos::*;\n    }\n',
     'target_vendor = "nonos"')

# 3. randomness
edit(f"{sysdir}/random/mod.rs",
     '    // Tier 1\n',
     '    target_vendor = "nonos" => {\n        mod nonos;\n        pub use nonos::fill_bytes;\n    }\n',
     'target_vendor = "nonos"')

# 4. thread locals -> no_threads + no-op destructor guard
tl = f"{sysdir}/thread_local/mod.rs"
with open(tl) as f:
    s = f.read()
s = s.replace(
    '        target_os = "vexos",\n    ) => {\n        mod no_threads;',
    '        target_os = "vexos",\n        target_vendor = "nonos",\n    ) => {\n        mod no_threads;')
s = s.replace(
    '            target_os = "vexos",\n        ) => {\n            pub(crate) fn enable() {',
    '            target_os = "vexos",\n            target_vendor = "nonos",\n        ) => {\n            pub(crate) fn enable() {')
with open(tl, "w") as f:
    f.write(s)

# 5. std is a known (non-restricted) platform for nonos
br = f"{stddir}/build.rs"
with open(br) as f:
    s = f.read()
if 'target_vendor == "nonos"' not in s:
    s = s.replace('|| target_os == "vexos"\n',
                  '|| target_os == "vexos"\n        || target_vendor == "nonos"\n', 1)
    with open(br, "w") as f:
        f.write(s)
PY

echo "NONOS std PAL applied to $STD"
