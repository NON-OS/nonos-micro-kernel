# ROOT CAUSE (DEFINITIVE) — per-CPU `PCPU_USER_STACK` clobbered across preemption

**Date:** 2026-05-23 · **Branch:** feature/bootloader-hardening · **Status:**
root cause caught at the instruction level and confirmed against `syscall.S`.
**Supersedes** the `2026-05-22-ROOT-CAUSE-frame-collision.md` theory (wrong) and
the IrqPollOut theory in `b3066e1aa` (wrong).

## The bug

`src/arch/x86_64/asm/syscall.S` round-trips the user stack pointer through a
**single per-CPU slot**:

```
syscall_entry_asm:
    swapgs
    mov  gs:PCPU_USER_STACK, rsp     ; (19) save user rsp  -- PER-CPU, one slot
    mov  rsp, gs:PCPU_KERN_STACK
    ... run syscall_handler ...
    mov  rsp, gs:PCPU_USER_STACK     ; (64) restore user rsp  <-- reads the slot
    swapgs ; sysretq
```

`PCPU_USER_STACK` is **per-CPU, not per-process**. When pid 2 (virtio-rng)
calls `mk_yield`, the handler runs `yield_now() = hlt` *inside the syscall*; the
timer preempts it there. While pid 2 is parked, **any other capsule's syscall**
runs `syscall_entry_asm` and overwrites the one slot with *its* user rsp. When
pid 2 is resumed and reaches line 64, it restores **the other process's user
rsp** and `sysretq`s to CPL=3 with the wrong `rsp`.

In the minimal repro the other capsule is **proof-io** (`_start = mk_debug;
mk_exit`), whose rsp is near the top of its (identically-based, `0x00007FFF…`)
user stack. So pid 2's rsp **jumps from deep in its fill loop to ≈ stack top**.

## Evidence (instruction-level, via the `nonos-trap-kstack-writer` facility)

The DR0/DR1 write watchpoint on the eventual return slot
`0x00007ffffffeffe8`, with the `#DB` handler logging on **any CPL** (the prior
handler filtered CPL=0 only, which hid this for the entire investigation):

```
[USER-STACK-WRITER] cpl=3 rip=<_start+6>      val=0x2          # 1st: _start prologue `pushq %r15` (saved r15)
[USER-STACK-WRITER] cpl=3 rip=0x2079c2a0      val=0x2079c299   # 2nd: a `call` (val = rip-7 = pushed RA)
```

The 2nd write is a `call` executed with `rsp = 0x7ffffffefff0` — the **top** of
the stack, inside `_start`'s register-save area. So `rsp` had drifted/jumped up
to the top. `[USTACK=KSTACK-LIVE]` and `[FRAME-OWNER]` never fired (no kernel
frame shares the page), and DR1 only matched once the alias was corrected to
`VMAP_BASE + phys` — together proving the writer is **pid 2 itself at CPL=3**,
not the kernel. The crash is then `ret` → `[0x7ffffffeffe8]` (a leftover small
value, e.g. `0x2`) → CPL=3 instruction-fetch fault → teardown `#GP`.

## Why earlier theories were wrong

- **IrqPollOut leak (b3066e1aa):** kernel `IrqPollOut.seq` is an `AtomicU64`
  counter, never a pointer.
- **Frame collision (2026-05-22):** the kernel-pointer bytes seen in the user
  dump were stale/neighbor content; the RA slot is written by the *capsule*, and
  no live kstack maps the frame. The watchpoints (once correctly aliased + made
  CPL-agnostic) caught a CPL=3 capsule write, not a kernel write.

## The fix (smallest correct change)

`PCPU_USER_STACK` must be preserved **per process** across preemption. Two
acceptable shapes:

1. **Save the user rsp on the per-process kernel stack** in `syscall.S`
   (push it after switching to the kstack; pop it at exit instead of reading
   `gs:PCPU_USER_STACK`). The kernel stack is per-process, so preemption can't
   clobber it. Requires re-balancing the 7-push frame + the `[rsp+…]` arg
   offsets — delicate but self-contained.
2. **Save/restore `PCPU_USER_STACK` with the context** (lower-risk): add
   `pcb.saved_user_stack: AtomicU64`; on preempt/yield save, store the current
   per-cpu `PCPU_USER_STACK` into the PCB; in `resume_kernel_thread` (and the
   other resume paths), restore the per-cpu slot from the PCB before control
   returns through the syscall exit. This makes the per-cpu slot effectively
   per-process without touching the asm round-trip.

Recommended: **option 2** (no asm surgery on the hot syscall path). Add a
`debug_assert!`/regression that a resumed syscall exits with the same user rsp
it entered with.

## FIX IMPLEMENTED (commit on feature/bootloader-hardening)

`fix(sched): preserve per-process user rsp across preemption` — added
`pcb.saved_user_stack`, a `percpu::set_user_stack`/`user_stack` pair, a snapshot
of the per-cpu user rsp into the PCB on the **timer-preempt** (`switch.rs`) and
**cooperative-yield** (`yield_body.rs`) park paths, and a restore in
`resume_kernel_thread` before `ctx.restore()` (guarded `!= 0`). This is option 2
below (no asm surgery on the syscall hot path).

**Gate A: PASS.** The virtio-rng minimal repro, previously a deterministic
crash by ~line 318 within ~70 s, now boots **zero `[TRAP]`** for ~144 s and
continues spawning capsules. (An intermediate version that restored an unsaved
`0` produced `rsp=0`; fixed by also saving on the yield path + the `!= 0`
guard.)

## Verification gates

- **Gate A:** minimal virtio-rng repro, **zero `[TRAP]` for ≥120 s**.
- **Gate B:** `make nonos-mk-desktop-gui-prod` boots to a live compositor +
  wallpaper + desktop_shell (screenshot).

## Cleanup

Revert all `nonos-trap-kstack-writer` facilities (the `debug_watch` module, the
PF dump, the `#DB` log, the `entry.rs`/`user_stack.rs`/`first_entry.rs` probes,
the Cargo feature) before merge; ship only the surgical user-rsp fix + the
regression assertion.
