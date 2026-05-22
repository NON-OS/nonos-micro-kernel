# Kstack Wild-Writer Trap — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Catch the exact kernel RIP that writes a kernel-BSS pointer into a parked capsule's frozen kernel-stack frame, then fix the root so the desktop boots cleanly.

**Architecture:** A feature-flagged (`nonos-trap-kstack-writer`) kernel facility arms CPU debug registers DR0 (frame VA) + DR1 (its directmap alias) as 8-byte *write* breakpoints whenever the target capsule (pid 2, the deterministic crasher) is parked; the existing #DB handler logs the trapping RIP and halts. Runs at native speed — no gdbstub. Once the writer is named, fix it. The fragile context-switch rewrite is a separate follow-on.

**Tech Stack:** Rust `#![no_std]`, x86_64, custom target `x86_64-nonos.json`, Makefile build, QEMU/OVMF boot smoketest (no `cargo test` for the image — verification is serial markers on the minimal repro).

**Verification model:** there is no unit-test harness for the kernel image. The "test" for every task is: build, then boot the ~30 s deterministic minimal repro and read `/tmp/s.log`. Repro recipe is **Appendix A**; use it verbatim wherever a task says "run the repro".

---

## File structure

- `Cargo.toml` — add the `nonos-trap-kstack-writer` feature (off by default).
- Create `src/interrupts/debug_watch/mod.rs` — the facility: `arm(frame_va)`,
  `disarm()`, the const `TARGET_PID`, and the DR read/write asm. One
  responsibility: own the debug registers for this trap.
- `src/interrupts/handlers/exceptions/<debug handler>.rs` — extend the existing
  `#DB` handler (`handlers::debug`) to log DR6 + RIP + value and halt, **only**
  under the feature flag.
- `src/process/scheduler/preemption/switch.rs` — arm on the park (preempt) side.
- `src/process/scheduler/preemption/yield_body.rs` — arm on the cooperative
  yield side.
- `src/arch/x86_64/context/switch/dispatch.rs` — disarm before running the
  target.
- `src/interrupts/mod.rs` (or wherever modules are declared) — declare
  `debug_watch`.

All new behavior is `#[cfg(feature = "nonos-trap-kstack-writer")]`; with the flag
off the kernel is byte-for-byte unchanged.

---

## Task 1: Locate the corrupted frame offset (investigation, no commit)

**Files:** none (lldb session against the repro).

- [ ] **Step 1: Build + boot the minimal repro halted under lldb.**

Run the build/sign/package from Appendix A, then:
```
qemu-system-x86_64 -m 2G -cpu max -smp 1 -machine q35 \
  -drive format=raw,file=fat:rw:target/esp \
  -drive if=pflash,format=raw,readonly=on,file=/usr/local/share/qemu/edk2-x86_64-code.fd \
  -device virtio-vga,disable-modern=on,vectors=0,xres=1024,yres=768 \
  -device virtio-rng-pci -serial file:/tmp/s.log -display none -no-reboot -s -S &
```

- [ ] **Step 2: Break at the crash and read the per-cpu kstack top.**

```
/usr/local/opt/llvm/bin/lldb target/x86_64-nonos/release/nonos-kernel --batch \
  -o "gdb-remote localhost:1234" \
  -o "breakpoint set --address 0xffffffff80045200" \
  -o "continue" \
  -o "x/6gx 0xffffffff8215d000" \
  -o "quit"
```
`0xffffffff80045200` is `page_fault::handle` in the minimal kernel (re-verify
with `llvm-nm target/x86_64-nonos/release/nonos-kernel | grep page_fault6handle`
if the build changed). `0xffffffff8215d000` is `PERCPU_DATA`; offset +32 (5th
qword) is the crashing capsule's `kernel_stack_top`. Expected ~`0xffffff50…`.

- [ ] **Step 3: Walk the kstack to find the BSS pointer.**

With `KTOP` = the kernel_stack_top from Step 2, dump the top 512 bytes:
```
… -o "x/64gx <KTOP - 512>" -o "quit"
```
Find the qword whose value is in the kernel-BSS range `0xffffffff821…`–`822…`
(the corrupt return frame). Record **`N` = KTOP − that_slot_addr** (the offset).
Expected: `N` is a fixed value across runs (the `mk_yield` call depth is
constant). If two runs disagree, record both and the plan watches each.

- [ ] **Step 4: Record the directmap alias path (sanity).**

```
printf 'gva2gpa 0x<slot_addr>\n' | nc -w 2 127.0.0.1 55557   # needs -monitor; see Appendix A
```
Confirms the slot is mapped and gives its phys (for cross-checking the alias
the kernel will compute). No value is hardcoded — the kernel computes it live.

