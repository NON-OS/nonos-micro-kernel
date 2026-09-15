# Syscalls the kernel dispatches that no userland code references

Counted by `scripts/check_syscall_reach.py`. The number may only go down.

A syscall on this list is implemented, sits in the dispatch table, and is
called by nothing in the tree. That is the shape of the bug that keeps
recurring: MkCapsuleVerify, MkAttestDoc and MkFutexWait were all here until
something needed them and found them unwired.

Current set, and why each is still here:

- `SYS_FUTEX_WAKE` (MFTK): reached by the std PAL in the pinned rust-src,
  which lives outside this tree and is not scanned. Not dark.
- `SYS_THREAD_SPAWN` (MTSP), `SYS_SET_TLS` (MSTB): the std PAL's thread
  support is the intended consumer. Check the PAL before treating as dark.
- `SYS_SPAWN` (MSPN): the kernel spawns capsules itself; userland goes
  through the installer and `SYS_SPAWN_INSTANCE`. Dark by design so far.
- `SYS_STDOUT_WRITE` (MSOW): std uses `MkProcOutput`. Dark.
- `SYS_DEV_ROOT_REQUEST`, `SYS_DEV_ROOT_CONFIRM`: device root handoff.
  Dark.

To lower the baseline, wire a syscall and reduce the number in
`syscall-unreachable-count.txt` in the same commit. To add one, say why
here.
