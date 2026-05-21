# Next blocker — widespread userland crashes (post GS-fix)

## Context
After the GS-base interrupt fix (`362095aee`) removed the silent timer-ISR
deadlock, the kernel reaches the gpu and the fault handlers now PRINT instead
of deadlocking. This unmasked a **second, distinct** failure: multiple capsules
crash in userland. This note tracks pinning and fixing it. The GUI milestone
(static desktop) is blocked behind it because the gpu capsule (`pid=8`) is one
of the crashers and stops at `find: bar ok`.

## Symptoms (from `/tmp/fix1-smp1.log`, `/tmp/fix1-verify.log`, SMP=1)
- User `#PF`, cpl=3, across several pids (`0x8` gpu, `0x17`=23, …):
  - near-null derefs: `cr2=0x10`, `cr2=0x1000`
  - **jump to `rip=0`** with `err=0x15` (P|U|instruction-fetch) — a `ret`/call
    through a **corrupted return address / function pointer**.
- Each crashed capsule then triggers a kernel `#GP`, cpl=0, at
  `ipc::nonos_inbox::inbox::Inbox` BTreeMap `remove_kv` (inbox teardown over a
  corrupted/!valid map as the capsule dies). err=0 (non-selector).
- The gpu's `debug::marker` buffer prefix gets clobbered (zeros, or a
  user-stack-ptr + an incrementing counter) between fill and the `mk_debug`
  read — i.e. **userland stack memory mutating under the capsule**.

## Hypotheses (ranked)
1. **Systemic userland stack corruption** by a kernel path that runs on every
   capsule (most likely the **timer-preemption context save/restore** — it
   touches user context on every preempt; the clobber value looked like a saved
   `(rsp, counter)`). The marker-buffer clobber + corrupted return addresses +
   multi-capsule spread all fit one shared corruptor.
2. A usercopy / user-stack-build bug on a common syscall or signal path.
3. The `#GP` in inbox teardown is *secondary* (consequence of a capsule dying
   with corrupted heap/IPC state), not the root.

## Plan
- Gather the full trap inventory (all pids, rips, cr2, err) from a clean boot to
  see whether the crashes share a trigger (e.g. always shortly after the first
  timer preempt of that capsule).
- Read the preemption save/restore path
  (`src/process/scheduler/preemption/`, `context/switch/resume.rs`,
  `restore_user_context_iretq`) for any write into the user stack at the saved
  rsp, or an off-by-one in the saved-frame layout.
- This is layout/timing-sensitive; **interactive GDB** (watchpoint on the
  clobbered stack slot, or break at the preempt save) is likely more effective
  than serial markers.

## Ruled out (2026-05-21)
- **Preemption is NOT the corruptor.** Disabling the timer preempt switch
  (`tick.rs`) while keeping the GS fix: corruption (`buf[0]` ← `0x7fffffffff80`
  + incrementing counter) and the crashes persist on SMP=1.
- **libc syscall path is clean.** `userland/libc/src/syscall/raw.rs` uses
  `options(nostack)` + registers only; `SYSCALL` saves return state to rcx/r11,
  not the stack — so `mk_debug` does not clobber its own buffer.
- The resume paths (`restore_user_context_iretq`, `resume_user_asm`) build the
  iretq frame on the **kernel** stack, not the user stack.

## Refined symptom
On SMP=1 cooperative (no preemption), the only async kernel entry during a
capsule's CPL=3 run is the timer ISR — which provably does not write user
memory. Yet `buf[0..16]` (a 219-byte stack array in `debug::marker`) is
overwritten with a fixed-ish high user pointer `0x00007fffffffff80` + a per-call
counter, while bytes 16+ (`"] " + label`) survive. The gpu (`pid=8`) then
crashes writing to `0x10` (null+offset) right after `find` returns; other
capsules jump to `rip=0` / garbage. Candidate not yet excluded: **cross-capsule
user-stack aliasing** (two capsules' user stacks mapping the same physical
frame — an asid/cr3/paging bug), which would let one capsule's `{ptr,counter}`
land in another's stack.

## Next step
Needs a **watchpoint** (interactive GDB/lldb via the `-s` stub): break when the
gpu reaches `find:listed`, read `&buf`, set a hardware write-watchpoint on it,
continue, and catch the writer's RIP. Static analysis is exhausted. Likely
eK's domain (paging/asid or context machinery).

## Also ruled out (2026-05-21)
- **Cross-capsule stack aliasing: NO.** Instrumented `allocate_user_stack` to
  print each pid's top stack frame phys — all **unique**, zero duplicates
  across ~30 capsules. Stacks do not share frames.