- [ ] **Step 5: Write `N` into the plan and the module.**

Record `N` here: `KSTACK_BAD_OFFSET = 0x____`. Used in Task 5.

---

## Task 2: Feature flag + module skeleton

**Files:**
- Modify: `Cargo.toml` (the `[features]` block, ~line 53)
- Create: `src/interrupts/debug_watch/mod.rs`
- Modify: `src/interrupts/mod.rs` (module declarations)

- [ ] **Step 1: Add the feature.** In `Cargo.toml` under `[features]`:

```toml
# Debug-only: trap the wild writer that corrupts a parked capsule's
# kernel stack. Arms DR0/DR1 from kernel code. Not in any shipping profile.
nonos-trap-kstack-writer = []
```

- [ ] **Step 2: Create the module skeleton** `src/interrupts/debug_watch/mod.rs`
(AGPL header omitted here for brevity — copy it verbatim from any sibling file):

```rust
//! Debug-register trap for the parked-kstack wild writer. Feature-gated;
//! arms DR0 (frame VA) and DR1 (its directmap alias) as 8-byte write
//! breakpoints so the #DB handler can name the writer's RIP.

#![cfg(feature = "nonos-trap-kstack-writer")]

use crate::memory::addr::VirtAddr;
use crate::memory::layout::DIRECTMAP_BASE;

/// The deterministic crasher in the minimal virtio-rng repro.
pub const TARGET_PID: u32 = 2;
/// Offset of the corrupted return frame below kernel_stack_top. This is an
/// output of Task 1 (the lldb walk); set it before Task 5 builds. 0x0 is a
/// deliberate sentinel that makes Task 6 miss until Task 1 fills it.
pub const KSTACK_BAD_OFFSET: u64 = 0x0;

pub fn arm(_frame_va: u64) {}
pub fn disarm() {}
```

- [ ] **Step 3: Declare the module.** In `src/interrupts/mod.rs` add:

```rust
#[cfg(feature = "nonos-trap-kstack-writer")]
pub mod debug_watch;
```

- [ ] **Step 4: Build (flag on) to verify it compiles.**

The Makefile target builds flag-*off*; to build flag-*on*, get the exact cargo
invocation the target uses and append the feature:
```
make -n nonos-mk-driver-virtio-rng-prod | grep 'cargo build'
```
Copy that command and append `,nonos-trap-kstack-writer` to its `--features`
list, then run it. Expected: `Finished release`. (This resolved command is the
"flag-on build" referenced in Tasks 3–6; re-run it there.)

- [ ] **Step 5: Commit.**

```bash
git add Cargo.toml src/interrupts/debug_watch/mod.rs src/interrupts/mod.rs
git commit -m "debug(sched): scaffold nonos-trap-kstack-writer facility"
```

---

## Task 3: Implement DR arm/disarm

**Files:** Modify `src/interrupts/debug_watch/mod.rs`

- [ ] **Step 1: Implement `arm`/`disarm` with the DR asm.** Replace the stubs:

```rust
// DR7: L0|L1 enable (bits 0,2), reserved bit 10, and for DR0/DR1
// R/W=01 (write), LEN=10 (8 bytes): nibble 0x9 at bits 16 and 20.
const DR7_ARMED: u64 = 0x0099_0405;
const DR7_OFF: u64 = 0x0000_0400;

pub fn arm(frame_va: u64) {
    let alias = match crate::memory::unified::translate::virt_to_phys(VirtAddr::new(frame_va)) {
        Some(pa) => DIRECTMAP_BASE + pa.as_u64(),
        None => frame_va,
    };
    unsafe {
        core::arch::asm!(
            "mov dr0, {a}",
            "mov dr1, {b}",
            "mov dr7, {c}",
            a = in(reg) frame_va,
            b = in(reg) alias,
            c = in(reg) DR7_ARMED,
            options(nostack, preserves_flags),
        );
    }
}

pub fn disarm() {
    unsafe {
        core::arch::asm!("mov dr7, {c}", c = in(reg) DR7_OFF,
            options(nostack, preserves_flags));
    }
}
```

- [ ] **Step 2: Build (flag on).** Run the Step-2.4 build command. Expected:
`Finished release`. (Confirms the DR asm + `virt_to_phys` path compile.)

- [ ] **Step 3: Commit.**

```bash
git add src/interrupts/debug_watch/mod.rs
git commit -m "debug(sched): arm DR0/DR1 (frame VA + directmap alias) write watchpoints"
```

---

## Task 4: Extend the #DB handler to log + halt

