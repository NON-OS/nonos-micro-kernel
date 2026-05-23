# Architecture: the bootstrap→paged-heap transition is dead-coded

**Date:** 2026-05-23 · **Context:** stepping back from the teardown-#GP symptom
to the heap/teardown architecture (the recurring desktop-load fragility).

## Structural root cause

The kernel has two heap init paths over one global allocator
(`KERNEL_HEAP: SecureHeapAllocator { inner: linked_list_allocator::LockedHeap }`):

- `heap::manager::init_bootstrap()` (boot, `boot/main/core_init.rs:42`) →
  `KERNEL_HEAP.init(BOOTSTRAP_HEAP_MEMORY, 16 MiB)` over a **static `.bss`
  array**. Sets `initialized = true`.
- `heap::manager::init()` (`memory/unified/system.rs`, after `frame_alloc`) is
  meant to allocate the **256 MiB** paged main heap (`KHEAP_BASE`,
  `KHEAP_SIZE = 0x1000_0000`), map it, re-point `KERNEL_HEAP`, and clear
  `USING_BOOTSTRAP`.

**The bug:** `init()` opens with `if KERNEL_HEAP.is_initialized() { return Ok(()) }`
— and `init_bootstrap` already set `initialized = true`. So `init()` **always
early-returns**, the 256 MiB main heap is never created, `USING_BOOTSTRAP` is
never cleared, and **the entire system runs forever on the 16 MiB bootstrap
heap** — which `constants.rs` explicitly sized only for "early static state, a
couple of concurrent AEAD round-trips, and the ELF loader scratch" (~3-4 MiB
transient), *not* a ~25-capsule desktop.

## Why this is the recurring desktop-load fragility

Under the full desktop fleet (25 capsules: PCBs incl. 8 KiB io_bitmap each,
inboxes, IPC bounce buffers, the inbox registry `BTreeMap`, services, caps,
scheduler state, crypto transients) the 16 MiB heap is pressured/fragmented.
That is the most likely source of the **teardown #GP**: the inbox-registry
`BTreeMap<String, Arc<Inbox>>` node pointers are corrupted, so
`unregister_for_pid → BTreeMap::remove` `#GP`s when a capsule dies — and it is
**desktop-load-specific** (clean in the 2-capsule minimal repro). The earlier
"bootstrap-heap pointer" corruption values seen on user stacks are consistent
with the registry/PCBs living in this single 16 MiB region.

## Fix-design analysis (both naive fixes are risky — by design)

1. **Enlarge `BOOTSTRAP_HEAP_SIZE`.** Simple, but `constants.rs` warns it is
   bounded so as **not to stretch the bootloader's mapping/signature-verify
   window**. The bootstrap heap is a static `.bss` array (NOBITS, placed last);
   growing it grows `p_memsz`, which the **separate UEFI bootloader** must map.
   Requires verifying/extending the bootloader's kernel-segment mapping —
   a cross-crate (hand-synced ABI) change. **Test empirically before trusting.**
2. **Make the transition work.** The intended design. Blocker:
   `LockedHeap`/`linked_list_allocator::Heap` is single-region and single-`init`
   — calling `init` twice (re-point to the 256 MiB region) is unsafe, and the
   bootstrap and main regions are non-contiguous so `extend` cannot bridge them.
   A correct transition needs either (a) swapping the global allocator's inner
   to a freshly-constructed `Heap` over the main region — orphaning pre-
   transition bootstrap allocations (safe only if none are freed afterward, or
   if freed-bootstrap blocks rejoining the free list is proven harmless), or
   (b) a multi-region allocator. Both are deliberate allocator changes needing
   careful boot + teardown verification.

## Recommendation

Pursue **(2a)**: at `init()`, gate on `!USING_BOOTSTRAP` (not
`is_initialized()`), construct a fresh `Heap` over the mapped 256 MiB main
region, swap it into `KERNEL_HEAP.inner`, and clear `USING_BOOTSTRAP` — *after*
auditing that allocations made between `init_bootstrap` and `init()`
(`phys::init` bitmap [leaked], `frame_alloc` ranges, the main-heap frame Vec)
are either permanent or safe to free post-swap. Verify: full-desktop boots,
teardown #GP gone, `get_heap_stats().free_memory()` shows the 256 MiB region in
use. Keep the bootstrap heap at 16 MiB (do not stretch the bootloader window).

This is a careful, testable change, not a one-liner — staged separately from the
two committed corruption fixes so it can be reviewed and boot-verified on its
own.

## FIX IMPLEMENTED + VERIFIED (`857baf664`)

