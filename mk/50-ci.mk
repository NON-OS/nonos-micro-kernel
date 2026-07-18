# CI-friendly security evidence checks.
nonos-mk-release-audit:
	@bash scripts/audit_release_profile.sh

nonos-mk-claims-check:
	@bash scripts/validate_claims_registry.sh

nonos-mk-qemu-net-audit:
	@bash scripts/audit_qemu_network_mode.sh

ci-fast:
	@RUSTUP_TOOLCHAIN=$(TOOLCHAIN) $(CARGO) test --manifest-path nonos-verify/Cargo.toml --tests
	@$(MAKE) nonos-mk-claims-check

ci-security: ci-fast nonos-mk-release-audit nonos-mk-qemu-net-audit

ci-release: ci-security
	@echo "ci-release: heavy reproducibility check is explicit; run scripts/repro_build_check.sh for release candidates"

nonos-mk-no-telemetry-capture:
	@bash scripts/no_telemetry_qemu_capture.sh

nonos-mk-boot-evidence:
	@bash scripts/qemu_boot_evidence.sh

nonos-mk-hardware-dossier:
	@test -n "$(NONOS_HW_SERIAL_LOG)" || { echo "NONOS_HW_SERIAL_LOG is required"; exit 1; }
	@NONOS_HW_OUT="$(NONOS_HW_OUT)" \
		NONOS_HW_SERIAL_LOG="$(NONOS_HW_SERIAL_LOG)" \
		NONOS_HW_MACHINE_JSON="$(NONOS_HW_MACHINE_JSON)" \
		NONOS_HW_BOOT_MEDIA="$(NONOS_HW_BOOT_MEDIA)" \
		NONOS_HW_ARTIFACTS="$(NONOS_HW_ARTIFACTS)" \
		$(NONOS_PYTHON) scripts/bare_metal_evidence.py

nonos-mk-validate-machine-metadata:
	@test -n "$(NONOS_HW_MACHINE_JSON)" || { echo "NONOS_HW_MACHINE_JSON is required"; exit 1; }
	@$(NONOS_PYTHON) scripts/validate_machine_metadata.py "$(NONOS_HW_MACHINE_JSON)"

nonos-mk-bench:
	@NONOS_BENCH_OUT="$(NONOS_BENCH_OUT)" \
		NONOS_BENCH_BUILD_CMD="$(NONOS_BENCH_BUILD_CMD)" \
		NONOS_BENCH_SKIP_BUILD="$(NONOS_BENCH_SKIP_BUILD)" \
		NONOS_BENCH_SKIP_BOOT="$(NONOS_BENCH_SKIP_BOOT)" \
		NONOS_BENCH_STRICT="$(NONOS_BENCH_STRICT)" \
		NONOS_BENCH_BOOT_TIMEOUT="$(NONOS_BENCH_BOOT_TIMEOUT)" \
		NONOS_BENCH_BOOT_CMD="$(NONOS_BENCH_BOOT_CMD)" \
		$(NONOS_PYTHON) nonos-ci/bench_suite.py

nonos-mk-bench-host:
	@$(NONOS_PYTHON) nonos-ci/bench_host.py "$(NONOS_BENCH_HOST_OUT)"

nonos-mk-bench-boot-log:
	@test -n "$(NONOS_BENCH_SERIAL_LOG)" || { echo "NONOS_BENCH_SERIAL_LOG is required"; exit 1; }
	@$(NONOS_PYTHON) nonos-ci/bench_boot_log.py "$(NONOS_BENCH_SERIAL_LOG)" \
		"$(NONOS_BENCH_BOOT_JSON)"

nonos-mk-bench-collect:
	@test -n "$(NONOS_BENCH_OUT)" || { echo "NONOS_BENCH_OUT is required"; exit 1; }
	@$(NONOS_PYTHON) nonos-ci/bench_collect.py "$(NONOS_BENCH_OUT)"

