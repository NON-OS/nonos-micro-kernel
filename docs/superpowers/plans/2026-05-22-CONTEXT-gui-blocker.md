# NØNOS GUI/Desktop Blocker — Full Working Context

**Branch:** `feature/bootloader-hardening`  ·  **Last updated:** 2026-05-22
**Status:** root cause localized to **kernel-writes-into-capsule-user-memory**;
not yet pinned to the exact writer. Three real fixes committed; desktop renders
statically; one residual corruptor blocks a live compositor desktop.

---

## 1. Mission

Get a static GUI desktop running under QEMU:
`virtio-gpu → compositor → wallpaper → desktop_shell`, on the NØNOS upper-half
`#![no_std]` x86_64 microkernel (RAM-resident, capability-enforced, signed CPL=3
capsules). The blocker is a memory-corruption bug that crashes capsules and (via
teardown) wedges the kernel.

---

## 2. The crash signature (deterministic)

On the **minimal repro** (virtio-rng only, ~30 s, see §8):

```
[TRAP PF] cpl=3 rip=<garbage> rsp=0x00007ffffffefff0 cs=0x23 ... pid=0x2 err=0x15 cr2=<garbage>
[TRAP GP] cpl=0 rip=0xffffffff8000ff5c ... pid=0x2          (inbox BTreeMap::remove during teardown)
```

- A capsule (pid 2) faults at **CPL=3**, executing a **garbage rip**, with `rsp`
  near the **top of its user stack**.
- `err=0x15` = Present + User + Instruction-fetch → it tried to *execute* an
  address it can't (a kernel page from CPL=3).
- The capsule then gets torn down, and teardown `#GP`s in the inbox map.

**The injected rip varies across runs** (this is the key clue):
`0x2`, `0xff00`, `0x100000000`, `0x18`, `0x7ffffffffffffffc`, and pointers into
the bootstrap heap (`0xffffffff82157b20`, `…64b20`, `…67b20`, `…2e0b20`).

---

## 3. What is FIXED (committed, real fixes)

| Commit | Fix | Effect |
|---|---|---|
| `362095aee` | **GS-base ISR deadlock** | The original "GUI hang": the timer ISR did `mov gs:0,…` with GS base = 0 → `#PF` storm. Was misattributed to virtio-gpu driver logic. |
| `fcb1d2c21` | **kernel-stack UAF** in `pending_stack_free::drain` | `drain()` ran from the timer trap and could `deallocate_page` the kernel stack the CPU was *still on*. Fix: skip any deferred stack the current `rsp` falls inside, re-queue it. Killed the crash **cascade** and the kernel `rip=0` wedge. |
| `460cb7729` | **`resume_kernel_thread` missing TSS RSP0 update** | The other two CPL=3 entry paths (`try_first_entry`, `try_resume`) set RSP0; this one didn't, so a capsule resumed via it ran with stale RSP0. Real asymmetry bug. |

**Also working & visible:** the kernel renders a static desktop to the GOP
framebuffer (NONOS title + dock label), and the compositor reaches
`[compositor] setup complete` in the GOP-fb fallback (Approach B). See commits
`dc006d994`, `ccf9586ed`, `df805456e`.

---

## 4. THE DECISIVE FINDING (this is the important part)

The entire early investigation assumed a **kernel-side resume corruption**
(setjmp/longjmp context switch, frozen kstack frames, the SYSRET `rcx` slot).
**That was the wrong layer.** Proven by a targeted probe:

> A feature-gated assertion in the syscall-exit asm (`syscall.S`, just before
> `pop rcx`) checked `rsp == kstack_top - 24` (the saved-rcx slot). **It never
> fired.** So the SYSRET resume `rsp` is *correct* — the kernel resumes the
> capsule to a **valid** rip.

Therefore:

1. **The kernel resume machinery is NOT the bug.** `Context`/setjmp/the global
   `INTERRUPT_SAVED_CONTEXTS`/`INTERRUPT_SAVED_FPU_STATES` BTreeMaps are sound
   for this crash. (The probe was reverted after proving this.)
2. The capsule **resumes correctly, runs in user mode, then jumps to garbage** —
   i.e. a return address / function pointer **in its own user memory** is
   corrupted, and it `ret`s/`call`s into the garbage.
3. **pid 2 is virtio-rng**, the DMA/IRQ-looping driver (it `mk_yield`s 100+
   times in its fill loop). It is **NOT proof-io** — proof-io's `_start` is
   trivially `mk_debug; mk_exit` and cannot loop. (Earlier notes that said
   "proof-io" were wrong about which pid.)
4. **The injected values are KERNEL data**, not device/entropy bytes:
   - `0x7ffffffffffffffc` = the `movabsq $0x7ffffffffffffffc, %r14` constant in
     `allocate_kernel_stack` (a value that lives on the **kernel stack**).
   - `0xffffffff82157b20` etc. = pointers **into the 16 MiB bootstrap heap**
     (`BOOTSTRAP_HEAP_MEMORY = 0xffffffff82150000`; confirmed via `llvm-nm`).
   So the writer copies **live kernel stack/heap memory** onto a capsule's user
   stack — a kernel-pointer/info leak that also smashes control flow.

