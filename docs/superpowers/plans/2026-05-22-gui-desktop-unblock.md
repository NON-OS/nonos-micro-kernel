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

## 2026-05-22 (catch attempt) — it's resume RSP/control-flow corruption, NOT the slot
Built the live-phys directmap watchpoint (script reads proof-io's kstack base
from `rdi` at the `memset` call in `allocate_kernel_stack` @0xffffffff80036abb,
gva2gpa's `base+0x4000-24`, arms `DIRECTMAP_BASE+phys`). It armed correctly
(base=0xffffff502c4e6000, slot=0xffffff502c4e9fe8, live phys e.g. 0x3fbedfe8) —
**but never fired**; the crash did.

Decisive diagnostic (`wp-phys-compare.py`): at the crash,
`phys_pre==phys_post` (no page remap) and **`[slot] = 0x1ca4f2bb` (a valid
proof-io code addr), NOT the crash rip `0x2`**. Per `syscall.S`, SYSRET does
`pop rcx` from `[kstack_top-24]`; that slot is intact, yet `rip(=rcx)=0x2`. So
**SYSRET popped `rcx` from a wrong `rsp`** — the kernel **resume `rsp` /
control-flow is corrupted**, not the rcx data slot. All slot-watchpoints
(kernel-VA and directmap) missed because the slot is never the write target.

**Corrected root target:** what corrupts the resume's `rsp`/return-chain for a
capsule preempted at CPL=0 (in `mk_yield`'s `hlt`) and resumed via
`resume_kernel_thread → Context::restore`. Candidates: the saved `Context.rsp`
in `INTERRUPT_SAVED_CONTEXTS` (heap BTreeMap), or a return-address frame on the
kstack between the `Context` restore point and the syscall-exit pops. The
existing `Context::validate` only checks canonical + `rsp!=0`, so a
canonical-but-wrong `rsp` passes. On the full desktop the same corruption also
hits the **kernel's own** control flow (cpl=0 jump to garbage), so a
SYSRET-side guard alone won't save it — the resume-`rsp` corruptor must be
found/fixed.

**Next catch:** watchpoint the saved `Context` for proof-io (find the
`INTERRUPT_SAVED_CONTEXTS` node, watch its `rsp` field), or single-step the
resume to see where `rsp` diverges from `kstack_top-56`. Scripts:
`wp-directmap-catch.py` (arm helper), `wp-phys-compare.py` (pre/post phys+slot).

## 2026-05-22 (ruled out) — NOT stack overflow; resume path asm is correct
- **Stack overflow ruled out conclusively**: 32 KB `KERNEL_STACK_SIZE` on the
  *minimal* repro (2 capsules, no spawn-memory-pressure) crashes identically
  (pid 2 → rip=0x2 → same `#GP`). The earlier 32/64 KB full-desktop test was
  confounded by an unrelated spawn livelock; the minimal repro removes that.
- **`context_restore_asm` is correct**: it loads `rsp = ctx.rsp` (offset 56),
  pushes rip/rflags/rdi, pops rdi/popfq/ret — leaving `rsp = ctx.rsp`, `rip =
  ctx.rip`. No bug. So the corruption is in the *input data* (`ctx.rsp` /
  `ctx.rip` in `INTERRUPT_SAVED_CONTEXTS`, or a kstack return-chain frame),
  written by an external wild write, not in the restore logic.
- `Context::validate` only checks canonical + `rsp!=0`, so a canonical-but-wrong
  `rsp` (our case) passes — a stricter check (rsp inside the owning kstack, rip
  in kernel text) could *contain* the failure but won't find the writer.

**Status:** root = a wild write corrupting **resume control-flow data** (saved
`Context.rsp` or a kstack frame for a capsule preempted in `mk_yield`'s `hlt`),
which makes SYSRET pop `rcx` from a bad `rsp` → jump to a tiny garbage rip;
on the full desktop it also corrupts the kernel's own return path. Ruled out
this session: the rcx data slot, directmap write to the slot, page remap, stack
overflow, restore-asm logic, timer-snapshot path, frame-allocator fallback,
heap/DMA frame collision, DeviceRecord ABI, device_list overflow, DMA scrub.
**Next:** watch the saved `Context` node (`INTERRUPT_SAVED_CONTEXTS` for pid 2)
or single-step proof-io's crashing resume to see where `rsp` first diverges.

## 2026-05-22 (DECISIVE pivot) — it's USER-memory corruption, NOT the resume
Built a feature-flagged probe: in the syscall-exit asm, assert `rsp ==
kstack_top-24` right before `pop rcx`; on divergence dump the frozen frames.
**It never fired** — so the SYSRET resume rsp is *correct*. The capsule resumes
to a valid rip and only then jumps to garbage **in user mode** (`TRAP PF cpl=3
rip=<garbage> rsp≈user-stack-top`). So the entire kstack / `Context` / resume
investigation (and the `kstack-writer-trap` plan) targeted the **wrong layer**.

What's actually happening:
- The crashing pid (pid 2) is **virtio-rng** (the DMA/IRQ-looping driver that
  resumes 100+ times), **not** proof-io (whose `_start` is just
  `mk_debug; mk_exit` — it can't loop).
- virtio-rng's **user-mode control flow** is corrupted: a return address /
  function pointer in its user memory gets overwritten, and it jumps there.
- The injected values **vary** across runs: `0x2`, `0xff00`, `0x7ffffffffffffffc`
  (the `movabsq` mask from `allocate_kernel_stack`), and pointers **into the
  16 MiB bootstrap heap** (`BOOTSTRAP_HEAP_MEMORY = 0xffffffff82150000`; seen
  `…82157b20`, `…82164b20`, `…82167b20`, `…822e0b20`). So the writer copies
  *varying live data* onto a capsule's user stack, not a fixed value.

Ruled out this round: kernel resume (rsp-probe), and **all syscall out-struct
ABI sizes** (IrqPollOut 16, MmioMapOut 24, DmaMapOut 32, IrqBindOut 16,
DeviceRecord 176 — each `static_assert`-checked, kernel == userland). So it is
**not** a usercopy out-struct overflow.

**Corrected target:** something writes varying live kernel/device data onto a
capsule's **user stack** (return address). Leading hypothesis given pid 2 =
virtio-rng: the **virtio DMA / virtqueue** — the device DMA-writing past the
allocated buffer (a queue-layout/size bug) into the adjacent physical frame that
backs the user stack, OR a kernel path copying a buffer to the wrong user
offset. Next probe: a hw watchpoint (DR0+DR1 VA+directmap) on the **corrupted
user-stack return slot** of pid 2 (found by reading its user stack at the
crash), armed while it's parked — catches the writer. The `kstack-writer-trap`
plan's machinery applies, retargeted from the kstack to the **user stack**.

## Test loop
`make nonos-mk-desktop-gui-prod && make nonos-mk-esp`, boot headless with
`-monitor tcp` + `-serial file`, `screendump` the framebuffer, grep serial for
`TRAP`, `RESUME-BAD`, `compositor`. SMP=1 first (deterministic), then SMP=2.
