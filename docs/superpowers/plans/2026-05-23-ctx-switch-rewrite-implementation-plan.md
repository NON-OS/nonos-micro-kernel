# NØNOS Cooperative Context-Switch Rewrite — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: use superpowers:executing-plans to
> implement this plan phase-by-phase. Each phase ends with a boot gate; do not
> advance until it passes. Steps use `- [ ]` checkboxes.

**Goal:** Replace the fragile setjmp/longjmp + global-`BTreeMap` kernel-thread
context mechanism with a standard `ret`-based context switch whose saved kernel
`rsp` and FPU state live **in the PCB** — fixing the parked-kstack corruption
(the broken voluntary-yield resume that blocks the GUI) and the IPC service
loops that hang on it.

**Architecture:** A single `cpu_switch(prev_rsp_slot, next_rsp)` saves callee-
saved regs + the return address on the *outgoing* kernel stack, stores its `rsp`
in `prev.kernel_rsp`, loads `next.kernel_rsp`, pops, and `ret`s into the *next*
task's resume point. First-entry and CPL=3-resume are unified into this single
switch via **fake initial switch frames** whose return address is a small
trampoline that performs the `iretq`. The per-cpu `CONTEXT_JUST_RESTORED` flag,
`Context::save_to`/`restore`, `context_restore_asm`, and both global maps
(`INTERRUPT_SAVED_CONTEXTS`, `INTERRUPT_SAVED_FPU_STATES`) are deleted.

**Tech stack:** Rust `#![no_std]`, x86_64, custom target; naked asm functions;
`make nonos-mk-driver-virtio-rng-prod` repro; full target build is the real gate.

---

## Why this fixes the bug

Today the voluntary yield (`perform_yield_inline`) and timer preempt
(`preempt_current_process`) save a full `Context` via setjmp (`save_to`), park it
in `INTERRUPT_SAVED_CONTEXTS`, and rely on `resume_kernel_thread` calling
`ctx.restore()` (longjmp) to land back on the `was_just_restored()` check. The
resume **longjmps into a frozen interrupt/return chain on the parked kernel
stack**; any stray write to that frozen chain while the task is parked corrupts
the unwind → SYSRET to garbage → CPL=3 fault → teardown `#GP`. Empirically the
voluntary-yield resume **never returns at all** (instrumented: `pre-yield` fires,
`post-yield` never does), so every recv-poll service loop (compositor included)
hangs at the first yield.

A `ret`-based switch collapses the "every byte of the parked kstack must survive"
surface to **one frame** (callee-saved + one return address). The task resumes by
a normal `ret` out of `cpu_switch`, not a longjmp into a deep frozen chain. This
is the standard mechanism (xv6/Linux `swtch`); it removes the fragile invariant
entirely rather than guarding it.

**Honesty on certainty:** this is expected to *fix* the corruption (it deletes
the corruptible surface), but the stray writer was never positively pinned, so
Phase 5 includes a parallel watchpoint task and a hard pass/fail gate.

---

## Chosen design vs alternatives

**Chosen — unified `cpu_switch` + iretq trampolines.** One switch primitive; every
`next_rsp` points at a kernel-stack frame `cpu_switch` rets into — either a real
parked frame (kernel→kernel) or a *fake* frame whose return address is a
trampoline (`first_entry_trampoline` / `resume_user_trampoline`) that builds the
iretq frame and enters CPL=3. This makes the switch the **single choke point**
for TSS.RSP0 / CR3 / FPU updates (kills the RSP0-asymmetry bug class) and answers
hard-question-1 (the two resume shapes unify).

