# NONOS std platform layer

This directory holds the `std::sys` platform modules that make unmodified
`use std::...` Rust code build and run on NONOS. `apply.sh` copies them into
the pinned `rust-src` and patches the `cfg_select` selectors, keyed on
`target_vendor = "nonos"` so capsules that build only core+alloc are
unaffected. Build a std program with `toolchain/nonos-pack.sh <crate-dir>`
or package a crates.io app with `toolchain/nonos-package.sh`.

Two gotchas the scripts already handle, worth knowing when debugging:
cargo does not reliably notice edits to the sysroot std sources, so after
changing a PAL file run `cargo clean` in the app crate before rebuilding;
and a broken Homebrew python3 exits 0 without running anything, so apply.sh
prefers `/usr/bin/python3`.

## Status: real vs unsupported

Everything listed as real is backed by a syscall or an IPC service and is
exercised by `userland/capsule_std_proof` in the boot log. Everything else
returns `Unsupported` loudly; nothing pretends.

| Surface | State | Backing |
|---|---|---|
| heap (`alloc`) | real | dlmalloc over `MMAP`, spin-locked (thread-safe) |
| `println!` / stdout / stderr | real | `MDBG` serial sink, mirrored to `proc.<pid>` inbox |
| stdin | real | blocking read of this process's kernel stdin channel (`MSRD`), fed by a launcher (the terminal); no EOF-on-close yet |
| `args` | real | `MKAR` |
| env vars | real, process-local | in-process map; nothing is inherited across spawns yet |
| `current_dir` | fixed `/` | capsules see the VFS from its root; `chdir` unsupported |
| `temp_dir` | real | `/tmp` seeded in the VFS store |
| time (`Instant`, `SystemTime`) | real, ms resolution | `SystemTime` is real wall time (`MTMS`, RTC-seeded at boot, NTP-corrected); `Instant` is the monotonic clock (`MMON`, TSC base, no NTP) so it never runs backwards |
| random | real | kernel entropy syscall |
| fs open/read/write/close | real | `vfs_pool` IPC (`OP_OPEN/READ/WRITE/CLOSE`) |
| fs seek/tell | real | `OP_SEEK` (added with this layer) |
| fs stat/metadata, `File::metadata` | real | `OP_STAT` (by path; open files re-stat their path) |
| fs mkdir/rmdir/unlink/rename/truncate | real | `OP_MKDIR/RMDIR/UNLINK/RENAME/TRUNCATE` |
| fs readdir | real | `OP_LIST` |
| fs errors | mapped | VFS errnos become `ErrorKind` (NotFound, AlreadyExists, ...) |
| `create_new` (O_EXCL) | approximated | stat-then-create; not atomic in the store |
| symlink/hardlink/canonicalize/times/locks | unsupported | the VFS does not model them |
| `remove_dir_all`, `File::duplicate` | unsupported | no recursive op / no by-fd dup in the protocol |
| threads: spawn/join/sleep/yield | real | `MTSP` spawn, `MPAL`+`MYLD` polling join, `MTMS` sleep |
| thread-local (`thread_local!`) | real | per-thread key table, address carried in fs base via `MSTB`, restored by the scheduler per task |
| TLS destructors | real | dtor list run at thread exit / runtime cleanup |
| `Mutex`/`RwLock`/`Condvar`/`Once`/parking | real | std futex backends over the kernel wait queue (`MFTW`/`MFTK`): a waiter sleeps and a waker wakes it directly, so contention no longer spins a core. Each wait is capped so a raced wakeup self-heals |
| mpsc channels | real | thread parking above |
| `available_parallelism` | 1 | no core-count syscall yet; honest lower bound |
| thread names | no-op | nowhere to put them yet |
| detached thread stacks | leaked | join frees; without join nobody learns when the task dies |
| net `TcpStream` connect/read/write | real | `net.sockets` IPC through the userland stack |
| net `TcpListener`, `UdpSocket` core | real | same service; IPv4 only |
| DNS (`ToSocketAddrs` by name) | real | `net.dns` |
| socket options/timeouts/peek/nonblocking | mostly no-op | the sockets protocol does not carry them yet |
| IPv6, multicast | unsupported | userland stack is IPv4 |
| `process::Command` (spawn subprocesses) | unsupported | capsule spawn is the installer's job, by design |
| stack overflow guard pages | none | thread stacks are heap allocations |
| unwinding | none | `panic=abort` everywhere |

## Kernel pieces this layer leans on

- `MSTB` (MkSetTls): validates the base as readable user memory, stores it
  in the PCB, writes MSR_FS_BASE. The x86_64 scheduler restores the PCB
  value on every switch into a task, which both carries per-thread TLS and
  scrubs the previous capsule's base. aarch64/riscv64 return failure until
  their trap frames carry the TLS slot; nothing fakes success there.
- `MTSP` (MkThreadSpawn): a new schedulable task sharing the caller's
  address space (CR3 fallback in the switch paths) and capability set.
  Thread teardown is pid-scoped and frees none of the shared space.

## Keyboard input (companion work)

Shift, caps lock and national layouts resolve inside the keyboard drivers
through the shared `userland/nonos_keymap` crate (US, UK, DE QWERTZ,
FR AZERTY, ES, IT), so the PS/2 and USB HID paths agree on every rule.
Ctrl+Alt+Space cycles the layout at runtime. Known gaps: the ISO 102nd
key (PS/2 0x56 / HID 0x64) is unmapped on both paths, dead keys arrive
as plain characters (no composition), and caps lock does not capitalize
non-ASCII letters.

## Proofs

- Host: `userland/fs_proofs` includes the real VFS store source and covers
  the seek semantics (`cargo test`, 84 green).
- Boot: `capsule_std_proof` prints one PASS/FAIL line per subsystem
  (crates.io code, threads+channels+mutex, env, fs incl. seek, TCP socket
  against a host responder) and is spawned through the signed, attested
  capsule path.
