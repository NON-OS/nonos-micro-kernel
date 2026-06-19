# Handoff module — CONTRIBUTING

Hey, this is Ek. Quick, practical rules for working on the handoff code in this repo. Keep changes small, testable, and obvious. 
The handoff layer is the last thing that runs in firmware before the kernel — mistakes here are painful to debug on hardware, so follow the rules.

What this module does
- Capture UEFI memory map (uefi = 0.23 semantics).
- Call ExitBootServices(image_handle, MemoryMapKey).
- Allocate one BootHandoffV1 page (repr(C)) and populate it (magic/version/size/entry_point/mmap pointers).
- Allocate an optional cmdline buffer and a small handoff stack, then trampoline to kernel.entry with RDI = pointer-to-BootHandoffV1 and RSP = stack_top.
- Leave the memory-map buffer allocated for the kernel to read.

Location & organisation
- Path: `src/handoff/handoff.rs` and `src/handoff/mod.rs`.

Coding rules (my expectations)
- Use `#![no_std]`.
- Keep BootHandoffV1 exactly matching the kernel’s ABI. Do not change layout without a coordinated kernel change.
- Minimize `unsafe`. When you must use `unsafe`, add a one-line justification above the block describing the invariant you rely on.
- No runtime heap in the hot path. Use BootServices `allocate_pages` / `free_pages` only.
- Use SystemTable-based logging helpers (log_info/log_error) — we need serial output for CI and debugging.

BootHandoffV1 contract
- Always set:
  - `magic` == `HANDOFF_MAGIC`
  - `version` == `HANDOFF_VERSION`
  - `size` == `size_of::<BootHandoffV1>() as u16`
- `mmap.ptr` must point at a readable buffer after ExitBootServices. Either:
  - point at the raw UEFI memmap buffer (current approach) and set `mmap.entry_size`/`mmap.entry_count` appropriately, or
  - convert firmware descriptors into a canonical MemoryRegion[] and point `mmap.ptr` there (preferred for portability).
- Kernel must validate `magic` and `version` on entry. If validation fails, print a clear message and stop.

Testing (local)
1. Build
   - cargo build --release --target x86_64-unknown-uefi
2. Create uefi image (FAT) and place BOOTX64.EFI + kernel ELF per repo script.
3. Run OVMF headless:
   - qemu-system-x86_64 -m 1024 \
     -drive if=pflash,format=raw,readonly,file=/usr/share/OVMF/OVMF_CODE.fd \
     -drive file=uefi.img,format=raw \
     -nographic -serial file:serial.log
4. Check `serial.log` for:
   - memory-map allocation and ExitBootServices success
   - BootHandoffV1 allocation and `magic`/`entry_point` lines (early kernel should print them)

CI / validation check
We will add a GitHub Actions job that:
  - Builds bootloader and kernel for the UEFI target.
  - Creates the FAT image and boots QEMU+OVMF headless.
  - Captures serial.log and greps for:
    - `ExitBootServices succeeded` (loader log)
    - `Transferring control to kernel` / `BOOT_HANDOFF validated` / kernel banner
Then contributors will be able to test as well.

Security & sensitive data
- Handoff does NOT verify capsules; that belongs in `verify::`.
- `verify::load_validated_capsule` must zero any secret material before returning control to loader/handoff. If you touch verify, use `zeroize` or volatile writes.
- Do not write private keys or signature blobs into BootHandoff pages.

PR / commit checklist
- Use commit prefix `handoff:` for changes in this directory.
- Ensure `size_of::<BootHandoffV1>()` static parity:
- Add a quick build-time check in both loader and kernel if possible.
- Add or update a QEMU validation check that demonstrates the handoff and kernel entry.
- Every `unsafe` has a one-line invariant comment.
- Add a line in the PR body describing how you manually tested (serial.log snippet or link to recorded run).
- Sign commits (`git commit -S`) for provenance.

If you change the ABI
- OPEN A COORDINATED PR that includes matching kernel changes.
- Add a migration plan in the PR body. Prefer supporting both old & new ABI for a short window if feasible.
- Update this CONTRIBUTING.md and list the ABI change in release notes.

Quick example commit message (copy/paste)
handoff: populate BootHandoffV1 and preserve memmap buffer for kernel consumption

What I expect reviewers to check
- ABI parity (struct sizes/offsets)
- memmap pointer points at valid memory after ExitBootServices
- safe handling/freeing on errors (do not free on success)
- test evidence in PR (serial.log or CI passing)

That’s it. Keep the code small and testable. 
