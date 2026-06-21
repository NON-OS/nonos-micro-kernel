```text
Read loader module CONTRIBUTING

This file describes how to work on the loader code (src/loader). Keep changes focused, testable and obvious.
The loader runs in early firmware — mistakes are expensive. Follow these rules.

What the loader is responsible for
- Validate capsule and extract the ELF payload (verify::load_validated_capsule).
- Parse ELF program headers (goblin) and collect PT_LOADs without relying on a heap.
- Coalesce PT_LOAD ranges and allocate a contiguous region for ET_EXEC (AllocateType::Address).
- Support ET_DYN fallback (AllocateType::AnyPages + relocation).
- Copy p_filesz and zero (p_memsz - p_filesz) for BSS areas.
- Record every allocate_pages call and free them on error (no firmware page leaks).
- Return KernelImage (address/size/entry/allocations) for handoff to consume.

Files & layout
- Preferred location: src/loader/loader.rs and src/loader/mod.rs (this file).

Coding rules
- Keep the hot path no_std and heapless: fixed-size tables (MAX_LOADS, MAX_ALLOCS).
- Minimize unsafe. Every unsafe must have a one‑line invariant comment above it.
- Use SystemTable-backed logging (log_info/log_error) — serial/console output is critical for debugging.
- Make error cases explicit with LoaderError variants and log actionable messages.
- Use checked/guarded arithmetic where sizes/addresses are computed.

ABI / compatibility
- Kernel expects KernelImage.entry_point to be an absolute address (ET_EXEC) or relocated address (ET_DYN).
- Do not change the public KernelImage fields without coordinating kernel changes.
- Add build-time checks if possible to assert size_of types between loader and kernel.

Tests & validation checks (local) We will add the files needed.
1. Build:
   cargo build --release --target x86_64-unknown-uefi
2. Create FAT uefi image following repo scripts (place BOOTX64.EFI and kernel ELF where loader expects).
3. Run OVMF headless:
   qemu-system-x86_64 -m 1024 \
     -drive if=pflash,format=raw,readonly,file=/usr/share/OVMF/OVMF_CODE.fd \
     -drive file=uefi.img,format=raw \
     -nographic -serial file:serial.log
4. Inspect serial.log for:
   - allocation lines from loader,
   - "Kernel loaded" line and kernel banner after the handoff.

CI / validation check
- Add a GitHub Actions job that:
  - Builds loader + kernel for x86_64 UEFI target.
  - Assembles uefi.img and boots QEMU+OVMF headless.
  - Greps serial.log for loader messages and kernel banner.

Review checklist (PRs touching loader)
- [ ] All changes are covered by a quick local validation check (serial.log attached).
- [ ] All unsafe blocks have invariant comments.
- [ ] LoaderError variants communicate actionable reasons.
- [ ] No heap usage in hot path (no Vec in loader.rs).
- [ ] KernelImage layout unchanged unless kernel change is included in the same PR.
- [ ] Commit message uses prefix `loader:` and explains rationale.
- [ ] Signed commit (`git commit -S`) recommended.