**Files:** Modify the `#DB` handler reached via `isr_debug → handlers::debug`
(find it: `grep -rn 'pub fn debug' src/interrupts/handlers/`). It takes an
`InterruptStackFrame`.

- [ ] **Step 1: Add the trap-log path** at the very top of `debug(frame)`,
gated by the flag, before the existing body:

```rust
#[cfg(feature = "nonos-trap-kstack-writer")]
{
    let dr6: u64;
    unsafe { core::arch::asm!("mov {}, dr6", out(reg) dr6, options(nostack, preserves_flags)); }
    if dr6 & 0b11 != 0 {
        crate::sys::serial::print(b"[KSTACK-WRITER] rip=");
        crate::arch::x86_64::diag::print_hex_u64(frame.instruction_pointer.as_u64());
        crate::sys::serial::print(b" dr6=");
        crate::arch::x86_64::diag::print_hex_u64(dr6);
        crate::sys::serial::println(b"");
        crate::interrupts::debug_watch::disarm();
        unsafe { core::arch::asm!("mov dr6, {z}", z = in(reg) 0u64, options(nostack, preserves_flags)); }
        crate::arch::halt_loop();
    }
}
```

Note: a data-write #DB is a *trap* — `frame.instruction_pointer` is the
instruction **after** the write, so the writer is the instruction just before
the logged RIP. Halting keeps the serial log intact.

- [ ] **Step 2: Build (flag on).** Run the Step-2.4 build. Expected: `Finished`.

- [ ] **Step 3: Commit.**

```bash
git add src/interrupts/handlers/exceptions/<file>.rs
git commit -m "debug(sched): #DB handler logs kstack-writer RIP and halts"
```

---

## Task 5: Wire arm/disarm into the park and resume paths

**Files:**
- Modify `src/process/scheduler/preemption/switch.rs` (`preempt_current_process`)
- Modify `src/process/scheduler/preemption/yield_body.rs` (`perform_yield_inline`)
- Modify `src/arch/x86_64/context/switch/dispatch.rs` (`switch_to_user_pcb_x86_64`)

- [ ] **Step 1: Arm on preempt-park.** In `preempt_current_process`, immediately
after `save_interrupt_context(curr_pid, ctx);`:

```rust
#[cfg(feature = "nonos-trap-kstack-writer")]
if curr_pid == crate::interrupts::debug_watch::TARGET_PID {
    if let Some(p) = PROCESS_TABLE.find_by_pid(curr_pid) {
        let top = p.kernel_stack_top.load(core::sync::atomic::Ordering::Acquire);
        if top != 0 {
            crate::interrupts::debug_watch::arm(top - crate::interrupts::debug_watch::KSTACK_BAD_OFFSET);
        }
    }
}
```

- [ ] **Step 2: Arm on cooperative-yield-park.** In `perform_yield_inline`, after
its `save_interrupt_context(pid, ctx);` line, add the same block with `pid` in
place of `curr_pid` (repeat the code — do not abbreviate):

```rust
#[cfg(feature = "nonos-trap-kstack-writer")]
if pid == crate::interrupts::debug_watch::TARGET_PID {
    if let Some(p) = crate::process::nonos_core::PROCESS_TABLE.find_by_pid(pid) {
        let top = p.kernel_stack_top.load(core::sync::atomic::Ordering::Acquire);
        if top != 0 {
            crate::interrupts::debug_watch::arm(top - crate::interrupts::debug_watch::KSTACK_BAD_OFFSET);
        }
    }
}
```

- [ ] **Step 3: Disarm before the target runs.** In `switch_to_user_pcb_x86_64`,
right after `find_by_pid` returns the pcb and before `try_first_entry`:

```rust
#[cfg(feature = "nonos-trap-kstack-writer")]
if pid == crate::interrupts::debug_watch::TARGET_PID {
    crate::interrupts::debug_watch::disarm();
}
```

- [ ] **Step 4: Build (flag on).** Run the Step-2.4 build. Expected: `Finished`.
Also build **flag-off** (the prod profile) to confirm zero impact:
`make nonos-mk-driver-virtio-rng-prod` → `Finished`.

- [ ] **Step 5: Commit.**

```bash
git add src/process/scheduler/preemption/switch.rs \
        src/process/scheduler/preemption/yield_body.rs \
        src/arch/x86_64/context/switch/dispatch.rs
git commit -m "debug(sched): arm kstack-writer watchpoint on park, disarm on resume"
```

---

## Task 6: Catch the writer

**Files:** none (boot + read serial).