nonos-mk-bench-compare:
	@test -n "$(NONOS_BENCH_BASELINE)" || { echo "NONOS_BENCH_BASELINE is required"; exit 1; }
	@test -n "$(NONOS_BENCH_CANDIDATE)" || { echo "NONOS_BENCH_CANDIDATE is required"; exit 1; }
	@$(NONOS_PYTHON) nonos-ci/bench_compare.py "$(NONOS_BENCH_BASELINE)" \
		"$(NONOS_BENCH_CANDIDATE)" "$(NONOS_BENCH_REGRESSION_LIMIT)"

ci-soak:
	@echo "ci-soak: run nonos-mk-boot-evidence, nonos-mk-no-telemetry-capture, scripts/zero_state_forensic_qemu.sh, and hardware dossier collection"

# Release pipeline (paused)

nonos-mk-release:
	@echo
	@echo "release pipeline paused"
	@echo
	@echo "  The legacy 'release' / 'ci-release' / 'iso' / 'usb' / 'web-iso'"
	@echo "  targets produced the old monolithic-kernel artefacts and have"
	@echo "  been withdrawn from the active Makefile. A microkernel-shaped"
	@echo "  replacement is staged for a later CI step."
	@echo
	@echo "  Legacy recipes:  docs/legacy/Makefile.monolithic"
	@echo "  Status note:     docs/production-roadmap/master-execution-checklist.md"
	@echo
	@exit 1

# Clean

# Default `nonos-mk-clean` is kernel-only — userland artefacts survive
# so the next kernel build re-uses the existing capsule binaries.
nonos-mk-clean:
	@echo "Removing kernel build target (userland targets preserved)..."
	@rm -rf $(TARGET_DIR)/x86_64-nonos
	@rm -f $(TARGET_DIR)/kernel_signed.bin $(TARGET_DIR)/kernel_attested.bin

nonos-mk-clean-all:
	@echo "Cleaning kernel + bootloader + ESP..."
	@cd $(BOOTLOADER_DIR) && $(CARGO) clean 2>/dev/null || true
	@$(CARGO) clean 2>/dev/null || true
	@rm -rf $(TARGET_DIR)

nonos-mk-distclean: nonos-mk-clean-all
	@echo "Removing signing + ZK keys..."
	@rm -rf $(BOOTLOADER_DIR)/target target
	@rm -f $(SIGNING_KEY) $(KERNEL_MLDSA65_KEY) $(KERNEL_MLDSA65_PUB)
	@rm -rf $(ZK_KEYS_DIR)

nonos-mk-fmt:
	@RUSTUP_TOOLCHAIN=$(TOOLCHAIN) $(CARGO) fmt
	@cd $(BOOTLOADER_DIR) && RUSTUP_TOOLCHAIN=$(TOOLCHAIN) $(CARGO) fmt

# Help (default target)