- *Rejected: keep the iretq dispatch separate, only swap the kernel-thread path.*
  Lower change surface, but the "save prev, then iretq to next, resume prev
  later" composition reintroduces a returns-twice / orphaned-epilogue hazard
  (the same class of bug we're removing). Documented as the rollback-friendly
  fallback if the unified cutover proves too large in one step.
- *Rejected: dedicated per-cpu switch stack.* Unnecessary on `-smp 1`; revisit
  for SMP. PCB-stored `rsp` is simpler and SMP-correct (per-task, not per-cpu).
- *FPU: eager, moved into the PCB* (chosen). Lazy/`CR0.TS` deferred — eager is
  the current proven behavior; moving it into the PCB removes the second map.

---

## Invariant table (must hold at every transition)

| Invariant | Today | After |
| --- | --- | --- |
| **TSS.RSP0** = current task's kernel-stack top | set in each of the 3 resume paths (asymmetry-prone) | set in `cpu_switch`'s resume side / trampoline — one site |
| per-cpu `kernel_stack_top` mirror == TSS.RSP0 | dual-write in 3 paths | dual-write in the one switch site |
| **CR3/ASID** matches the resumed task | set in 3 paths | set on the resume side before iretq/ret-to-user |
| **FPU owner** == running task | eager fxsave/fxrstor via BTreeMap | eager fxsave/fxrstor via `pcb.fpu` |
| **GS base** kernel on CPL=0, user on CPL=3 | swapgs in iretq/sysret asm | unchanged (trampolines keep swapgs) |
| `saved_user_stack` not leaked across tasks | snapshot/restore via PCB | unchanged |
| parked kernel context is resumable | full `Context` + frozen chain (fragile) | `pcb.kernel_rsp` → one switch frame |

---

## New types / shapes

`src/process/core/pcb.rs` (add fields; net growth < 75 lines):
```rust
pub kernel_rsp: AtomicU64,        // saved kernel rsp of a parked switch frame; 0 = none
pub fpu: spin::Mutex<FpuState>,   // eager FPU, was INTERRUPT_SAVED_FPU_STATES[pid]
pub fpu_valid: AtomicBool,        // was has_saved_fpu_state(pid)
```
`Context` (144 B), `CONTEXT_JUST_RESTORED`, `INTERRUPT_SAVED_CONTEXTS`,
`INTERRUPT_SAVED_FPU_STATES` are **deleted** by Phase 4.

New asm (`src/arch/x86_64/context/switch/cpu_switch.rs`, naked):
```
cpu_switch(prev_rsp_slot: *mut u64 /*rdi*/, next_rsp: u64 /*rsi*/):
    push rbp; push rbx; push r12; push r13; push r14; push r15
    mov  [rdi], rsp
    mov  rsp, rsi
    pop  r15; pop r14; pop r13; pop r12; pop rbx; pop rbp
    ret
```
Trampolines (`ret`-target of a fake frame; run with interrupts off, on the new
task's kernel stack): `first_entry_trampoline` reads `pcb.pending_user_entry` →
`return_to_usermode_asm`; `resume_user_trampoline` reads `pcb.saved_user_context`
→ `restore_user_context_iretq`. Both first set TSS.RSP0 / CR3 / FPU for the task.

Fake initial frame builder (`build_initial_switch_frame`): on first-entry setup
and on CPL=3 preempt-snapshot, write onto the task's kernel stack
`[r15..rbp (zeroed)] [retaddr = trampoline]` and set `pcb.kernel_rsp` to it, so a
plain `cpu_switch` into the task lands in the trampoline.

---

## Phases (kernel boots + passes the repro at each gate)

### Phase 1 — FPU into the PCB (independent, low-risk)
**Files:** `pcb.rs` (+`fpu`,`fpu_valid`), `core/table/create.rs` (init), `suspend.rs`
(rewrite `save_fpu_state`/`restore_fpu_state`/`has_saved_fpu_state`/`clear_fpu_state`
to use the PCB; delete `INTERRUPT_SAVED_FPU_STATES`), verify the 3 resume callers
+ 2 save callers compile unchanged (same function names).
- [ ] Add the two PCB fields + init them in `create.rs`.
- [ ] Rewrite the four `*_fpu_state` fns to read/write `pcb.fpu`/`fpu_valid`.
- [ ] Delete `INTERRUPT_SAVED_FPU_STATES` + its (now-unused) imports.
- [ ] **Gate:** target build clean; repro **zero TRAP ≥120 s** (FPU still correct).

### Phase 2 — Introduce the switch primitive (unrouted)
**Files:** new `cpu_switch.rs` (asm + trampolines + `build_initial_switch_frame`),
`pcb.rs` (+`kernel_rsp`), wire the module. Nothing calls `cpu_switch` yet.
- [ ] Write `cpu_switch` naked asm + a unit-style boot self-test (switch between
  two trivial kernel closures behind a `#[cfg(feature=...)]`, off by default).
- [ ] Write the two trampolines + `build_initial_switch_frame`.
- [ ] **Gate:** target build clean; repro unchanged (new code dead).

### Phase 3 — Build fake frames at entry/preempt
**Files:** `setup.rs` (first-entry: also build the fake switch frame + set
`kernel_rsp`), `timer_trampoline.rs`/`switch.rs` (CPL=3 preempt: after writing
`saved_user_context`, build the fake `resume_user_trampoline` frame + set
`kernel_rsp`). Old path still drives resumes; `kernel_rsp` is populated in
parallel and asserted consistent via a temporary debug log.
- [ ] Populate `kernel_rsp` for first-entry and CPL=3 cases; keep old path live.
- [ ] **Gate:** build clean; repro unchanged; debug log confirms `kernel_rsp` set
  for every resumable task.

### Phase 4 — Cut over + delete the old mechanism (the core change)
**Files:** `yield_body.rs`, `switch.rs` (replace the `clear_restored_flag`/
`save_to`/`was_just_restored`/`save_interrupt_context` block with
`cpu_switch(&pcb.kernel_rsp, next.kernel_rsp)`), `kernel_thread.rs` +
`dispatch.rs` (resume is now a plain `cpu_switch`; the trampolines own
iretq/TSS/CR3/FPU), then **delete** `Context::save_to`/`restore`,
`context_restore_asm`, `CONTEXT_JUST_RESTORED`, `INTERRUPT_SAVED_CONTEXTS`, and
`suspend.rs`'s context APIs; migrate the 2 secondary readers
(`process/context/install.rs`, `signal/delivery/syscall_return.rs`) and the
suspend/resume API (`SUSPENDED_CONTEXTS` path) to the new shape.
- [ ] Route yield + preempt + resume through `cpu_switch`.
- [ ] Migrate `install.rs`, `syscall_return.rs`, suspend/resume.
- [ ] Delete the dead setjmp/longjmp + `INTERRUPT_SAVED_CONTEXTS`.
- [ ] **Gate (PRIMARY):** repro **zero TRAP ≥120 s**; then full desktop —
  `[compositor] setup complete` → `tick` → `[BLIT ok]`, compositor presents,
  zero `8000fdac`, ≥3 min stable.

### Phase 5 — Verify the corruption is gone + clean up
- [ ] Parallel writer-catch: re-arm the `nonos-trap-kstack-writer` DR watchpoint
  on the (now one-frame) switch slot for one repro; expect **no** stray write.
- [ ] Revert all debug facilities (`nonos-heap-debug`, `nonos-trap-kstack-writer`,
  `nonos-desktop-lean` if unwanted) per the cleanup list.
- [ ] Screenshot the live compositor/wallpaper.

---

## Hard-question answers (PROMPT §"Hard questions")

1. **Unify the resume shapes** via trampolines (above). 2. **First entry:**
`build_initial_switch_frame` seeds `kernel_rsp` → fake frame → `first_entry_
trampoline` → existing `return_to_usermode_asm`. 3. **TSS.RSP0 / kernel_stack_top:**
updated only on the switch's resume side (single choke point). 4. **FPU:** eager,
in `pcb.fpu`. 5. **SMP:** `kernel_rsp`/`fpu` are per-task (PCB), not per-cpu; the
deleted `CONTEXT_JUST_RESTORED` per-cpu array was an SMP hazard — removing it
helps. `cpu_switch` is per-cpu-safe (operates on the running cpu's stack).
6. **CR3 ordering:** switch CR3 on the resume side *before* the trampoline iretqs
to user (kernel half is shared/global so the stack switch itself is CR3-agnostic).
7. **Signals / suspend:** migrate `saved_user_context` stays as-is; the
`SUSPENDED_CONTEXTS`/`save_interrupt_context` secondary uses move to the new
`kernel_rsp` model or are retired. 8. **Proof:** Phase 5 watchpoint + the
zero-TRAP / present gate.

## Risk / rollback
- Phases 1–3 are additive/independent — revertable individually.
- Phase 4 is the atomic cutover (highest risk). Rollback = revert the Phase-4
  commit; Phases 1–3 remain valid. Keep the fallback design (separate iretq
  dispatch) documented if the unified cutover regresses.
- Each phase is its own commit; ≤75-line growth rule may force sub-commits in
  Phase 4 (yield/preempt vs resume vs deletions).
