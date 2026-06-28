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
         "\texport OBJCOPY=llvm-objcopy\n"
         "\texport CPPFLAGS=\n"
         "\texport CARGO_TEST=\n"
         "\toverride CARGOFLAGS := -Z build-std=core,alloc,compiler_builtins "
         f"-Z json-target-spec --target={tgtdir}/x86_64-unknown-nonos.json "
         "--no-default-features\n"
         "endif\n")
s = re.sub(r"\nifeq \(\$\(TARGET\),x86_64-unknown-nonos\).*?endif\n", "", s, flags=re.S)
with open(cfgmk, "w") as f:
    f.write(s.rstrip("\n") + "\n" + block)
PY

# 3. reuse linux generic-x86_64 ABI definitions for nonos (idempotent)
python3 - "$RELIBC" <<'PY'
import sys
relibc = sys.argv[1]

def patch(rel, old, new):
    p = f"{relibc}/{rel}"
    with open(p) as f:
        s = f.read()
    if new in s:
        return
    if old not in s:
        raise SystemExit(f"anchor not found in {rel}")
    with open(p, "w") as f:
        f.write(s.replace(old, new, 1))

LX = '#[cfg(target_os = "linux")]'
NX = '#[cfg(any(target_os = "linux", target_os = "nonos"))]'

def broaden(rel, marker):
    patch(rel, f'{LX}\n{marker}', f'{NX}\n{marker}')

for m in ("_paths", "bits_open-flags", "fcntl", "netdb", "signal",
          "sys_mman", "sys_syslog", "termios"):
    patch(f"src/header/{m}/mod.rs",
          f'{LX}\n#[path = "linux.rs"]', f'{NX}\n#[path = "linux.rs"]')

patch("src/header/time/constants.rs",
      f'{LX}\n#[path = "linux.rs"]', f'{NX}\n#[path = "linux.rs"]')
patch("src/header/unistd/sysconf.rs",
      f'{LX}\n#[path = "sysconf/linux.rs"]', f'{NX}\n#[path = "sysconf/linux.rs"]')
patch("src/platform/mod.rs", f'{LX}\npub mod auxv_defs;', f'{NX}\npub mod auxv_defs;')
patch("src/header/sys_epoll/mod.rs",
      f'{LX}\npub const EPOLL_CLOEXEC: c_int = 0x8_0000;',
      f'{NX}\npub const EPOLL_CLOEXEC: c_int = 0x8_0000;')

broaden("src/header/termios/mod.rs",
        '#[repr(C)]\n#[derive(Default, Clone)]\npub struct termios {')
for fn in ("cfgetispeed", "cfgetospeed", "cfsetispeed", "cfsetospeed"):
    broaden("src/header/termios/mod.rs",
            f'#[unsafe(no_mangle)]\npub unsafe extern "C" fn {fn}')

broaden("src/header/grp/mod.rs", "const SEPARATOR: char = ':';")
broaden("src/header/pwd/mod.rs", "const SEPARATOR: u8 = b':';")
broaden("src/header/shadow/mod.rs", "const SEPARATOR: char = ':';")
broaden("src/header/pwd/mod.rs", "mod linux;")
broaden("src/header/pwd/mod.rs", "use self::linux as sys;")
broaden("src/header/limits/mod.rs", "pub const PAGE_SIZE: usize = 4096;")
broaden("src/header/sys_mman/mod.rs", 'static SHM_PATH: &[u8] = b"/dev/shm/";')
broaden("src/header/stdlib/mod.rs",
        '    let r = unsafe { open(c"/dev/ptmx".as_ptr(), flags) };')
broaden("src/header/stdlib/mod.rs",
        '        let name = format!("/dev/pts/{}", pty);')

patch("src/header/signal/mod.rs",
      '#[cfg(not(target_os = "linux"))]\npub struct sigevent {',
      '#[cfg(not(any(target_os = "linux", target_os = "nonos")))]\npub struct sigevent {')
broaden("src/header/signal/mod.rs", "pub struct sigevent {")

patch("src/header/sys_ioctl/mod.rs",
      'use crate::{\n    error::ResultExt,\n'
      '    platform::types::{c_char, c_int, c_ulong, c_ushort, c_void},\n};',
      '#[cfg(not(target_os = "nonos"))]\nuse crate::error::ResultExt;\n'
      'use crate::platform::types::{c_char, c_int, c_ulong, c_ushort, c_void};')
patch("src/header/sys_ioctl/mod.rs",
      '    #[cfg(target_os = "redox")]\n'
      '    unsafe { self::redox::ioctl_inner(fd, request, out) }.or_minus_one_errno()\n}',
      '    #[cfg(target_os = "nonos")]\n    { let _ = (fd, request, out); -1 }\n'
      '    #[cfg(target_os = "redox")]\n'
      '    unsafe { self::redox::ioctl_inner(fd, request, out) }.or_minus_one_errno()\n}')

patch("src/ld_so/tcb.rs",
      '#[cfg(target_os = "linux")]\npub type OsSpecific = ();',
      '#[cfg(any(target_os = "linux", target_os = "nonos"))]\npub type OsSpecific = ();')
patch("src/ld_so/tcb.rs",
      '#[cfg(any(target_os = "linux", target_os = "redox"))]\n    unsafe fn os_new(',
      '#[cfg(any(target_os = "linux", target_os = "redox", target_os = "nonos"))]\n    unsafe fn os_new(')
patch("src/ld_so/tcb.rs",
      '#[cfg(all(target_os = "linux", target_arch = "x86_64"))]',
      '#[cfg(all(target_os = "nonos", target_arch = "x86_64"))]\n'
      '    unsafe fn os_arch_activate(_os: &(), _tls_end: usize, _tls_len: usize) {}\n\n'
      '    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]')

patch("src/ld_so/mod.rs",
      '#[cfg(any(target_os = "linux", target_os = "redox"))]\npub unsafe fn init(',
      '#[cfg(any(target_os = "linux", target_os = "redox", target_os = "nonos"))]\npub unsafe fn init(')
patch("src/ld_so/mod.rs",
      '#[cfg(all(target_os = "linux", target_arch = "x86_64"))]\n    {\n        const ARCH_GET_FS',
      '#[cfg(all(target_os = "nonos", target_arch = "x86_64"))]\n    { tp = 0; }\n'
      '    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]\n    {\n        const ARCH_GET_FS')

patch("src/crt0/src/lib.rs",
      '#[cfg(target_arch = "x86_64")]\nglobal_asm!(',
      '#[cfg(target_os = "nonos")]\nglobal_asm!(\n'
      '    "\n'
      '    .globl _start\n'
      '    .type _start, @function\n'
      '_start:\n'
      '    xor rbp, rbp\n'
      '    and rsp, -16\n'
      '    sub rsp, 8\n'
      '    push 0\n    push 0\n    push 0\n    push 0\n    push 0\n'
      '    mov rdi, rsp\n'
      '    call relibc_crt0\n'
      '    .size _start, . - _start\n'
      '"\n);\n\n'
      '#[cfg(all(target_arch = "x86_64", not(target_os = "nonos")))]\nglobal_asm!(')
PY

echo "NONOS relibc backend grafted into $RELIBC"
echo "  src/platform/nonos/{mod,lowlevel,socket,signal,epoll,ptrace}.rs"
echo "  cfg arm + Sync-errno patch + config.mk TARGET block (override CARGOFLAGS)"
