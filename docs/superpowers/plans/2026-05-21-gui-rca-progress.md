# GUI bring-up — RCA progress note (2026-05-21)

## What we're working on
Get a static NONOS desktop on screen under QEMU (wallpaper + desktop_shell
chrome composited via the virtio-gpu primary surface), per
`2026-05-21-gui-static-desktop.md`. Diagnose-first, bottom-up: make the
`driver_virtio_gpu` capsule observable, fix its bring-up, then walk the
surface handshake up the chain. No input wiring this milestone.

## Committed (on `feature/bootloader-hardening`)
- `7a2c5af0e` fix(userland): compile main's WIP net+crypto capsules (Task 0).
- `8519b3acf` docs(boot-handoff): GUI bring-up baseline + ceremony locality.
- `6818013c9` feat(virtio_gpu): grant **Debug** capability for bring-up tracing.
- `de69c6e2f` fix(make): attach virtio-gpu to `nonos-mk-run-serial` (the serial
  diagnostic target had no GPU device, so the driver always dead-ended at
  "device not found" headlessly).

## Key finding vs ek's model
ek framed this as a **driver-internal** problem (suspects: legacy IRQ, modern
BAR/notify, queue bring-up) gated on the `[gfx.virtio_gpu0] boot` marker.

- The "zero markers" case was real but its cause was **the capsule lacked the
  `Debug` cap (0x100)** — every `mk_debug` was dropped. Granting Debug in both
  the kernel spawn spec (`src/hardware/virtio_gpu_capsule/spawn.rs`) and the
  signed manifest (`userland/capsule_driver_virtio_gpu/Capsule.mk`,
  `0x1F8018 → 0x1F8118`) makes the driver observable.
- With markers visible, the driver runs `boot → find → claim → mmio` and then
  **the whole kernel hangs** — but **not** in driver logic. It's a kernel-side
  fault, so ek's IRQ/BAR/queue suspects are downstream of a stage the driver
  never survives to reach.

## Root cause (debugger-confirmed)
A periodic **timer interrupt that fires while a capsule runs at CPL=3** lands in
the timer ISR (`src/interrupts/isr/timer_trampoline.rs`), which calls
`set_interrupt_context()` (`src/interrupts/safety/context.rs`). That does
`mov %gs:0x0` to read the per-CPU id — but **GS base = 0** at that point, so it
reads unmapped linear address 0 (PML4[0] is cleared by invariant) → **#PF**
(CR2=0, easily misread as "no fault") → the fault handler re-faults on GS too →
silent storm → deadlock. Captured live via the QEMU monitor:
`RIP` pinned at `set_interrupt_context`'s `mov gs:0`, `GS base = 0`, CPL=0,
HLT=0, identical across samples. The gpu capsule is the **trigger** (longest
CPL=3 runner), not the root.

Ruled out with evidence: CPU fault visibility (added `[EXC]` printer at the
exception dispatch — zero exceptions; the storm is a spin), SMP race (SMP=1
reproduces), signals/alarm (no sender fires for a capsule with no alarm),
timer preemption (disabling it still hangs), stack overflow (2 MiB stack),
MMIO/stack VA collision (device maps at 0x80_0000_0000, stack at 0x7fff_…),
and the syscall-handler bodies (`sys_device_list`/`sys_pio_grant`/
`sys_mmio_map`/`sys_mk_debug` are all loop-free and don't write back to user
memory). Two subagents further verified: **all** CPL=3 entry paths `swapgs`
correctly before `iretq`/`sysretq` (incl. first-entry), and `init_unified_vm`'s
`clear_low_half()` zeros only PML4[0] — the kernel half (where `.bss`/
`PERCPU_DATA` live) is untouched.

## Refined picture (2026-05-21, later): TWO distinct, sensitivity-prone failures
With `ensure_gs_base` + `gs_diag` in place, monitor + serial comparison shows:
- **SMP=1**: boot hangs **early — right after `unified-vm`, before the gpu
  spawns** (no gfx ladder). RIP pinned at `set_interrupt_context`'s `mov gs:0`,
  GS base=0 — the GS-storm. (My earlier "gpu-phase GS=0" monitor captures were
  actually this *early* storm, not the gpu.) `ensure_gs_base` does NOT prevent
  it, because its `PERCPU_DATA` scan/BSP-fallback finds no valid `self_ptr`
  (the percpu reload source is itself 0 in that context).
