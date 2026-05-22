# Note for eK — GUI/desktop blocker: root cause + plan

**TL;DR.** The desktop is blocked by a capsule **resume corruption**, now traced
to a **wild write into a parked capsule's frozen kernel stack** — a return-chain
frame gets overwritten with a kernel-BSS pointer (`0xffffffff821…`) while the
capsule is halted in `mk_yield`. On resume the unwind lands `rsp` in the BSS,
SYSRET pops a garbage rip, the capsule faults at CPL=3, and teardown `#GP`s in
the inbox `BTreeMap::remove`. **It is NOT** the saved `Context` (validated
sane), **NOT** the SYSRET `rcx` slot (intact, phys stable, no write caught on
its VA or directmap alias), NOT a stack overflow, NOT page remap.

**Already fixed & committed (this branch, `feature/bootloader-hardening`):**
- `362095aee` GS-base ISR deadlock (the original "GUI hang").
- `fcb1d2c21` kernel-stack UAF in `pending_stack_free::drain` (freed the stack
  the timer was running on → killed the crash cascade + kernel `rip=0` wedge).
- `460cb7729` `resume_kernel_thread` was missing the TSS RSP0 update that
  `try_first_entry`/`try_resume` both do (real asymmetry; resumed capsules ran
  with stale RSP0).

**Working & visible:** kernel static desktop renders (NONOS + DESKTOP), and
`[compositor] setup complete` in GOP-fb fallback (Approach B). Only the residual
writer blocks a live, stable compositor desktop.

**Plan (your call welcome):** catch the writer with a **kernel-side hardware
watchpoint** — arm DR0=frame VA and DR1=its directmap alias on the parked
capsule's corrupted frame, log the trapping RIP from the #DB handler. Native
speed (the lldb route is ~7× slower under the gdbstub). Then fix the root.

**Fast deterministic repro (~30 s):** `make nonos-mk-driver-virtio-rng-prod`,
sign+attest+package into `target/esp` (recipe in the prompt below), boot SMP=1
`-smp 1 -device virtio-vga,disable-modern=on,vectors=0 -device virtio-rng-pci`.
Crash: `pid 2` (proof-io) `TRAP PF cpl=3 rip=<garbage>` then `TRAP GP cpl=0` in
`0x8000ff5c` (BTreeMap remove). lldb on `-s` (port 1234), no KASLR slide.

**Suspected your domain:** the cooperative switch (`Context::save_to`/`restore`
setjmp + global `INTERRUPT_SAVED_CONTEXTS`/`INTERRUPT_SAVED_FPU_STATES`
BTreeMaps, churned per switch) is structurally fragile and worth a rewrite to a
standard PCB-stored-rsp switch — but that's **hygiene**, it won't fix this root
(the kstack is frozen+resumed-through either way). Tracked as a follow-on.

Full RCA + scripts: `docs/superpowers/plans/2026-05-21-userland-crash-blocker.md`,
`…/2026-05-22-gui-desktop-unblock.md`,
`…/2026-05-22-ctx-switch-rewrite-PROMPT.md`, `wp-directmap-catch.py`,
`wp-phys-compare.py`.
