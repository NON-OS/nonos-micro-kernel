# CI evidence and release plumbing: benchmarks, security audits, hardware
# dossiers, the clean targets, cargo fmt, and the help text.

.PHONY: ci-fast ci-release ci-security ci-soak help nonos-mk-bench nonos-mk-bench-boot-log nonos-mk-bench-collect nonos-mk-bench-compare nonos-mk-bench-host nonos-mk-boot-evidence nonos-mk-claims-check nonos-mk-clean nonos-mk-clean-all nonos-mk-distclean nonos-mk-fmt nonos-mk-hardware-dossier nonos-mk-no-telemetry-capture nonos-mk-qemu-net-audit nonos-mk-release nonos-mk-release-audit nonos-mk-validate-machine-metadata

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
	@echo "Three ways to build:"
	@echo "  make                 the production image: full OS, all drivers, TPM,"
	@echo "                       STARK attestation, dual-signed, anti-rollback"
	@echo "  make qemu            build and boot the emulator cut under QEMU + OVMF"
	@echo "  make menuconfig      pick your own components, then: make from-config"
	@echo
	@echo "Also:"
	@echo "  make qemu-serial     headless boot, serial console to a log"
	@echo "  make nonos-mk-clean  remove build artefacts (add -all / distclean for more)"
	@echo "  make nonos-mk-fmt    cargo fmt across kernel + bootloader"
	@echo
	@echo "Everything the build does internally is still available as nonos-mk-*"
	@echo "targets in mk/*.mk; the three above are all most people need."
	@echo
	@echo "Environment:"
	@echo "  SIGNING_KEY=<path>   override signing key (default auto-gen)"
	@echo "  OVMF=<path>          override OVMF firmware discovery"

# ---------------------------------------------------------------------------
# Live GUI demo. One command boots the desktop image to a virtio-vga window
# with wallpaper, shell, and the SDK/App Kit apps. The capture variant boots
# headless and saves a framebuffer screenshot + a bounded serial log.
.PHONY: nonos-mk-run-gui-demo
nonos-mk-run-gui-demo: nonos-mk-run-nat