- **SMP=2**: progresses *past* the early point, **reaches the gpu** (`boot`),
  runs `find → claim(?) → find:bar ok`, then **total kernel death** (no
  scheduler) — but with **no GS-storm** (`gs_diag` silent post-init). The gpu
  debug-marker buffers are corrupted (the `[gfx.virtio_gpu0` prefix overwritten
  with zeros, or a user-stack-ptr + incrementing counter), an as-yet
  unexplained user-memory clobber.

Both failures are **highly instrumentation/SMP/layout-sensitive**: adding code
moves the hang point (older build w/o the guards reached `stage:mmio` on SMP=1;
the instrumented build hangs earlier). This sensitivity is why printf-style
debugging keeps shifting the target — the next effective step is **interactive
GDB single-stepping** or a **root fix of the GS-base invariant**, not more
serial markers.

## FIX LANDED (2026-05-21): `362095aee` — GS-independent interrupt cpu_id
There are **two** `cpu_id()`s. `crate::smp::cpu_id()` (`src/smp/cpu.rs:22`) is
already GS-independent (APIC id → `CPU_DESCRIPTORS`), and `percpu::current()`
uses it. But `interrupts::safety::context::cpu_id()` (`context.rs:37`) had its
**own** `mov gs:0` implementation — and that is the one `set_interrupt_context`
calls. Changed it to delegate to `crate::smp::cpu_id()`. Result on SMP=1: the
boot no longer storms after `unified-vm` — it **reaches the gpu**, and the
fault handlers now **print** (`[TRAP PF]`/`[TRAP GP]`) instead of silently
deadlocking. The GS-storm fault class is gone.

Newly *visible* (previously masked) faults to chase next, all ordinary/
debuggable: user `#PF`s in `pid=3` (cr2=0x10 / 0x1000 — near-null derefs) and
one in `pid=8` (gpu, rip≈0x148535d); a kernel `#GP` at `0xffffffff8000ff8c` =
`ipc::nonos_inbox::inbox::Inbox` BTreeMap `remove_kv` (tearing down a crashed
capsule's inbox over a corrupted map). The gpu still stops at `find:bar ok`
with the marker-prefix corruption — now a tractable userland/IPC problem, not a
kernel deadlock.

## Assessment / recommended fix direction
The common root is that the kernel can run a timer ISR with **GS base = 0**
after `init_unified_vm` tears down the low identity map, and `set_interrupt_
context()` hard-depends on `mov gs:0`. Two robust root fixes (kernel/eK domain):
1. **Make per-CPU id GS-independent** so the ISR can't fault on a null GS:
   change `interrupts::safety::context::cpu_id()` (and `crate::smp::cpu_id`,
   used by `percpu::current()`) to derive the index from the APIC id (or, since
   `self_ptr % MAX_CPUS == 0` makes it effectively 0 today, return 0) instead of
   `mov gs:0`. Removes the fault class entirely.
2. **Guarantee a valid kernel GS base across `init_unified_vm`** — re-establish
   `GsBase`/percpu (or ensure the percpu mapping + GS base survive the page-
   table rebuild) so no kernel context ever runs with GS=0.
The `ensure_gs_base` guard added here is necessary-but-insufficient (it can't
reload when its own percpu source reads 0). The SMP=2 `find:bar ok` corruption
needs separate investigation (likely interactive GDB), and may be the same root
expressed differently. For the GUI *milestone* specifically, Approach B
(compositor scans out the bootloader GOP framebuffer) sidesteps both.

## Uncommitted (diagnostic scaffolding — to remove/finalize after the fix)
- `src/smp/percpu/operations.rs` + `mod.rs`: `ensure_gs_base()` (defensive ISR
  guard — the agreed hardening; correct in principle but insufficient alone)
  and `gs_diag()` tracing.
- `src/interrupts/isr/timer_trampoline.rs`: calls the above.
- `src/syscall/microkernel/device.rs` `[DLS]`, `src/arch/x86_64/idt/handlers/
  contract_bridge.rs` `[EXC]`, and gpu-capsule markers in `discover.rs`,
  `setup/mmio.rs`, `setup/sequence.rs`.

## For ek
The `Debug`-cap and serial-target fixes unblock observability and are committed.
The remaining blocker is a **kernel timer-ISR GS-base fault**, not virtio-gpu
setup logic — the driver can't be bisected past `mmio` until that's fixed.
Fix domain is the GS-base / per-CPU invariant (why GS reads 0 in the timer ISR
on the CPL=3 path), which is your area and overlaps the documented
init_unified_vm / identity-map-teardown class of issue.
