# ROOT CAUSE — user-stack ⟂ kernel-stack physical-frame collision

**Date:** 2026-05-22 · **Branch:** feature/bootloader-hardening · **Status:** root
cause decisively localized; fix candidate identified; confirming probe specified.

## The finding (corrects the prior IrqPollOut theory)

The pid-2 (virtio-rng) CPL=3 crash is **not** an `IrqPollOut` leak. The kernel's
`sys_irq_poll` writes `IrqPollOut{ seq, overflow }` where `seq` comes from
`slots::read_counters` → an `AtomicU64` counter (`slot.seq.fetch_add(1)`), a
small integer — **never a pointer**. So commit `b3066e1aa`'s "leaked kernel
pointer in seq" interpretation is wrong (theory lost to the code).

**Actual root cause:** pid 2's **user stack** and a **live kernel stack** are
backed by the **same physical frame** (`0x7de83000` in the captured run). The
kernel writing that frame *as a kernel stack* (at interrupt time) overwrites
pid 2's user-stack return slot. pid 2 then `ret`s to the clobbered value
(`0x2`, kernel-stack/heap bytes, …) → CPL=3 instruction-fetch fault → teardown
`#GP`.

## Evidence (from the `nonos-trap-kstack-writer` probes, raw)

```
[WATCH-ARM] va=0x7ffffffeffe8 alias=0x7ffffffeffe8 dr7=0x90401 slot_at_arm=0x0
[TRAP PF]   cpl=3 rip=0x2 rsp=0x7ffffffefff0 ... pid=0x2 err=0x15 cr2=0x2
[USER-DUMP] 0x7ffffffeffe0 = 0xffff80007de83740     <- directmap ptr → phys 0x7de83740
[USER-DUMP] 0x7ffffffeffe8 = 0x2                    <- the RA slot → ret to 0x2
[USER-DUMP] 0x7ffffffefff0 = 0xffff80007de836f0     <- directmap ptr → phys 0x7de836f0
[TRAP GP]   cpl=0 rip=0xffffffff8000fdac ... pid=0x2 (inbox BTreeMap teardown)
```

Decoding:
- `0xffff8000…` = directmap; `DIRECTMAP_BASE=0xffff800000000000`. The values are
  pointers into **phys `0x7de83…`** — a kernel stack (pid 1's stack was
  `0xffff80007de83d60`).
- The two pointers are a **self-consistent rbp frame-chain** (`…740`→`…6f0`,
  0x50 apart) sitting near the **top of a 4 KiB page** (offsets `0xfe0`,`0xff0`)
  → a **shallow kernel stack** (idle/boot context; `[TICK] from_user=0 pid=0x0`
  confirms a pid-0 kernel context runs).
- Reading pid 2's *user* VA returns this **live kernel** content ⟹ same frame.
- `[WATCH-ARM] dr7=0x90401` (DR0 only; `virt_to_phys` returned None at arm time
  so DR1/the directmap alias was never armed) → the watchpoint on the **user
  VA** could not see a write done via the **kstack/directmap VA**, and **no
  `[SLOT CHG]`** means the write is **not in a pid-2 syscall** — both are exactly
  what a shared-frame, interrupt-time kernel-stack write produces.

## Mechanism

`allocate_kernel_stack` (kernel stacks) → `memory::page_allocator` →
`frame_alloc::allocate_frame`.
`allocate_user_stack` (user stacks) → `frame_alloc::allocate_frame`.
**Same pool.** The phys bitmap allocator (`phys/allocator/alloc.rs`) is correct
(tests a free bit, sets it before returning), so it cannot hand the same frame
to two live callers — therefore the colliding frame was **freed while still
live** and then reallocated. The deferred kstack-free path
(`pending_stack_free::drain`, the file fixed in `fcb1d2c21`) only guards the
**current `rsp`**; it does **not** guard a kstack that is freed while still
referenced by another live/parked context (e.g. a failed-spawn teardown in the
minimal repro, where most capsules fail cert-decode and are torn down).

Two candidate sub-mechanisms, to be disambiguated by the probe below:
1. **Free-while-live (UAF):** a torn-down capsule's kstack is freed while a
   context (idle/scheduler/parked) is still on it; the frame is reused for
   pid 2's user stack.
2. **Dual-path allocator inconsistency:** `frame_alloc::alloc` tries
   `phys::alloc` (bitmap A) then falls back to the `usable` ranges (bump B),
   but `frame_alloc::dealloc` *always* frees via `phys::free` (A). A frame
   handed out by B but freed into A is then re-allocatable by A → double issue.
   Only matters if B is ever reached (A uninit/exhausted).

## Confirming probe (run next — names the colliding owner)

In `allocate_user_stack` (`src/kernel_core/process_spawn/user_stack.rs`), gated
by `nonos-trap-kstack-writer`, right after `frame_alloc::allocate_frame()`
returns `frame`, scan the process table for a kstack that contains `frame`'s
phys and log it:

```rust
#[cfg(feature = "nonos-trap-kstack-writer")]
{
    let fp = frame.start_address().as_u64();
    for p in crate::process::core::PROCESS_TABLE.iter_pcbs() {
        let top = p.kernel_stack_top.load(core::sync::atomic::Ordering::Acquire);
        if top == 0 { continue; }
        let kphys_top = top.wrapping_sub(crate::memory::layout::constants::DIRECTMAP_BASE);
        let kphys_base = kphys_top.wrapping_sub(KERNEL_STACK_SIZE as u64);
        if fp >= kphys_base && fp < kphys_top {
            crate::sys::serial::print(b"[USTACK=KSTACK] userpid=");
            crate::arch::x86_64::diag::print_hex_u64(pid as u64);
            crate::sys::serial::print(b" frame=");
            crate::arch::x86_64::diag::print_hex_u64(fp);
            crate::sys::serial::print(b" kstack_owner=");
            crate::arch::x86_64::diag::print_hex_u64(p.pid as u64);
            crate::sys::serial::println(b"");
        }
    }
}
```

- If it logs a **live** `kstack_owner` (esp. a still-running pid) → UAF
  (sub-mechanism 1); fix the teardown/`defer_release` to not free a kstack with
  a live/parked reference, or to validate the frame is unmapped from all ASIDs
  before reuse.
- If `kstack_owner` is a **torn-down/exited** pid whose kstack was freed → same
  UAF, fix the ordering so the frame is only freed once nothing can run on it.
- Adapt `iter_pcbs`/field accessors to the real `PROCESS_TABLE` API.

## Fix direction (smallest correct change)

Make a kernel-stack frame **un-reusable until provably dead**: the deferred-free
must not release a kstack while any saved/parked context (`INTERRUPT_SAVED_
CONTEXTS`, a pending resume, or TSS RSP0 / per-cpu `kernel_stack_top`) still
references it — not merely while the *current* `rsp` is inside it. Mirror the
`fcb1d2c21` guard but widen the liveness check beyond the current stack.

## Verification gates (Phase 5)

- **Gate A:** minimal virtio-rng repro, **zero `[TRAP]` for ≥120 s**.
- **Gate B:** `make nonos-mk-desktop-gui-prod` boots to a live compositor +
  wallpaper + desktop_shell; screenshot attached.

## Cleanup

Revert all `nonos-trap-kstack-writer` facilities (probe, DR watchpoint, dump,
slot-probe) before merge; ship only the surgical teardown/allocator fix +
regression assertion.