Gated `heap::init()` on `!USING_BOOTSTRAP` instead of `is_initialized()`, so the
previously-dead 256 MiB paged-heap transition runs. The existing transition code
(allocate frames → map at `KHEAP_BASE` → re-init `KERNEL_HEAP` → clear
`USING_BOOTSTRAP`) was correct; only the guard was wrong. Bootstrap-window
allocations are orphaned in the static (safe: re-init builds a fresh hole list
over the main region; freed orphans rejoin as isolated holes).

- **Minimal repro:** boots zero TRAP with the 256 MiB heap (no "Heap init
  failed", no frame exhaustion).
- **Full desktop:** **teardown #GP eliminated — 0 occurrences** of `8000fdac`/
  `TRAP GP` (was the wedge). `[compositor] setup complete` + `[desktop_shell]
  boot` now appear and the system survives; the only residual traps are 2
  *different* app capsules (pid 24, 32) faulting once each — contained (no loop,
  no teardown GP). The recurring desktop-load fragility is resolved.

Also `7c7d5f374`: removed the kernel-side static NONOS/DESKTOP placeholder text
(screen now shows the textless background/bars/dock).

## Compositor present path: correct, but blocked by an intermittent teardown #GP

Investigated the GUI present path (user request). The path is **correct**:
`frame_pacer::tick` composites into `ctx.backing_va` and, in GOP-fb mode
(`gfx_port==0`), calls `nonos_surface_present_full(0, backing_va)` → kernel
`graphics_present::blit` (validated; `surface_span_for_id` accepts the mmap VA;
copies the backing into the GOP framebuffer via the directmap + `wbinvd`). The
runner loop calls `tick` every iteration with `drain_ipc` non-blocking, and
setup `mark_full`s damage so the first tick should present.

**But the compositor never presents** (instrumented `tick`: zero `present:`
markers). The serial shows why: right after `[compositor] setup complete`, app
capsules fault (their own startup bugs — jumps to low garbage rips like
`0x1474`) and **one teardown #GPs the kernel** (`8000fdac`, inbox-registry
`BTreeMap::remove`) **before the compositor is scheduled to run its render
loop**, wedging the system.

**The teardown #GP is intermittent even with the 256 MiB heap:** desk3 (heap
fix) = **0** teardown GPs; desk6 = **1**. So the heap transition *reduced* the
inbox-registry corruption (less pressure/fragmentation) but did **not eliminate
it** — a residual stray writer corrupts the registry `BTreeMap` occasionally.
That intermittent teardown #GP — not the present code — is the true blocker for
a live wallpaper desktop (it wedges the kernel before the compositor presents).

**Next:** hunt the residual registry corruptor (watchpoint the registry nodes,
or audit IPC enqueue/Arc<Inbox> paths for a stray write/UAF), independent of the
heap-pressure factor already addressed. The present code itself needs no change.

## Lean GUI profile result — confirms the teardown #GP is the universal blocker

Added `nonos-desktop-lean` (skip network/apps/market; GUI core only) to test
whether removing the buggy app capsules lets the compositor present. Result:
the compositor reaches `setup complete` + `[desktop_shell] boot`, but **the
screen still shows the kernel desktop, not the compositor output** — because a
**GUI-core capsule still faults** (pid 12, `ret` to `rip=0x0` — a capsule
returning from `_start` into a zeroed initial RA) and its **teardown #GPs the
kernel** (`8000fdac`) before the compositor's first present. So the lean profile
*reduced* the fault surface but did not yield a live desktop.

**Converged conclusion:** the teardown #GP (inbox-registry `BTreeMap` corruption)
fires on **any** capsule death and wedges the kernel — it is the universal
blocker for a stable/live desktop, not the apps, net, present code, or heap size
alone. The 256 MiB heap reduced its frequency but it recurs intermittently.
Secondary finding: `setup_initial_user_context` leaves the initial user RA at
`0x0`, so a capsule that returns from `_start` (instead of `mk_exit`) jumps to
NULL — one source of capsule faults that then trigger the teardown #GP.

**The one fix that unblocks everything:** make capsule teardown robust — root
out the residual inbox-registry corruptor (watchpoint the registry nodes / audit
IPC enqueue + `Arc<Inbox>` for a stray write or UAF), so capsule deaths stop
wedging the kernel. Then the compositor (present path already verified correct)
presents the wallpaper.

## Status of the broader effort
- FIXED+verified: user-rsp drift (`8d2d0e5c1`), PF-loop on kernel address
  (`74fb14aeb`). Desktop fleet launches; minimal-repro zero TRAP; full-desktop
  traps 1901→2.
- NEXT: this heap transition (root of the teardown #GP), then the compositor
  GOP-fb present path (live wallpaper).