- [ ] **Step 1: Build the flagged minimal kernel + package.** As Appendix A but
add `,nonos-trap-kstack-writer` to the build's `--features`, then sign/attest/
package into `target/esp` (Appendix A steps 2–4 unchanged).

- [ ] **Step 2: Boot (no lldb) and read the trap log.**

Boot the QEMU line from Appendix A step 5, `sleep 60`, then:
```
grep -a 'KSTACK-WRITER' /tmp/s.log
```
Expected: one line `[KSTACK-WRITER] rip=0xffffffff80…… dr6=0x…`. If absent and a
`TRAP` line appeared first, the watchpoint missed (wrong offset/alias) → revisit
Task 1 `N`, and add DR2 on the alternate alias.

- [ ] **Step 3: Symbolize the writer.**

```
llvm-symbolizer --obj=target/x86_64-nonos/release/nonos-kernel <rip from Step 2>
```
No KASLR slide — the symbol is exact. Record the function + `file:line`. The
writer is at/just-before this RIP.

- [ ] **Step 4: Commit the finding** into this plan (edit below) and the RCA note:

`WRITER = <function> @ <file:line>; dr6=<…> (DR0=VA / DR1=directmap alias).`

---

## Task 7: Diagnose and fix the root

**Files:** the writer's source (`file` from Task 6, Step 3) + this plan.

- [ ] **Step 1: Read the writer.** Open `file:line`. Determine *why* it writes
into the parked capsule's kstack frame: a destination pointer computed from the
wrong base, a length/stride overrun, a stale/reused pointer, or a struct copy to
a wrong target. Write the one-sentence root cause here.

- [ ] **Step 2: Form the minimal fix** matching the root (e.g. correct the
destination/base, bound the length, drop the stale pointer). Keep it surgical;
no unrelated changes; ≤75 lines.

- [ ] **Step 3: Build flag-off (prod) + run the repro.**

```
make nonos-mk-driver-virtio-rng-prod   # then sign/attest/package, Appendix A 2-4
# boot Appendix A step 5, sleep 75
grep -ac 'TRAP' /tmp/s.log    # expect 0
```
Expected: **zero `TRAP`**, kernel stays alive (serial keeps advancing).

- [ ] **Step 4: Commit the fix.**

```bash
git add <writer file>
git commit -m "fix(<scope>): <root cause> corrupting parked capsule kernel stack"
```

---

## Task 8: Verify on the full desktop + retire the facility

**Files:** possibly remove the trap module / its hooks (or leave flag-off).

- [ ] **Step 1: Full-desktop verification.**

```
make nonos-mk-desktop-gui-prod && make nonos-mk-esp
# boot graphically (Appendix A QEMU line without -display none, or screendump),
# sleep 120
grep -ac 'TRAP' /tmp/s.log            # expect 0
grep -aiE 'compositor|present' /tmp/s.log | tail
```
Expected: zero `TRAP`, system stable, compositor presents past `setup complete`.
Capture a `screendump` to confirm the desktop renders the compositor's content.

- [ ] **Step 2: Decide on the facility.** It is flag-off in all shipping
profiles, so it is safe to leave. If preferred, revert the `debug_watch` module
and its three hook sites (the feature was a one-shot diagnostic):

```bash
git revert --no-commit <Task2..Task5 commits>   # or delete the files + hooks
git commit -m "debug(sched): drop kstack-writer trap facility (writer fixed)"
```

- [ ] **Step 3: Update the RCA notes** (`docs/superpowers/plans/2026-05-22-
gui-desktop-unblock.md`) with the named writer + the fix, and tick the GUI
milestone.

---

## Appendix A — the ~30 s deterministic minimal repro

```bash
# 1. build minimal kernel (add ,nonos-trap-kstack-writer for Tasks 1,6)
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
# 5. boot (add -monitor tcp:127.0.0.1:55557,server,nowait for gva2gpa; -s -S to attach lldb)
qemu-system-x86_64 -m 2G -cpu max -smp 1 -machine q35 \
  -drive format=raw,file=fat:rw:target/esp \
  -drive if=pflash,format=raw,readonly=on,file=/usr/local/share/qemu/edk2-x86_64-code.fd \
  -device virtio-vga,disable-modern=on,vectors=0,xres=1024,yres=768 \
  -device virtio-rng-pci -serial file:/tmp/s.log -display none -no-reboot &
```
Crash signature (before the fix): `pid 2` (proof-io) `TRAP PF cpl=3 rip=<garbage>`
then `TRAP GP cpl=0 rip=0xffffffff8000ff5c` (inbox `BTreeMap::remove`). Always
`lsof -i :1234 -t | xargs -r kill -9; pkill -9 -f qemu-system-x86_64` between runs.
```
