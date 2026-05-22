# Plan: get the GUI desktop + wallpaper working (2026-05-22)

## Goal
A stable NØNOS desktop under QEMU: the kernel boots, the compositor presents
its frames to the GOP framebuffer, and the system does **not** hang when a
capsule hits the residual kernel-heap corruption.

## What already works (verified)
- Kernel static desktop renders to the GOP fb (NONOS title + DESKTOP dock).
- `[compositor] setup complete` in GOP-fb fallback (Approach B); wm + desktop_shell boot.
- Kernel-stack UAF fixed (commit fcb1d2c21): the crash *cascade* and kernel
  `rip=0` wedge are gone on SMP=1 and SMP=2.

## Root of the remaining hang (pinned)
A second kernel-heap zeroing corruptor zeros a preempted capsule's
`saved_user_context.rip` (in its PCB) between the timer snapshot and resume.
The preempt-resume path `arch/x86_64/context/switch/resume.rs::try_resume`
iretqs `saved` **without validating rip/rsp** (unlike `context/full/restore.rs`,
which checks `is_canonical`/`is_user_space_addr`). So it iretqs into `rip=0` →
user `#PF` → teardown → kernel `#GP` in inbox `BTreeMap::remove` → guest hangs.

## Strategy (first-principles)
Don't chase the dynamic-heap corruptor with watchpoints. Instrument the **fixed
choke point** every preempted capsule passes through — `try_resume` — to:
1. **Survive** the corruption (park a capsule with a bad snapshot instead of
   iretq-ing into a fault). This unblocks the desktop *now*.
2. **Diagnose** the root (log pid + rip/rsp at the moment of the bad resume).

## Steps
- [ ] **A. Guard `try_resume`.** Validate `saved` (cs RPL=3; rip/rsp non-zero,
  user-canonical ≤ 0x0000_7FFF_FFFF_FFFF). If bad: log `[RESUME-BAD pid rip rsp]`
  and set state `Stopped` (scheduler skips it, no teardown) instead of iretq.
- [ ] **A2. Build + boot.** Confirm: no kernel `#GP`, guest stays alive, serial
  keeps advancing past the first `[RESUME-BAD]`.
- [ ] **B. Verify the compositor presents.** Look for frame/present/blit activity
  after `setup complete`; screendump and confirm compositor content (not just the
  kernel static desktop) reaches the GOP fb. Debug the present loop if not.
- [ ] **C. Root-cause the corruptor** using `[RESUME-BAD]` (which pid, when) →
  instrument writes to that PCB's `saved_user_context` → name + fix the writer.
- [ ] **D. Polish.** Remove debug noise (`[TICK]`, `[SCHED]`, `[RESUME-BAD]`),
  re-verify a clean, smooth desktop boot.

## Findings (2026-05-22 deep session)
The second corruptor is a **relentless kernel-heap + kernel-stack wild write**:
with a fault-park experiment it corrupts capsule after capsule (resume rips →
`0`/`0x1`/`0x100000000`/`0xff00`) and finally the kernel itself (jump to a
garbage rip). On SMP=1 it deterministically takes virtio-rng (pid 6, the only
real device). The damaged data is the cooperatively-yielded capsule's resume
context: capsules `mk_yield → hlt` in the kernel (so the timer always fires
`from_user=0`; the preempt **snapshot path never runs**), are preempted by the
timer, saved as kernel `Context` in `INTERRUPT_SAVED_CONTEXTS`, and resume via
SYSRET to `rcx` on their **kernel stack** — corrupt that and the rip goes bad.

**Ruled out by inspection or experiment this session (do not re-do):**
- Kernel-stack UAF in `pending_stack_free::drain` — FIXED (fcb1d2c21).
- Timer snapshot write — path never runs (`from_user=0` always, all 32 `[TICK]`).
- `Context::save_to` (writes exactly 144 B == sizeof(Context)) — no overrun.
- `FpuState` fxsave — buffer is 1024 B, `align(64)` — no overrun.
- `frame_alloc` `usable`-range fallback — `[FRAME-FB]` never printed; bitmap always used.
- Heap / DMA / kernel-stack frames — **all** route through `frame_alloc → phys`
  bitmap (`page_allocator`, buddy, heap all do); fully coordinated, disjoint.
- `find_contiguous_free` / `set_bit_range` — correct, no overlap.
- `DeviceRecord`/`Bar` ABI (byte-identical), `sys_device_list` (caps at count),
  DMA `scrub`/`zero_run` (length page-aligned).
