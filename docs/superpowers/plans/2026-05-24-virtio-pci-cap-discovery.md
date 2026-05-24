# virtio-pci capability discovery — implementation plan

> **For agentic workers:** phased; the kernel boots + the gpu repro runs at each gate. Steps use `- [ ]`.

**Goal:** Let the virtio-gpu driver capsule find and map the *correct* virtio register BAR (not the VGA framebuffer) by having the kernel parse the device's virtio PCI capabilities and expose them, so the modern bring-up path works and the device finishes init.

**Architecture:** Mirror the existing MSI-X cap flow (`enumerate_capabilities → MsixInfo → PciHandle.msix → exposed`). Add a `VirtioPciCfg` parsed from the vendor caps (cfg_type 1/2/4 = common/notify/device), store it on `PciHandle`, surface it to the capsule, and rework the driver to use `common_cfg.bar+offset` for registers and `notify_cfg.bar+offset+multiplier` for the kick.

**Critical precondition:** capability discovery only works on a **modern** device. QEMU's `disable-modern=on` leaves a legacy-only device with **no** virtio caps. So the repro QEMU line must drop `disable-modern=on` (keep INTx, which we already support: `vectors=0` keeps it off MSI-X, or add MSI-X later). Confirm the device then exposes the modern caps.

**Tech stack:** `src/drivers/pci/capabilities/`, `src/drivers/pci/types/`, `src/hardware/broker/pci_index.rs` + `device.rs` (DeviceRecord wire ABI), a discovery syscall, `userland/capsule_driver_virtio_gpu/`.

---

### Phase 0 — confirm the modern device exposes caps
- [ ] Boot the gpu repro with `virtio-vga` **without** `disable-modern=on` (keep `vectors=0`). With the current (unchanged) driver, capture whether it still mis-detects; this only confirms the device now presents modern caps (kernel-side check next).
- [ ] Add a temporary kernel log in `enumerate_capabilities` for the gpu device dumping each cap's vendor/id; confirm the virtio vendor caps (cap_vndr 0x09, cfg_type 1/2/3/4) are present.
- [ ] **Gate:** virtio caps observed in the log → discovery is viable. If absent even without disable-modern, stop and reassess (device-model issue).

### Phase 1 — kernel: parse virtio caps
**Files:** `src/drivers/pci/types/` (new `VirtioPciCfg`), `src/drivers/pci/capabilities/parse.rs`.
- [ ] Define `VirtioPciCfg { common_bar, common_off, notify_bar, notify_off, notify_mult, device_bar, device_off, isr_bar, isr_off }` (offsets u32, bars u8).
- [ ] In the cap walker, for each vendor cap (0x09) read `cfg_type, bar, offset, length` (+ `notify_off_multiplier` for cfg_type 2) and fill the matching field.
- [ ] **Gate:** host check + target build clean; the Phase-0 log now also prints the parsed common/notify bar+offset.

### Phase 2 — kernel: store + expose
**Files:** `pci_index.rs` (+`virtio: Option<VirtioPciCfg>` on `PciHandle`), the PCI enumerate→install path, `device.rs` (DeviceRecord) + a discovery syscall.
- [ ] Add `virtio: Option<VirtioPciCfg>` to `PciHandle`; populate it where `msix` is populated.
- [ ] Expose to capsules: add a `MkVirtioCfg(device_id, claim_epoch, out_ptr)` syscall returning the `VirtioPciCfg` (mirrors the IrqBind out-pointer pattern), validated like the other broker syscalls.
- [ ] Add the `nonos-libc` binding + `VirtioCfgOut` struct.
- [ ] **Gate:** target build clean; repro unchanged (capsule not yet using it).

### Phase 3 — driver: consume caps, map the right BAR
**Files:** `userland/capsule_driver_virtio_gpu/src/setup/mmio.rs`, `discover.rs`, `init.rs`, `regs.rs`.
- [ ] In setup, call `mk_virtio_cfg`; map the BAR named by `common_bar` (not `first_register_bar`'s framebuffer guess); build `Regs` at `common_off`, notify at `notify_bar`+`notify_off` (apply multiplier).
- [ ] Use the modern bring-up path with the cap-derived offsets (replaces the hardcoded `MOD_*`); negotiate queue size down to `VQ_MAX_SIZE` via the writable modern `QUEUE_SIZE` (legacy's fixed 256-layout assumption stays satisfied).
- [ ] **Gate (PRIMARY):** repro shows `device claimed, scanouts=…`, no `device timeout` / `queue size out of range` / `claim failed`; the gpu driver stays up.

### Phase 4 — desktop + cleanup
- [ ] Boot the lean desktop with the modern GPU; confirm `[compositor] setup complete` → `BLIT ok` → **wallpaper on screen** (screendump).
- [ ] Revert all `GPUDBG`/`IRQDBG` debug markers and the forced-legacy stopgap.
- [ ] Decide MSI-X vs INTx for the GPU (INTx works today; MSI-X optional later).

## Risk / rollback
- Phases 1–2 are additive (kernel parses+exposes; nothing consumes yet) — revertable alone.
- Phase 3 is the driver cutover; rollback = revert it, kernel caps stay.
- DeviceRecord/syscall additions are a capsule ABI change — bump in lockstep with `nonos-libc`.
- If modern bring-up via caps still times out, the next suspect is DMA/queue address translation (IOMMU), not BAR discovery.
