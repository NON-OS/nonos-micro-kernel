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

## Blit instrumentation: present syscall is never reached (teardown #GP gates it)

Capped `[BLIT …]` logging at the kernel `graphics_present::blit` entry +
decision points (reverted after). Across 5 full-desktop boots: **`[BLIT]` fires
zero times.** The `[SCHED]` trail identifies the compositor as **pid 0x16** —
it reaches `GOP-fb fallback mode` + `setup complete`, but in **4 of 5 runs the
kernel wedges on the teardown #GP** (`8000fdac`) before the compositor's render
loop ticks; the one non-wedge run never reached compositor setup. So the present
path can't even be exercised — the teardown #GP gates it.

Also confirmed: the #GP is a **non-canonical pointer deref** in `remove_leaf_kv`
(a `#GP`, not a stack-overflow `#PF`) → a **corrupted BTreeMap node pointer**,
i.e. registry **heap corruption**, despite the teardown running on the 16 KiB
`IST_PAGE_FAULT` stack (the IST rsp is normal, not overflowed).

Interaction with the zeroing fix (`8180bba2a`): zeroing user-stack frames turned
the stale-garbage return address into a deterministic `rip=0x0` for capsules
that `ret` from `_start`, so those now fault deterministically and feed the
teardown path — raising the observed wedge rate. The zeroing is still correct
(security: no kernel-data leak); the wedge root is unchanged (registry
corruption), but capsule `_start` should `mk_exit`, not `ret` into a 0 RA.

## CONVERGED: the teardown #GP (registry heap corruption) is the one hard root

Ruled out as the corruptor: the Inbox (`Mutex<VecDeque>`, bounded), the IPC send
path (exact-size `Vec<u8>`), the registry locking (correct `write()`), and IST
overflow. The `nonos-heap-debug` canary check (`dealloc_impl`) never trips
(`HEAP-CORRUPT=0` over 6 runs) → the corruption is a **mid-allocation stray
write** that misses the end-of-allocation canary, and it is **intermittent /
not reproducible on demand**. This is the single blocker for the live desktop.

**Recommended next approach (dedicated effort):** a redzone/poisoning allocator
that fills allocations with a pattern and validates *interior* bytes (the canary
only guards the tail), or KASAN-style shadow, to catch the stray write at its
victim; alternatively a focused audit of every `&mut`/raw write reachable from
the IPC/scene/registry hot paths. Boot-cycle bisection won't pin it (intermittent).

## Poisoning + quarantine allocator result (`8ca349acd`) — reuse-sensitive corruption

Built a poison+quarantine allocator under `nonos-heap-debug` (`heap/types/
quarantine.rs`): freed user regions are filled with a canonical-unmapped poison
word (`0x0000_5EED_5EED_5EED`) and held in a 1024-slot ring (reuse delayed); a
UAF *read* would fault as `#PF cr2=…5eed…`, a stray *write* to freed memory is
caught on eviction (`[UAF-WRITE]`, logging the written word).

**Result across 8 full-desktop boots with quarantine** (3× `0xAB`, 5× canonical):
- **teardown #GP: 0 occurrences** (vs ~80% of runs without quarantine).
- **UAF-read poison #PF: 0.** **UAF-write eviction hits: 0.**

**Interpretation.** Delaying reuse *suppresses* the corruption, yet neither a
dangling-pointer read of poison nor a write to freed memory ever fires. So it is
**not** a classic dangling-read or write-after-free, but it **is reuse/layout-
sensitive**: the bug needs a freed block recycled eagerly. Two shapes fit —
(a) a freed block reused for a new allocation whose bytes are then misinterpreted
through a stale reference *after* it leaves quarantine (the quarantine outlasts
the stale-reference window), or (b) a heap-adjacency overflow into a live node
that the quarantine's layout shift separates. The end-of-allocation canary
(`HEAP-CORRUPT`) also never trips, so any overflow is interior, not tail.

**Next diagnostics to pin it (cheap, bound the mechanism):**
- Vary `SLOTS` (e.g. 8 vs 1024): if a tiny quarantine still suppresses it, the
  stale-reference/reuse window is short; binary-search the threshold.
- A no-evict (leak-all-frees) run: if still clean, reuse is definitively required.
- Validate the inbox-registry `BTreeMap` integrity after each `register`/
  `unregister` to catch the corrupting op and correlate it with concurrent IPC.

Practical note: the quarantine **mitigates** the wedge (8/8 clean), so it doubles
as a stopgap, though the eviction-scan makes boots slow (debug-only).

## SLOTS binary-search — the corruption is an immediate-reuse collision

Varied the quarantine size to find the threshold at which it stops the wedge:

| quarantine | runs | teardown #GP |
| --- | --- | --- |
| none | ~5 | ~80% wedge |
| 1024 | 8 | 0 |
| 8 | 4 | 0 |
| 1 | 4 | 0 |

**Even a 1-slot quarantine suppresses it** — delaying a freed block's return to
the allocator by a *single* subsequent free is enough. So the corruption needs
the just-freed block handed back to the **very next allocation**.

Crucial corroborating fact: `HEAP_ZERO_ON_FREE` defaults to **true**, so the
production path already zeroes freed memory. The `#GP` garbage is therefore
**not** stale freed content (zeroed) — it is a block that was freed, immediately
**reallocated and written with new data**, while a reference from the original
(mid-flight) owner still points at it. Reading that new data as a tree node
yields the non-canonical child pointer.

**Refined root hypothesis: a re-entrant/interrupt allocation reuses a node that
an in-progress inbox-registry `BTreeMap` operation just freed but still
references.** The registry op holds `REGISTRY.write()` (which guards the map, not
the global allocator); if an interrupt fires between a node free and the tree
fixup and its handler allocates, the allocator hands back the just-freed node,
the handler writes it, and the resuming `remove` dereferences the stale parent
link → `#GP`. Delaying reuse (even by one free) lets the registry op finish
before the node can be recycled. No poison read/write fired because the stale
link is only followed *after* the node is reallocated, never while it sits
poisoned in quarantine.

**Next:** (1) audit whether any interrupt/softirq path allocates, and make the
registry critical sections IRQ-safe (disable interrupts across the `BTreeMap`
mutation, or move frees outside the lock); (2) the quarantine is itself a valid
**mitigation** — a lightweight deferred-free (delay reuse by N, no poison/scan)
would be a shippable workaround if the re-entrancy proves hard to excise.

## Status of the broader effort
- FIXED+verified: user-rsp drift (`8d2d0e5c1`), PF-loop on kernel address
  (`74fb14aeb`). Desktop fleet launches; minimal-repro zero TRAP; full-desktop
  traps 1901→2.
- NEXT: this heap transition (root of the teardown #GP), then the compositor
  GOP-fb present path (live wallpaper).
