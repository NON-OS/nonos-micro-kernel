# PROMPT — Investigate & plan the NØNOS cooperative context-switch rewrite

> Paste everything below into a fresh Claude Code session in the
> `nonos-kernel` repo. It is self-contained. Your job is **investigation +
> a detailed plan**, not implementation, unless the plan is approved.

---

## Mission
The NØNOS microkernel's cooperative context-switch is structurally fragile and
is the locus of a memory-corruption bug that crashes capsules and wedges the
kernel (the GUI/desktop is blocked behind it). Produce a **detailed, reviewed
implementation plan** to replace the current setjmp/longjmp + global-BTreeMap
context mechanism with a **standard cooperative context switch** (saved kernel
`rsp` + callee-saved regs + FPU stored *in the PCB*; resume = a normal `ret`
out of a shallow switch frame). Do **not** start editing until the plan is
written and approved.

## Mindset — think like Jon Gjengset
- **Understand the machine before you touch it.** Trace the *exact* byte-level
  flow of one full yield→switch→resume, on paper, for both the CPL=0-preempt
  and CPL=3-preempt cases. If you can't draw the kernel stack at each step,
  you don't understand it yet.
- **Name the invariants.** What must be true about TSS RSP0, the per-cpu
  `kernel_stack_top`, CR3, the FPU owner, and the GS base at every transition?
  Write them down. The bug is an invariant violation; the rewrite must preserve
  every real invariant and delete the fragile ones.
- **No guessing.** Every claim ("this is called from X", "rsp is Y here") is
  backed by reading the code or by an experiment on the repro. Cite
  `file:line`.
- **Enumerate, don't sample.** Find *every* path that enters CPL=3 or restores
  a context, *every* caller of the functions you'll change, *every* reader of
  the BTreeMaps you'll delete.
- **Verify empirically.** There is a deterministic ~30s repro (below). Any
  hypothesis is testable; use it.

## Skills to use (in this order)
1. `superpowers:brainstorming` — explore the rewrite design space *before*
   committing to one. Surface alternatives (e.g. PCB-stored rsp vs a dedicated
   per-cpu switch stack; eager vs lazy FPU; keep `Context` struct vs replace).
   Resolve the hard questions below interactively.
2. `superpowers:writing-plans` — turn the chosen design into the deliverable
   plan (phased, with exact files, the new types/asm, a migration order that
   keeps the tree bootable at each step, a test gate per phase, and rollback).
3. Apply the repo's debugging discipline throughout (hypothesis → experiment on
   the repro → conclusion). Check for any other relevant installed skills with
   the Skill tool and use them if they fit.

## Background — what is known (do not re-derive; verify if you doubt it)
Read `docs/superpowers/plans/2026-05-21-userland-crash-blocker.md` and
`docs/superpowers/plans/2026-05-22-gui-desktop-unblock.md` for the full RCA.
Summary of the established facts:

- **The fragile mechanism.** `Context::save_to` (`src/process/context/full/
  save.rs`) + `Context::restore`/`context_restore_asm` (`.../full/restore.rs`,
  `restore_asm.rs`) implement a setjmp/longjmp keyed off a per-cpu
  `CONTEXT_JUST_RESTORED` flag. Per-process saved context + FPU live in two
  global maps in `src/process/core/suspend.rs`:
  `INTERRUPT_SAVED_CONTEXTS: RwLock<BTreeMap<Pid, Context>>` and
  `INTERRUPT_SAVED_FPU_STATES: RwLock<BTreeMap<Pid, FpuState>>` (FpuState is
  1 KiB), `insert`/`remove`-churned on every switch.
- **The two yield paths both use it:** cooperative `mk_yield` →
  `sys_yield` (`src/syscall/microkernel/process.rs`) → `sched::yield_now` =
  **`hlt`** (`src/nonos_time/core_time.rs`) → preempted by the timer, which
  calls `preempt_current_process` (`src/process/scheduler/preemption/
  switch.rs`); and `perform_yield_inline` (`.../preemption/yield_body.rs`).
- **The three CPL=3 entry/resume paths** (`src/arch/x86_64/context/switch/`):
  `try_first_entry` (first run, iretq), `try_resume` (snapshot →
  `restore_user_context_iretq`), `resume_kernel_thread` (Context →
  `ctx.restore()` → unwind → SYSRET). Dispatched by `switch_to_user_pcb`
  (`dispatch.rs`).
- **The corruption (lldb on the repro):** a capsule preempted in `mk_yield`'s
  `hlt` is resumed via `resume_kernel_thread`; its saved `Context` is **valid**
  (rsp/rip in range — verified), but a **return-chain frame on its frozen
  kernel stack** is overwritten with a kernel-BSS pointer (`0xffffffff821…`)
  while it is parked. The resume unwind then lands `rsp` in the BSS, SYSRET
  pops a garbage rip, and the capsule faults at CPL=3 → teardown `#GP` in the
  inbox `BTreeMap::remove`.
- **Ruled out** (don't repeat): stack overflow (32 KiB makes no difference on
  the minimal repro), `context_restore_asm` logic (correct), the SYSRET `rcx`
  slot itself (intact; phys stable; no write caught on its kernel-VA *or*
  directmap alias), page remap, timer snapshot path (timer fires `from_user=0`),
  frame-allocator fallback (never used), heap/DMA frame collision (all route
  through the one phys bitmap), DeviceRecord ABI, device_list overflow, DMA
  scrub. The slot's phys is non-deterministic per boot.
- **Already fixed/committed:** GS-base ISR deadlock (`362095aee`); kernel-stack
  UAF in `pending_stack_free::drain` (`fcb1d2c21`); missing TSS RSP0 update in
  `resume_kernel_thread` (`460cb7729`).