- **RSP0 in user memory: unlikely.** Per-process kernel stacks come from
  `allocate_pages` (`kernel_stack.rs`) = high-half kernel VAs; the corrupt
  value `0x7fffffff…` is user-half, so it isn't the kernel stack top, and the
  CPL=3 interrupt frame is pushed to a valid kernel RSP0.

## DECISIVE (2026-05-22): it's a real, stable async WRITE — not a read anomaly
Double-read test: `sys_mk_debug` reads the user buffer twice back-to-back and
compares. Result: **zero inconsistencies** across the whole boot, yet the gpu
markers are still corrupted. So both reads see the *same* garbage — the user
memory genuinely holds it. `copy_from_user` is fine; the capsule's `buf[0..16]`
is **really overwritten in physical memory** between `debug::marker`'s
prefix-write and the `mk_debug` read. The write is 16 bytes ({pointer,
small-counter}), async (SMP=1, so timer-ISR-driven), counter increments per
marker (≈per tick). Conclusion: a kernel interrupt path writes 16 bytes into
the running capsule's user stack at `buf[0]`. The only thing that pushes to a
stack on a CPL=3→0 interrupt is TSS.RSP0; if the gpu's RSP0/kernel-stack is
mis-set into (or overlapping) its user stack, the timer's iret-frame + GPR
pushes land in user memory. **Verify the gpu's actual RSP0 / kernel_stack_top
at runtime** and watch `buf[0]` — that is the remaining decisive datum.

## (earlier hypothesis, now superseded) read of stale bytes
`debug::marker` provably writes `buf[0..18]=PREFIX` (same code for the clean
`find:listed` and the corrupt `find:matched`/`bar ok`). Across runs the corrupt
`buf[0..16]` *varies* (all-zeros, `0x7fffffffed780`, `0x7fffffffff80`) while
`buf[16..]` (`"] " + label`) is always intact. So the kernel's `mk_debug`
`copy_from_user` reads **stale/garbage for the first 16 bytes only** of an
otherwise-correct buffer — a 16-byte-granular read/translation anomaly, not a
whole-frame mistranslation and not a traceable overwrite. Counter ~0x1f/0x20
increments per marker (≈ per timer tick).

## lldb investigation (2026-05-22) — works, and narrowed it sharply
lldb 21 attaches to QEMU's gdb stub (`-s`), no KASLR slide (ELF addrs ==
runtime). Breaking on the `#GP` handler (`gpf::handle`, 0xffffffff8003a060)
captured the exact wedge chain:
`page_fault::handle → terminate_user_process → exit::teardown →
ipc::nonos_inbox::registry::unregister_for_pid → BTreeMap remove_kv →
[0x18] #GP`. So a capsule takes a user `#PF`, and tearing down its inbox
walks a `BTreeMap` whose node pointer is smashed to `0x18` → kernel wedge.
`unregister_for_pid` itself is clean (single locked `map.remove`), so the
node was corrupted by the external write.

Caught the gpu marker buffer at its clean `find:listed` (lldb breakpoint
on `sys_mk_debug` 0xffffffff8002a760, callback matches the buffer text):
`user_va=0x7ffffffed780`. The corrupt bytes are `{0x7ffffffed780, 0x1f}` =
`{buf_ptr, total}` — i.e. **exactly the `mk_debug(buf.as_ptr(), total)`
arguments / a `{ptr,len}` slice** (libc `call_raw(N_MK_DEBUG,[buf,len,0,..])`).
This may be a `debug::marker` codegen/liveness artifact (the buffer's stack
slot vs the syscall arg array) and a **red herring** separate from the real
crashes — to be confirmed.

Watchpoint hunt status: `wp.py` catches `find:listed` and sets a conditional
hardware watchpoint (stop when `buf[0]` becomes a user pointer = the
corruptor's write). Not yet landed: the full 33-capsule boot under an
lldb breakpoint on hot `sys_mk_debug` is too slow/variable to reach the gpu
within the window. **Next: reproduce on a minimal gpu-only profile** (fast
boot → watchpoint lands in seconds), or run the session interactively.

## Disposition
Five hypotheses refuted by experiment (preemption, libc, signals, aliasing,
RSP0). Static analysis is exhausted; continuing to guess violates disciplined
debugging. **Next step is a hardware write/read watchpoint via interactive
GDB/lldb** (break at the kernel `sys_mk_debug`, read the user `buf_ptr` arg,
watch its first 16 bytes, continue, catch the writer/anomaly) — impractical to
drive non-interactively against dynamic userland addresses. Likely eK's domain
(usercopy/page-table/TLB or context machinery). Upstream GS fix committed
(`362095aee`). See `2026-05-21-gui-rca-progress.md`.
