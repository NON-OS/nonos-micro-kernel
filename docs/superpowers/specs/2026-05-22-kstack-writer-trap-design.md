# Design: trap the kstack wild-writer (kernel-side hardware watchpoint)

## Problem
A capsule preempted while halted in `mk_yield` is resumed via
`resume_kernel_thread`; its saved `Context` is valid, but a **return-chain frame
on its frozen kernel stack** is overwritten with a kernel-BSS pointer
(`0xffffffff821…`) while it is parked. On resume the unwind lands `rsp` in the
BSS, SYSRET pops a garbage rip → CPL=3 fault → teardown `#GP`. The desktop is
blocked behind this. The save/restore mechanism is *not* the bug (validated);
the bug is **whatever writes that frame**. Goal: name the writer's RIP, then fix
the root.

## Approach
Catch the write with the **CPU debug registers, driven from kernel code** — not
lldb (the gdbstub is ~7× slower and the frame's phys is non-deterministic per
boot, which defeated a hardcoded watchpoint all session). The kernel knows the
parked capsule's frame address at arm time and can also compute its directmap
alias. Everything is gated behind a feature flag so it is fully removable.

## Components

### 1. Locate the corrupted frame (investigation, no committed code)
On the fast minimal repro, break at `page_fault::handle` and walk the crashing
capsule's kernel stack from `kstack_top` downward to find the slot holding the
BSS pointer. Record offset `N` from `kstack_top` (deterministic — the call
depth at the `mk_yield` park point is fixed) and the VA→phys translation.
Output: `N` and confirmation of which alias (kstack VA vs directmap) the write
uses, if determinable.

### 2. `trap_kstack_writer` module (feature-flagged: `nonos-trap-kstack-writer`)
- `arm(frame_va: u64)`: set **DR0 = frame_va** and **DR1 = DIRECTMAP_BASE +
  phys(frame_va)**, configure DR7 for two 8-byte *write* breakpoints, enable.
  Two registers because the write may go through the directmap alias rather than
  the kstack VA (session evidence) — cover both.
- `disarm()`: clear DR7 enables.
- Arm/disarm hooks: in the **park** path (`preempt_current_process` /
  `perform_yield_inline`, save side) arm when `current_pid == TARGET_PID`; in the
  **resume** path disarm so the target's own legitimate kstack writes don't trip
  it. `TARGET_PID` is a const (the deterministic crasher, pid 2) for the focused
  catch — YAGNI, no general API.

### 3. #DB (vector 1) handler
Confirm/extend the existing debug-exception handler to, on entry: read **DR6**
(which DR matched), log the trapping **RIP**, a short kernel backtrace, the
written 8 bytes, and DR6; clear DR6; on the first hit **halt** so the serial log
is intact and readable. (If NONOS has no #DB handler, the plan adds a minimal
one.)

## Data flow
park(target) → `arm(frame_va)` → other contexts run → writer's `mov`/`rep stosb`
to frame_va (or its directmap alias) → #DB → handler logs writer RIP + halts.

## Error handling / safety
- Must not collide with any existing DR usage — the plan audits the tree for
  DR0–DR7 reads/writes first.
- DRs are per-CPU; correct at SMP=1 (the repro). For SMP the arm/disarm must be
  per-cpu — noted as a constraint, not implemented now.
- Arming on the directmap alias requires a reliable VA→phys at arm time
  (`translate`/page-table walk); the frame is mapped (the capsule just yielded).
- Feature-flagged, no effect when the flag is off; removed before merge.

## Testing
Minimal repro (`nonos-mk-driver-virtio-rng-prod`, ~30 s, deterministic). Pass of
the *catch*: serial shows `[KSTACK-WRITER] rip=… val=…` and halts. Pass of the
*fix*: rebuild, repro boots with **zero `TRAP`**, stays stable N minutes, and on
the full desktop the compositor presents past `setup complete`.

## Constraints
`#![no_std]`; 75-line-per-file growth cap (split commits); no `//` in function
bodies; preserve AGPL headers; commits `fix(scope): …`, no `Co-Authored-By`,
git user `senseix21`. Behind `nonos-trap-kstack-writer`; not in any shipping
profile.

## Out of scope (follow-on spec)
The context-switch rewrite (replace setjmp + global BTreeMaps with a standard
PCB-stored-rsp switch). Hygiene; does not fix this root (the kstack is frozen
and resumed-through regardless of the save/restore mechanism).
