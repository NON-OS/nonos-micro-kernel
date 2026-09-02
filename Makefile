# SPDX-License-Identifier: AGPL-3.0-or-later
#
# NONOS microkernel build.
#
# A capability-based, RAM-resident microkernel. The kernel and every capsule
# ship a transparent post-quantum STARK attestation, are dual-signed with
# Ed25519 and ML-DSA-65, and boot behind an anti-rollback floor measured into
# the TPM.
#
# Everything a human needs is on this page; `make help` prints it.
#
#     make              build the production image (the default)
#     make qemu         build it and boot under QEMU + OVMF + a software TPM
#     make usb          build the real-hardware image; DISK=/dev/... writes it
#     make menuconfig   choose your own components, then: make from-config
#     make test         the boot-and-verify harness CI gates on
#     make bench        measured performance, with provenance, never invented
#     make doctor       check this host has what the build and boot need
#     make clean        remove build artefacts    (fmt to format the tree)
#
# The build is split by concern into mk/*.mk, with per-capsule rules in
# userland/*/Capsule.mk. The nonos-mk-* targets those define are the internals
# CI drives; they all still work, they are just not the surface. This file is
# the curated top over them and defines nothing a person has to memorise.

# Per-user build configuration from `make menuconfig`. Optional: the leading
# dash keeps an absent file from being an error, so a fresh checkout builds.
-include .nonos-config

