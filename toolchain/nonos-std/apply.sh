#!/usr/bin/env bash
# Apply the NONOS std platform layer (PAL) into the pinned rust-src so that
# `-Zbuild-std=std` produces NONOS binaries from unmodified `use std::...`
# code. Keyed on `target_vendor = "nonos"` (the capsule target sets it), so
# capsules that build core+alloc are unaffected. Idempotent; re-run after a
# `rustup` update. Usage: RUSTUP_TOOLCHAIN=nightly-2026-01-16 apply.sh
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
TC="${RUSTUP_TOOLCHAIN:-nightly-2026-01-16}"
SR="$(rustup run "$TC" rustc --print sysroot)"
STD="$SR/lib/rustlib/src/rust/library/std"
SYS="$STD/src/sys"

# 1. copy the platform modules
cp "$HERE/sys/alloc/nonos.rs"            "$SYS/alloc/nonos.rs"
cp "$HERE/sys/io/error/nonos.rs"         "$SYS/io/error/nonos.rs"
cp "$HERE/sys/random/nonos.rs"           "$SYS/random/nonos.rs"
cp "$HERE/sys/stdio/nonos.rs"            "$SYS/stdio/nonos.rs"
cp "$HERE/sys/args/nonos.rs"             "$SYS/args/nonos.rs"
cp "$HERE/sys/env/nonos.rs"              "$SYS/env/nonos.rs"
# fs and net/connection are modularized (one unit per file), so install the
# whole directory. Clear any stale flat `nonos.rs` first: a file and a
# directory for the same module are an E0761 ambiguity.
rm -rf "$SYS/fs/nonos" "$SYS/fs/nonos.rs"
cp -R "$HERE/sys/fs/nonos"                "$SYS/fs/nonos"
mkdir -p "$SYS/pal/nonos" "$SYS/net/connection"
cp "$HERE/sys/pal/nonos/mod.rs"          "$SYS/pal/nonos/mod.rs"
cp "$HERE/sys/pal/nonos/os.rs"           "$SYS/pal/nonos/os.rs"
cp "$HERE/sys/pal/nonos/time.rs"         "$SYS/pal/nonos/time.rs"
cp "$HERE/sys/pal/nonos/common.rs"       "$SYS/pal/nonos/common.rs"
cp "$HERE/sys/pal/nonos/futex.rs"        "$SYS/pal/nonos/futex.rs"
rm -rf "$SYS/net/connection/nonos" "$SYS/net/connection/nonos.rs"
cp -R "$HERE/sys/net/connection/nonos"   "$SYS/net/connection/nonos"
rm -rf "$SYS/fd/nonos" "$SYS/fd/nonos.rs"
cp -R "$HERE/sys/fd/nonos"               "$SYS/fd/nonos"
mkdir -p "$SYS/thread" "$SYS/thread_local/key"
cp "$HERE/sys/thread/nonos.rs"           "$SYS/thread/nonos.rs"
cp "$HERE/sys/thread_local_key/nonos.rs" "$SYS/thread_local/key/nonos.rs"

# Shared std files the os::fd port must patch in place: keep pristine copies
# so a broken edit can be reverted to known-good with one cp.
for f in "$STD/src/os/mod.rs" "$STD/src/os/fd/owned.rs" "$STD/src/os/fd/raw.rs" \
         "$STD/src/os/fd/mod.rs" "$SYS/fd/mod.rs" "$SYS/pipe/unsupported.rs"; do
  [ -f "$f.nonos-orig" ] || cp "$f" "$f.nonos-orig"
done

# 2. patch the cfg_select selectors + build.rs (idempotent). Prefer the
# system interpreter: a broken Homebrew python3 exits 0 without running
# anything, which would leave the sysroot half-patched while this script
# reports success.
PY_BIN=/usr/bin/python3
command -v "$PY_BIN" >/dev/null 2>&1 || PY_BIN=python3
"$PY_BIN" - "$SYS" "$STD" <<'PY'
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

# env: nonos keeps a process-local map (sgx-style); it needs the shared
# Env iterator from env/common.rs, so join the gated `mod common` list.
insert_before(f"{sysdir}/env/mod.rs", '    target_os = "hermit" => {', ARM, 'target_vendor = "nonos"')
with open(f"{sysdir}/env/mod.rs") as f:
    s = f.read()
if '    target_os = "xous",\n    target_vendor = "nonos",\n))]\nmod common;' not in s:
    s = s.replace('    target_os = "xous",\n))]\nmod common;',
                  '    target_os = "xous",\n    target_vendor = "nonos",\n))]\nmod common;')
    with open(f"{sysdir}/env/mod.rs", "w") as f:
        f.write(s)

