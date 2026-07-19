# SPDX-License-Identifier: AGPL-3.0-or-later
#
# NONOS microkernel build.
#
# A capability-based, RAM-resident microkernel. The kernel and every capsule
# ship a transparent post-quantum STARK attestation, are dual-signed with
# Ed25519 and ML-DSA-65, and boot behind an anti-rollback floor measured into
# the TPM.
#
# Three entry points, and nothing else to remember:
#
#     make             build the production image (the default)
#     make qemu        build and boot the emulator cut under QEMU
#     make menuconfig  choose your own components, then: make from-config
#
# The build is split by concern into mk/*.mk, with the per-capsule rules in
# userland/*/Capsule.mk. This file wires those together and defines the entry
# points above; `make help` lists the rest.

# Per-user build configuration from `make menuconfig`. Optional: the leading
# dash keeps an absent file from being an error, so a fresh checkout builds.
-include .nonos-config

# Build concerns, included in numeric order (config, qemu, build, image, run,
# ci) so immediate `:=` assignments resolve exactly as in a single file.
include $(sort $(wildcard mk/*.mk))

# `make` with no target builds the shipping image, never nothing.
.DEFAULT_GOAL := nonos

.PHONY: nonos qemu qemu-serial

# The image that ships. The full ZeroState system: every capsule and driver,
# TPM-measured boot, a transparent STARK attestation for the kernel and each
# capsule, dual Ed25519 + ML-DSA-65 signatures, and an anti-rollback floor.
nonos: nonos-mk-zerostate nonos-mk-esp nonos-mk-iso
	@echo
	@echo "Production image ready."
	@echo "  bootable ESP: $(ESP_DIR)"
	@echo "  ISO:          $(TARGET_DIR)/nonos.iso"
	@echo "  boot it:      make qemu"

# The ship image booted under QEMU + OVMF + a software TPM. Same kernel, same
# signing, attestation, and rollback path as the ISO; the real-hardware drivers
# ride along and simply find no device under emulation.
qemu: nonos-mk-run

# The desktop cut, headless, serial console to a log. This drops the
# real-hardware-only drivers so the boot reaches the ready marker under
# emulation; it is the profile CI's boot harness drives.
qemu-serial: nonos-mk-run-serial-log