# Build concerns, included in numeric order (config, qemu, build, image, run,
# ci) so immediate `:=` assignments resolve exactly as in a single file.
include $(sort $(wildcard mk/*.mk))

# `make` with no target builds the shipping image, never nothing.
.DEFAULT_GOAL := nonos

.PHONY: nonos qemu qemu-serial usb hardware verify test bench doctor clean clean-all distclean fmt

# ── The image that ships ─────────────────────────────────────────────────────
# The full ZeroState system: every capsule and driver, TPM-measured boot, a
# transparent STARK attestation for the kernel and each capsule, dual Ed25519 +
# ML-DSA-65 signatures, and an anti-rollback floor. Fail-closed: without an
# enrolled identity it stops rather than shipping a forgeable one. NONOS_DEV=1
# mints a clearly marked throwaway identity for evaluation (mk/00-config.mk).
#
# The build does not end at packaging: it ends by verifying itself. Five
# independent checks (ledger, signatures, declared caps, STARK membership with
# the same gate the kernel enforces, root embedding) run against the artifacts
# just written, and the build receipt records the measured result. A build
# that cannot prove what it produced does not get to say it is ready.
nonos: nonos-mk-zerostate nonos-mk-esp nonos-mk-iso
	@$(MAKE) --no-print-directory nonos-mk-trust-ledger
	@$(MAKE) --no-print-directory nonos-mk-verify-image
	@echo
	@echo "  Production image ready, and proven:"
	@echo "    bootable ESP : $(ESP_DIR)"
	@echo "    ISO          : $(TARGET_DIR)/nonos.iso"
	@echo "    receipt      : $(TARGET_DIR)/attestation/build-receipt.json"
	@echo "    boot in QEMU : make qemu"
	@echo "    to hardware  : make usb DISK=/dev/..."

# ── Verify an already-built image ────────────────────────────────────────────
# The same five checks the default build ends with, standalone. Anyone holding
# the tree can re-run them and diff the receipt; that is the point.
verify: nonos-mk-verify-image

# ── The fast loop ────────────────────────────────────────────────────────────
# Proving is a release cost, never an iteration cost. One changed capsule
# invalidates every membership proof (the policy tree commits to the set), so
# the edit-compile-boot loop must not pay ten minutes of STARK grinding per
# keystroke. `make dev` rebuilds and signs what changed and boots with stale
# proofs tolerated in rollout mode, labelled as such at build and at every
# spawn. `make` stays the only path that proves; `make verify` will correctly
# refuse a dev tree, which is the system working.
dev: nonos-mk-dev
dev-qemu: nonos-mk-run-from-config
.PHONY: dev dev-qemu

# ── Boot it under emulation ──────────────────────────────────────────────────
# Same kernel, signing, attestation, and rollback path as the shipped ISO; the
# real-hardware drivers ride along and simply find no device. A software TPM
# (swtpm, CRB at 0xFED40000) backs the measured boot, so the attestation chain
# is exercised, not stubbed.
qemu: nonos-mk-run

# Headless desktop cut, serial console to a log: the profile CI's boot harness
# drives. Drops real-hardware-only drivers so the boot reaches ready under QEMU.
qemu-serial: nonos-mk-run-serial-log

# ── Boot it on real hardware ─────────────────────────────────────────────────
# A GPT-partitioned image firmware will boot from a stick, which an El Torito
# ISO is not dependable for. `make usb` builds it; add DISK=/dev/... to write
# it, behind a retype-the-path confirmation, because dd aimed at the wrong disk
# ends a machine. On macOS the raw device is used and the disk unmounted first;
# find yours with `diskutil list` (macOS) or `lsblk` (Linux).
usb: nonos-mk-usb-img
ifeq ($(strip $(DISK)),)
	@echo
	@echo "  Real-hardware image ready: $(USB_IMG)"
	@echo "  Write it with:  make usb DISK=/dev/diskN    (macOS: diskutil list)"
	@echo "                  make usb DISK=/dev/sdX      (Linux: lsblk)"
else
	@echo
	@echo "  About to OVERWRITE $(DISK) with $(USB_IMG)."
	@echo "  Everything on that disk is destroyed. This cannot be undone."
	@printf "  Type the disk path again to confirm: "; read confirm; \
	if [ "$$confirm" != "$(DISK)" ]; then echo "  Mismatch; not writing."; exit 1; fi; \
	if [ "$(UNAME_S)" = "Darwin" ]; then \
		diskutil unmountDisk $(DISK) || exit 1; \
		raw=$$(echo $(DISK) | sed 's#/dev/disk#/dev/rdisk#'); \
		echo "  Writing to $$raw (raw device)..."; \
		sudo dd if=$(USB_IMG) of=$$raw bs=4m && sync; \
		diskutil eject $(DISK); \
	else \
		echo "  Writing to $(DISK)..."; \
		sudo dd if=$(USB_IMG) of=$(DISK) bs=4M status=progress conv=fsync; \
	fi
	@echo "  Done. Boot the target machine from the stick."
endif

hardware: usb

# ── Verify ───────────────────────────────────────────────────────────────────
# The boot-and-verify harness: build, boot under QEMU, confirm the ready
# marker, the attestation count, and the trust chain. What CI gates on.
test: nonos-mk-test

# ── Measured performance ─────────────────────────────────────────────────────
# Real numbers only. Every run records host, commit, configuration, workload,
# iterations, and raw results next to the summary, and a QEMU number is a QEMU
# number, never presented as hardware. Harness: nonos-ci/bench_suite.py,
# knobs: NONOS_BENCH_* in mk/00-config.mk.
bench: nonos-mk-bench

# ── Host preflight ───────────────────────────────────────────────────────────
# Says whether this machine can build and boot NONOS before a long build finds
# out the hard way. Native hosts are macOS and Linux; on Windows use WSL2,
# which presents as Linux here and works unchanged.
doctor:
	@echo "  Host      : $(UNAME_S) $(UNAME_M)"
	@echo "  Toolchain : $(TOOLCHAIN)"
	@echo "  Jobs      : $(NONOS_JOBS) (cores $(NONOS_CORES), ram $(NONOS_RAM_GB) GiB)"
	@echo
	@ok=1; \
	for t in "$(CARGO)" "$(RUSTUP)" "$(QEMU)" "$(SWTPM)" xorriso mformat mcopy "$(NONOS_PYTHON)"; do \
		if command -v $$t >/dev/null 2>&1; then printf "  ok    %s\n" "$$t"; \
		else printf "  MISS  %s\n" "$$t"; ok=0; fi; \
	done; \
	if $(RUSTUP) toolchain list 2>/dev/null | grep -q "$(TOOLCHAIN)"; then \
		echo "  ok    rust $(TOOLCHAIN)"; \
	else echo "  MISS  rust $(TOOLCHAIN)   (rustup toolchain install $(TOOLCHAIN))"; ok=0; fi; \
	if [ -n "$(OVMF)" ] && [ -f "$(OVMF)" ]; then echo "  ok    OVMF $(OVMF)"; \
	else echo "  MISS  OVMF UEFI firmware"; ok=0; fi; \
	echo; \
	if [ $$ok = 1 ]; then echo "  This host can build and boot NONOS."; \
	else \
		echo "  Missing tools. Install with:"; \
		if [ "$(UNAME_S)" = "Darwin" ]; then \
			echo "    brew install qemu swtpm xorriso mtools"; \
		else \
			echo "    apt install qemu-system-x86 swtpm xorriso mtools   (or your distro's names)"; \
		fi; \
		echo "    curl https://sh.rustup.rs | sh    (then: rustup toolchain install $(TOOLCHAIN))"; \
		exit 1; \
	fi

# ── Housekeeping ─────────────────────────────────────────────────────────────
clean: nonos-mk-clean
clean-all: nonos-mk-clean-all
distclean: nonos-mk-distclean
fmt: nonos-mk-fmt