# thread_local: nonos takes the os backend (library TLS keys). The keys
# live in a per-thread table whose address the kernel carries in fs base,
# and the PAL runs destructors itself at thread exit, so the guard stays a
# no-op. An older apply routed nonos to no_threads; undo that first so
# re-applying over a patched tree converges on the os backend.
with open(f"{sysdir}/thread_local/mod.rs") as f:
    s = f.read()
s = s.replace('        target_os = "vexos",\n        target_vendor = "nonos",\n    ) => {\n        mod no_threads;',
              '        target_os = "vexos",\n    ) => {\n        mod no_threads;')
s = s.replace('            target_os = "vexos",\n        ) => {\n            pub(crate) fn enable() {',
              '            target_os = "vexos",\n            target_vendor = "nonos",\n        ) => {\n            pub(crate) fn enable() {')
with open(f"{sysdir}/thread_local/mod.rs", "w") as f:
    f.write(s)

KEY_ARM = ('        target_vendor = "nonos" => {\n'
    '            mod racy;\n'
    '            mod nonos;\n'
    '            pub(super) use racy::LazyKey;\n'
    '            pub(crate) use nonos::{destroy_tls, init_tls};\n'
    '            pub(super) use nonos::{Key, get, set};\n'
    '            use nonos::{create, destroy};\n'
    '        }\n')
insert_before(f"{sysdir}/thread_local/mod.rs",
    '        target_os = "xous" => {\n            mod racy;',
    KEY_ARM, 'use nonos::{create, destroy};')

# sync + parking: nonos rides the futex backends over the PAL polling futex
def extend_futex_arm(path, tail):
    with open(path) as f:
        s = f.read()
    if 'target_vendor = "nonos"' in s:
        return
    s = s.replace(tail, '        target_vendor = "nonos",\n' + tail, 1)
    with open(path, "w") as f:
        f.write(s)

HERMIT_TAIL = '        target_os = "hermit",\n    ) => {\n        mod futex;'
extend_futex_arm(f"{sysdir}/sync/mutex/mod.rs", HERMIT_TAIL)
extend_futex_arm(f"{sysdir}/sync/condvar/mod.rs", HERMIT_TAIL)
extend_futex_arm(f"{sysdir}/sync/once/mod.rs", HERMIT_TAIL)
extend_futex_arm(f"{sysdir}/sync/thread_parking/mod.rs", HERMIT_TAIL)
extend_futex_arm(f"{sysdir}/sync/rwlock/mod.rs",
    '       target_os = "motor",\n    ) => {\n        mod futex;')

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

# os::fd for nonos: enable the module, select the sys/fd/nonos backend, and
# route the dup/close/stdio paths through the descriptor table instead of
# libc. Each edit indexes its anchor so a rust-src drift fails loudly here,
# not as a mid-build compile error.

def sub_once(path, old, new, guard):
    with open(path) as f:
        s = f.read()
    if guard in s:
        return
    i = s.index(old)
    with open(path, "w") as f:
        f.write(s[:i] + new + s[i + len(old):])

# the public module gate
sub_once(f"{stddir}/src/os/mod.rs",
    '    target_os = "motor",\n    doc\n))]\npub mod fd;',
    '    target_os = "motor",\n    target_vendor = "nonos",\n    doc\n))]\npub mod fd;',
    'target_vendor = "nonos",\n    doc\n))]\npub mod fd;')

# the platform FileDesc backend
insert_before(f"{sysdir}/fd/mod.rs", '    _ => {}', ARM, 'target_vendor = "nonos"')

# owned.rs: cvt is the libc-errno helper; nonos has no libc
sub_once(f"{stddir}/src/os/fd/owned.rs",
    '    target_os = "motor"\n)))]\nuse crate::sys::cvt;',
    '    target_os = "motor",\n    target_vendor = "nonos"\n)))]\nuse crate::sys::cvt;',
    'target_vendor = "nonos"\n)))]\nuse crate::sys::cvt;')

# owned.rs: keep nonos out of the fcntl(F_DUPFD) arm ...
sub_once(f"{stddir}/src/os/fd/owned.rs",
    '        target_os = "motor"\n    )))]\n    #[stable(feature = "io_safety", since = "1.63.0")]\n    pub fn try_clone_to_owned(&self) -> io::Result<OwnedFd> {',
    '        target_os = "motor",\n        target_vendor = "nonos"\n    )))]\n    #[stable(feature = "io_safety", since = "1.63.0")]\n    pub fn try_clone_to_owned(&self) -> io::Result<OwnedFd> {',
    'target_vendor = "nonos"\n    )))]')

# ... and give it a descriptor-table dup arm instead
DUP_ARM = ('    /// Creates a new `OwnedFd` instance that shares the same underlying file\n'
    '    /// description as the existing `BorrowedFd` instance.\n'
    '    #[cfg(target_vendor = "nonos")]\n'
    '    #[stable(feature = "io_safety", since = "1.63.0")]\n'
    '    pub fn try_clone_to_owned(&self) -> io::Result<OwnedFd> {\n'
    '        let fd = crate::sys::fd::dup_fd(self.as_raw_fd())?;\n'
    '        // SAFETY: `dup_fd` installed a fresh descriptor owned by no other value.\n'
    '        Ok(unsafe { OwnedFd::from_raw_fd(fd) })\n'
    '    }\n\n')