**Conclusion:** something in the **kernel** writes kernel memory contents onto
**virtio-rng's user stack** (overwriting a return address). The capsule then
jumps there → CPL=3 fetch fault → teardown `#GP`.

---

## 5. What has been RULED OUT (do not re-investigate)

- **Kernel resume / context switch** (rsp-probe never fired — §4).
- **Saved `Context`** (validated sane; `CTX-BAD` guard never fired).
- **The SYSRET `rcx` slot** at `kstack_top-24` (intact at crash; phys stable; no
  write caught on its VA or directmap alias).
- **Stack overflow** (32 KB/64 KB kstack made no difference).
- **`context_restore_asm`** logic (verified correct).
- **All syscall out-struct ABI sizes** — kernel == userland, each with a
  `static_assert`: `IrqPollOut` 16, `MmioMapOut` 24, `DmaMapOut` 32,
  `IrqBindOut` 16, `DeviceRecord` 176. **Not a usercopy out-struct overflow.**
- **virtio-rng virtqueue DMA overflow** — `QUEUE_SIZE=16` ⇒ 8192-byte region,
  used ring at offset 4096; QEMU's virtio-rng vq is size **8** (≤16), so the
  device writes *within* the allocated region. The device does **not** DMA past
  the buffer. (There is a *functional* avail-ring offset mismatch between
  device-size-8 and driver-size-16 layouts — worth noting but not a memory
  overflow.)
- **page remap / timer-snapshot path / frame-allocator fallback / heap-DMA frame
  collision / DMA scrub** (all eliminated earlier).

---

## 6. CURRENT HYPOTHESIS & next probe

**Hypothesis:** a kernel path writes live kernel memory (stack/heap contents)
onto virtio-rng's user stack — either a usercopy with a wrong destination, a
variable-length copy that overruns the user buffer, or a write that targets the
wrong user offset. Source values implicate uninitialized/live **kernel stack**
(the allocator mask) and **bootstrap heap** (object pointers).

**Next probe (decisive, was about to be implemented):** dump virtio-rng's user
stack **at the crash**. At a CPL=3 fault the faulting process's CR3 is active, so
the kernel can read the user VA (`0x7fff…`) directly. Add to
`src/arch/x86_64/diag/dump_trap.rs` (or `page_fault::handle`), gated/temporary:
when `cpl==3 && rip >= 0xffff_8000_0000_0000 && pid==2`, dump
`[rsp-128 .. rsp+128]` of the **user VA**. The corruption **pattern** (how many
contiguous slots are kernel data, their alignment) reveals the source:
- a clean N-byte block of kernel data ⇒ a structured usercopy of size N → match
  it to a syscall out-path;
- a single wrong slot ⇒ a stray pointer write.

**Then:** retarget the `nonos-trap-kstack-writer` DR0/DR1 hardware-watchpoint
facility (designed in the spec/plan below) from the **kstack** to the
**user-stack** return slot: arm DR0 = user-stack slot VA, DR1 = its directmap
alias, while pid 2 is parked; the `#DB` handler logs the writer's RIP. Native
speed (no gdbstub). Symbolize the RIP (no KASLR) → the writer.