- try_resume rip guard — added; never fires (snapshot path dead). Reverted.
- Park-on-fault (don't teardown) — kernel survives longer but corruption is too
  widespread; still dies. Reverted.
- KERNEL_STACK_SIZE 16→32/64 KB — **inconclusive**: bigger stacks reproducibly
  livelock during spawn (100% spin) *before* the crash point, so TRAP=0 there
  proves nothing. Reverted to 16 KB. (Why bigger stacks livelock spawn is itself
  an open thread worth pulling.)

**Best remaining probe:** interactive lldb hardware **write-watchpoint** on a
live capsule's kernel-stack syscall-frame slot (the saved `rcx`) or on an
`INTERRUPT_SAVED_CONTEXTS` node, captured just after a `mk_yield`, then continue
to catch the writer's RIP. The free-running boot reaches the crash in ~110 s but
is much slower under the lldb stub; a minimal/faster repro would help. Likely the
context/scheduler machinery or an IRQ-delivery path — eK's domain.

## 2026-05-22 (later) — minimal repro + slot pinned + directmap-alias mechanism
**Minimal repro (huge speedup, ~30s vs ~130s, deterministic crash):** build
`make nonos-mk-driver-virtio-rng-prod`, then manually sign/attest/package
(ESP target hardwires desktop-gui, so bypass it):
```
make nonos-mk-driver-virtio-rng-prod
/usr/bin/python3 nonos-utils/sign_kernel.py target/x86_64-nonos/release/nonos-kernel \
    nonos-bootloader/keys/signing_key_v1.bin target/kernel_signed.bin
nonos-bootloader/tools/embed-zk-proof/target/x86_64-apple-darwin/release/embed-zk-proof \
    --input target/kernel_signed.bin --output target/kernel_attested.bin \
    --proving-key nonos-bootloader/tools/nonos-attestation-circuit/generated_keys/attestation_proving_key.bin \
    --seed "nonos-production-attestation-v1-2026"
cp target/kernel_attested.bin target/esp/EFI/nonos/kernel.bin
```
Boots, then **pid 2 (proof-io) crashes to a corrupted rip** (0x2 / 0xff00 /
varies) → same kernel `#GP` in inbox `BTreeMap::remove`. Reproduces every run.

**Exact corrupted datum (lldb on the minimal repro):** proof-io resumes from
`mk_yield` via `sysretq` to `rcx` popped from its **kernel stack at
`kernel_stack_top − 24`** (the 3rd push in `syscall.S` entry). That slot is what
holds the corrupt value (confirmed: `[top−24]` = the bad rip; SYSRET puts it in
rip).

**KEY MECHANISM — the corruptor writes via the DIRECTMAP, not the kernel VA:**
- A hw write-watchpoint on the slot's kernel VA (`kernel_stack_top−24`) caught
  only the *legit* `memset` zeroing it at `allocate_kernel_stack`, **never** the
  `0x2` write — yet the slot becomes `0x2`.
- So the corrupting write hits the slot's **physical frame via its directmap
  alias** (`DIRECTMAP_BASE + phys`), invisible to a linear-VA watchpoint.
- The slot's **phys is non-deterministic per boot** (saw `0x3fbedfe8`,
  `0x6e3c1fe8`) — KASLR/alloc order — so the directmap alias must be computed
  live each run (a hardcoded one missed).

**Conclusion:** proof-io's kernel-stack frame is being written through the
directmap by some kernel path that treats that frame as something else — a
frame collision / reuse (despite all allocators routing through the one phys
bitmap, there is a directmap writer hitting a live kstack frame). The corruptor
zeros/garbages pointers (rip→0/0x2/0x100000000, BTreeMap node→0x18).

**Remaining step (well-scoped):** `docs/superpowers/plans/wp-directmap-catch.py`
arms a directmap write-watchpoint on proof-io's slot using a run-live phys (via
QEMU monitor `gva2gpa`), triggered off `spawn_proof_io_capsule`. Still TODO:
read proof-io's *actual* `kernel_stack_top` for the run (the value at the
`spawn_verified` alloc-return site wasn't in rax/rdx as expected — read it from
`pcb.kernel_stack_top` or PERCPU_DATA+32 once proof-io is current instead), and
resolve `WatchAddress` returning None on the directmap address. Once armed, the
watchpoint's backtrace names the directmap writer = the root corruptor.

## Test loop
`make nonos-mk-desktop-gui-prod && make nonos-mk-esp`, boot headless with
`-monitor tcp` + `-serial file`, `screendump` the framebuffer, grep serial for
`TRAP`, `RESUME-BAD`, `compositor`. SMP=1 first (deterministic), then SMP=2.