insert_before(f"{stddir}/src/os/fd/owned.rs",
    '    /// Creates a new `OwnedFd` instance that shares the same underlying file\n    /// description as the existing `BorrowedFd` instance.\n    #[cfg(any(target_arch = "wasm32", target_os = "hermit", target_os = "trusty"))]',
    DUP_ARM, 'crate::sys::fd::dup_fd')

# owned.rs: Drop closes through the table, not libc
sub_once(f"{stddir}/src/os/fd/owned.rs",
    '            #[cfg(not(target_os = "hermit"))]',
    '            #[cfg(not(any(target_os = "hermit", target_vendor = "nonos")))]',
    'not(any(target_os = "hermit", target_vendor = "nonos"))')
sub_once(f"{stddir}/src/os/fd/owned.rs",
    '            #[cfg(target_os = "hermit")]\n            let _ = hermit_abi::close(self.fd.as_inner());',
    '            #[cfg(target_os = "hermit")]\n            let _ = hermit_abi::close(self.fd.as_inner());\n'
    '            #[cfg(target_vendor = "nonos")]\n            crate::sys::fd::close_fd(self.fd.as_inner());',
    'crate::sys::fd::close_fd')

# raw.rs: OwnedFd for the fs::File conversions, and the stdio descriptor
# constants in place of libc (the hermit/motor aliasing pattern)
sub_once(f"{stddir}/src/os/fd/raw.rs",
    '#[cfg(target_os = "motor")]\nuse moto_rt::libc;',
    '#[cfg(target_os = "motor")]\nuse moto_rt::libc;\n'
    '#[cfg(target_vendor = "nonos")]\nuse super::owned::OwnedFd;\n'
    '#[cfg(target_vendor = "nonos")]\nuse crate::sys::fd::stdio_raw as libc;',
    'stdio_raw as libc')

# pipes stay unsupported, but the fd trait plumbing must exist for os::fd
sub_once(f"{sysdir}/pipe/unsupported.rs",
    '#[cfg(any(unix, target_os = "hermit", target_os = "wasi"))]\nmod unix_traits {',
    '#[cfg(any(unix, target_os = "hermit", target_os = "wasi", target_vendor = "nonos"))]\nmod unix_traits {',
    'target_vendor = "nonos"))]\nmod unix_traits')

# os::fd nonos seam: an fd-based reactor (the vendored mio backend) registers a
# RawFd for readiness and must resolve it to the net.sockets handle OP_POLL
# takes. Expose the table lookup as a nonos-only free fn on the os::fd surface.
NONOS_FD_SEAM = ('\n#[cfg(target_vendor = "nonos")]\n'
    '/// The NONOS net.sockets handle backing `fd`, or `None` if `fd` is not a\n'
    '/// live socket descriptor. This is the seam a fd-based readiness reactor\n'
    '/// (a vendored `mio` backend) uses to translate a registered `RawFd` into\n'
    '/// the service handle its `OP_POLL` round trip addresses. NONOS-only.\n'
    '#[unstable(feature = "nonos_fd_seam", issue = "none")]\n'
    'pub fn nonos_socket_handle(fd: RawFd) -> Option<u32> {\n'
    '    crate::sys::fd::socket_handle(fd)\n'
    '}\n'
    '\n#[cfg(target_vendor = "nonos")]\n'
    '/// Install an already-open net.sockets `handle` into the descriptor table\n'
    '/// and return the `RawFd` assigned to it. The reactor half of the seam: a\n'
    '/// vendored `mio` backend creates sockets by talking to net.sockets\n'
    '/// directly, then registers the handle here so std socket I/O flows through\n'
    '/// the table. NONOS-only.\n'
    '#[unstable(feature = "nonos_fd_seam", issue = "none")]\n'
    'pub fn nonos_register_socket(handle: u32) -> crate::io::Result<RawFd> {\n'
    '    crate::sys::fd::register_socket(handle)\n'
    '}\n')
sub_once(f"{stddir}/src/os/fd/mod.rs",
    '#[stable(feature = "os_fd", since = "1.66.0")]\npub use raw::*;',
    '#[stable(feature = "os_fd", since = "1.66.0")]\npub use raw::*;\n' + NONOS_FD_SEAM,
    'nonos_socket_handle')
PY

echo "NONOS std PAL applied to $STD"
echo "modules: alloc, io_error, random, stdio, args, fs, pal(time, futex), net(connection), fd(os::fd), thread, thread_local(keys), sync(futex)"