**Candidate kernel paths to scrutinize** (write to a capsule's user memory):
- `mk_device_list` (`sys_device_list`) — does it bound the record count to the
  user's `max_records`, or write `actual` and overrun the user buffer?
- Any usercopy in virtio-rng's syscalls: `mk_mmio_map`, `mk_dma_map`,
  `mk_irq_bind`, `mk_irq_poll`, `mk_irq_ack` (out-structs are size-matched, but
  re-check the *destination pointer* validation and any signal/IRQ delivery that
  pushes a frame onto the user stack).
- The IRQ/signal delivery path to a capsule (does anything push onto the user
  stack on IRQ?).

---

## 7. Architecture notes relevant to the bug

- Kernel at PML4[511] = `0xffff_ffff_8000_0000`; 256 GiB directmap at PML4[256]
  = `0xffff_8000_0000_0000`; PML4[0] cleared post-handoff. Kernel reaches user
  pages via the directmap; **never switches CR3 to a user table**. Capsules run
  CPL=3 in their own ASID.
- `BOOTSTRAP_HEAP_MEMORY = 0xffffffff82150000` (16 MiB static; early allocations:
  PCBs, Arcs, BTreeMap nodes, IPC messages live here).
- `PERCPU_DATA = 0xffffffff831cb000`; `kernel_stack_top` at PerCpuData offset
  `0x20` (= `gs:PCPU_KERN_STACK` in `syscall.S`).
- Cooperative scheduling: `mk_yield → sys_yield → yield_now()=hlt`; the timer ISR
  preempts and context-switches. Three CPL=3 entry paths in
  `switch_to_user_pcb_x86_64` (`dispatch.rs`): `try_first_entry` (iretq),
  `try_resume` (snapshot iretq), `resume_kernel_thread` (Context → SYSRET).
- virtio-rng driver (`userland/capsule_driver_virtio_rng/`): legacy virtio-pci;
  setup = device_list → claim → mmio_map → irq_bind → dma_map(queue, 8192) →
  dma_map(buffer, 4096) → bring_up; then a fill loop of `mk_irq_poll` + `mk_yield`
  + reads the used ring. `QUEUE_SIZE=16` (hardcoded), `VQ_REGION_SIZE=8192`,
  `ENTROPY_BUF_LEN=4096`.

---

## 8. The fast deterministic repro (~30 s)

```bash
# 1. build minimal kernel (virtio-rng prod profile)
make nonos-mk-driver-virtio-rng-prod
# 2. sign
/usr/bin/python3 nonos-utils/sign_kernel.py \
  target/x86_64-nonos/release/nonos-kernel \
  nonos-bootloader/keys/signing_key_v1.bin target/kernel_signed.bin
# 3. attest
nonos-bootloader/tools/embed-zk-proof/target/x86_64-apple-darwin/release/embed-zk-proof \
  --input target/kernel_signed.bin --output target/kernel_attested.bin \
  --proving-key nonos-bootloader/tools/nonos-attestation-circuit/generated_keys/attestation_proving_key.bin \
  --seed "nonos-production-attestation-v1-2026"
# 4. package
cp target/kernel_attested.bin target/esp/EFI/nonos/kernel.bin
# 5. boot (SMP=1; add -s -S to attach lldb on :1234; -monitor tcp for gva2gpa)
qemu-system-x86_64 -m 2G -cpu max -smp 1 -machine q35 \
  -drive format=raw,file=fat:rw:target/esp \
  -drive if=pflash,format=raw,readonly=on,file=/usr/local/share/qemu/edk2-x86_64-code.fd \
  -device virtio-vga,disable-modern=on,vectors=0,xres=1024,yres=768 \
  -device virtio-rng-pci -serial file:/tmp/s.log \
  -monitor tcp:127.0.0.1:55557,server,nowait -display none -no-reboot &
sleep 70 ; grep -a TRAP /tmp/s.log
# always between runs:
lsof -i :1234 -t | xargs -r kill -9; lsof -i :55557 -t | xargs -r kill -9; pkill -9 -f qemu-system-x86_64
```

**Flag-on build** (for the trap facility): the make target builds flag-off; get
the cargo line with `make -n nonos-mk-driver-virtio-rng-prod | grep 'cargo build'`
and append `,nonos-trap-kstack-writer` to `--features`.

Build ≈ 70 s. No KASLR slide → `llvm-symbolizer --obj=<kernel> <addr>` is exact.

---

## 9. Constraints (project rules — must honor)

- `#![no_std]`, custom target `x86_64-nonos.json`, Makefile-driven (not plain
  cargo). Host fast-check: `cargo check --lib --features std --target
  x86_64-apple-darwin`.
- **75-line max growth per file per change.** No `//` comments inside function
  bodies. Preserve AGPL headers. Surgical changes only.
- Commits `fix(scope): imperative`, **no `Co-Authored-By`**, git user `senseix21`.
- Any debug facility behind a feature flag (`nonos-trap-kstack-writer`), never in
  a shipping profile, reverted before merge.

---

## 10. Artifacts produced this session (all committed)

- `docs/superpowers/plans/NOTE-FOR-EK-gui-blocker.md` — handoff note for eK.
- `docs/superpowers/specs/2026-05-22-kstack-writer-trap-design.md` — design spec
  for the DR0/DR1 hardware-watchpoint facility.
- `docs/superpowers/plans/2026-05-22-kstack-writer-trap.md` — phased
  implementation plan (annotated **RETARGETED**: kstack → user stack).
- `docs/superpowers/plans/2026-05-22-ctx-switch-rewrite-PROMPT.md` — Jon-Gjengset
  investigation prompt for the (now-deprioritized) context-switch rewrite.
- `docs/superpowers/plans/2026-05-22-gui-desktop-unblock.md` — running RCA;
  contains the "DECISIVE pivot" section (§4 here).

> Note: the context-switch rewrite (setjmp + BTreeMaps → PCB-stored-rsp switch)
> was deprioritized — §4 proved the resume is not the bug. It remains worthwhile
> **hygiene** but will not fix this crash.

---

## 11. TL;DR for the next session

The resume is fine. **The kernel scribbles live kernel memory (stack/heap
contents) onto virtio-rng's user-stack return address; the capsule jumps to it.**
Next: (a) dump pid 2's user stack at the crash to read the corruption pattern
(§6), then (b) arm the DR watchpoint on that user-stack slot while pid 2 is
parked to name the writer's RIP. Out-struct ABIs and virtqueue DMA are already
cleared — focus on kernel paths that *write* a capsule's user memory.