help:
	@echo "NONOS microkernel build"
	@echo
	@echo "Configure your own kernel:"
	@echo "  make menuconfig               choose a profile and options, step by step"
	@echo "  make from-config              build the kernel described by .nonos-config"
	@echo
	@echo "Build (curated profiles):"
	@echo "  make nonos-mk                 microkernel-capsules runtime baseline"
	@echo "  make nonos-mk-core            kernel only (microkernel-core, no capsules)"
	@echo "  make nonos-mk-check           cargo check (microkernel-core)"
	@echo "  make nonos-mk-capsules        microkernel-capsules build"
	@echo "  make nonos-mk-zerostate       the full ZeroState system (all capsules + drivers, attested)"
	@echo "Userland capsules:"
	@echo "  make nonos-mk-libc nonos-mk-proof-io nonos-mk-ramfs nonos-mk-keyring"
	@echo "  make nonos-mk-userland-clean"
	@echo
	@echo "Sign / attest / package:"
	@echo "  make nonos-mk-sign            Ed25519 manifest signature"
	@echo "  make nonos-mk-attest          transparent attestation proof"
	@echo "  make nonos-mk-bootloader      UEFI bootloader"
	@echo "  make nonos-mk-esp             EFI System Partition for QEMU"
	@echo
	@echo "Run:"
	@echo "  make nonos-mk-run             full OS with TPM 2.0, NAT, signatures, ZK"
	@echo "  make nonos-mk-run-net         same image with explicit hostfwd network"
	@echo "  make nonos-mk-run-wizard      QEMU + OVMF, first-boot setup wizard"
	@echo "  make nonos-mk-run-serial      headless serial-only"
	@echo "  make nonos-mk-run-serial-nat  headless serial with outbound NAT network"
	@echo "  make nonos-mk-run-serial-net  headless serial with explicit hostfwd network"
	@echo "  make nonos-mk-run-serial-log  headless serial with file log"
	@echo "  make nonos-mk-debug           QEMU + GDB on :1234"
	@echo
	@echo "Verify:"
	@echo "  make nonos-mk-static          CI static gates"
	@echo "  make nonos-mk-scan            symbol scan over kernel image"
	@echo "  make nonos-mk-verify-fast     static gates only (no kernel build)"
	@echo "  make nonos-mk-verify          static gates + capsules build + scan"
	@echo "  make nonos-mk-release-audit   release-profile safety audit"
	@echo "  make nonos-mk-claims-check    claims registry schema/status check"
	@echo "  make nonos-mk-qemu-net-audit  QEMU default/no-NIC command audit"
	@echo "  make nonos-mk-no-telemetry-capture bounded QEMU packet-capture harness"
	@echo "  make nonos-mk-boot-evidence   bounded no-network QEMU serial boot evidence"
	@echo "  make nonos-mk-bench           host/build/boot benchmark evidence"
	@echo "  make nonos-mk-bench-host      host benchmark metadata only"
	@echo "  make nonos-mk-bench-compare   compare benchmark runs for regressions"
	@echo "  make nonos-mk-hardware-dossier parse physical serial log into dossier"
	@echo "  make nonos-mk-validate-machine-metadata check hardware metadata JSON"
	@echo "  make ci-fast                  host security tests + claims check"
	@echo "  make ci-security              ci-fast + release-profile audit"
	@echo
	@echo "Release:"
	@echo "  make nonos-mk-release         (paused; see message)"
	@echo "  make ci-release               release evidence gate (heavy repro explicit)"
	@echo "  make ci-soak                  prints soak/forensic harness guidance"
	@echo
	@echo "Clean:"
	@echo "  make nonos-mk-clean           kernel artefacts only (preserve userland)"
	@echo "  make nonos-mk-clean-all       kernel + bootloader + ESP"
	@echo "  make nonos-mk-distclean       above + signing + ZK keys"
	@echo
	@echo "Aux:"
	@echo "  make nonos-mk-toolchain       install nightly + components"
	@echo "  make nonos-mk-fmt             cargo fmt across kernel + bootloader"
	@echo
	@echo "Environment:"
	@echo "  SIGNING_KEY=<path>            override signing key (default auto-gen)"
	@echo "  OVMF=<path>                   override OVMF firmware discovery"

# Compatibility aliases (transitional; remove once callers migrate)

kernel-capsules:                       nonos-mk-capsules
kernel-with-keyring:                   nonos-mk-capsules
check-static:                          nonos-mk-static
clean-kernel-only:                     nonos-mk-clean
microkernel-symbol-scan:               nonos-mk-scan

# ---------------------------------------------------------------------------
# Live GUI demo. One command boots the desktop image to a virtio-vga window
# with wallpaper, shell, and the SDK/App Kit apps. The capture variant boots
# headless and saves a framebuffer screenshot + a bounded serial log.
.PHONY: nonos-mk-run-gui-demo
nonos-mk-run-gui-demo: nonos-mk-run-nat