## The starting design hypothesis (validate or replace it)
A standard switch:
```
switch(prev_rsp_slot: *mut u64, next_rsp: u64):
    push rbx, rbp, r12, r13, r14, r15
    mov  [prev_rsp_slot], rsp
    mov  rsp, next_rsp
    pop  r15, r14, r13, r12, rbp, rbx
    ret
```
- Saved kernel `rsp` lives in the **PCB** (e.g. `pcb.kernel_rsp: AtomicU64`),
  not a global BTreeMap. FPU state likewise moves into the PCB (and consider
  lazy/`TS`-based save).
- Resume = `ret` out of a shallow `switch()` frame; **no longjmp into a frozen
  interrupt chain**, so the "every byte of the parked kstack must survive"
  surface collapses to one frame.
- First entry, user-context resume (iretq for a capsule preempted at CPL=3 with
  a real trap frame), and kernel-thread resume must all be expressible — work
  out how each maps onto the new switch.

## Hard questions the plan MUST answer
1. The current design has **two** resume shapes — `iretq` (a real CPL=3 trap
   frame, `try_resume`) and `ret`-unwind-then-SYSRET (`resume_kernel_thread`).
   Does the rewrite unify them, or keep both? How does a capsule preempted at
   CPL=3 (interrupt frame on its kstack) fit a `ret`-based switch?
2. **First entry:** how is a never-run capsule's `kernel_rsp` initialized so the
   first `switch()` lands it at its entry trampoline with the right user iretq
   frame?
3. **TSS RSP0 / per-cpu `kernel_stack_top`:** when must they be updated, and is
   the switch the single choke point that guarantees it (so the RSP0-asymmetry
   class of bug can't recur)?
4. **FPU:** eager save/restore in the switch, or lazy via CR0.TS? Where does the
   1 KiB state live and who owns it?
5. **SMP:** the design must be correct when SMP goes live (per-cpu current, the
   `CONTEXT_JUST_RESTORED` flag's replacement, locking).
6. **CR3/ASID** switch ordering vs the stack switch (you're switching kstacks;
   when is it safe relative to CR3?).
7. **Signals / `saved_user_context`** and the existing `SUSPENDED_CONTEXTS`
   (suspend/resume API) — what depends on the current Context/maps and must be
   migrated or preserved?
8. Does the rewrite **plausibly** fix the corruption, and how will you *prove*
   it (the writer is still unpinned)? Include a parallel watchpoint task on the
   specific frozen-frame offset, and define the pass/fail (desktop boots, N
   minutes stable, compositor presents, zero `TRAP`).

## Constraints (from `.github/copilot-instructions.md` / CLAUDE.md)
- `#![no_std]`, custom target; must compile under it. Host fast-check:
  `cargo check --lib --features std --target x86_64-apple-darwin` (the
  `panic_handler` error is a known shim artifact — the real gate is the target
  build).
- **75-line max growth per file per change**; split commits. No `//` comments
  inside function bodies. Preserve AGPL headers. Surgical changes; match style.
  Commits `fix(scope): …`, no `Co-Authored-By`, git user `senseix21`.
- The `BootHandoffV1` ABI is hand-synced across crates — don't touch unless
  required.

## Fast deterministic repro (≈30 s, vs ≈130 s for the full desktop)
```
make nonos-mk-driver-virtio-rng-prod
/usr/bin/python3 nonos-utils/sign_kernel.py \
  target/x86_64-nonos/release/nonos-kernel \
  nonos-bootloader/keys/signing_key_v1.bin target/kernel_signed.bin
nonos-bootloader/tools/embed-zk-proof/target/x86_64-apple-darwin/release/embed-zk-proof \
  --input target/kernel_signed.bin --output target/kernel_attested.bin \
  --proving-key nonos-bootloader/tools/nonos-attestation-circuit/generated_keys/attestation_proving_key.bin \
  --seed "nonos-production-attestation-v1-2026"
cp target/kernel_attested.bin target/esp/EFI/nonos/kernel.bin
qemu-system-x86_64 -m 2G -cpu max -smp 1 -machine q35 \
  -drive format=raw,file=fat:rw:target/esp \
  -drive if=pflash,format=raw,readonly=on,file=/usr/local/share/qemu/edk2-x86_64-code.fd \
  -device virtio-vga,disable-modern=on,vectors=0,xres=1024,yres=768 \
  -device virtio-rng-pci -serial file:/tmp/s.log \
  -monitor tcp:127.0.0.1:55557,server,nowait -display none -no-reboot &
# crash signature in /tmp/s.log: pid 2 (proof-io) TRAP PF cpl=3 rip=<garbage>
# then TRAP GP cpl=0 in BTreeMap remove. lldb attaches to -s (port 1234), no KASLR slide.
```
lldb scripts from the prior session: `docs/superpowers/plans/wp-directmap-catch.py`,
`wp-phys-compare.py`.

## Deliverable
A plan doc (via `superpowers:writing-plans`) containing:
- The chosen design + the rejected alternatives and *why*.
- The invariant table (RSP0, kernel_stack_top, CR3, FPU owner, GS) before/after.
- Exact files/types/asm to add/change/delete, with the new `Context`/PCB shape.
- A **migration order** where the kernel boots & passes the repro at each phase
  (e.g. introduce the new switch behind the old, route one path at a time).
- Per-phase test gate (build + minimal repro: zero TRAP, N-min stable).
- Risk/rollback notes; SMP-readiness notes.
- A statement on whether this is expected to fully fix the corruption or only
  shrink its surface, plus the parallel writer-catch task.

Start by invoking `superpowers:brainstorming`. Do not write code first.
