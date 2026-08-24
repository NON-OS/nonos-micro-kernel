# Booting the image under QEMU (GUI, headless, serial, GDB, TPM), plus the
# static and verification gates that run over the built kernel.

.PHONY: nonos-mk-debug nonos-mk-plan-a-runtime nonos-mk-run nonos-mk-run-input-probe-inject-serial-log nonos-mk-run-nat nonos-mk-run-net nonos-mk-run-serial nonos-mk-run-serial-log nonos-mk-run-serial-nat nonos-mk-run-serial-net nonos-mk-check-caps nonos-mk-scan nonos-mk-static nonos-mk-swtpm-start nonos-mk-swtpm-stop nonos-mk-verify nonos-mk-verify-fast

# QEMU

$(QEMU_BLK_IMG):
	@mkdir -p $(dir $@)
	@truncate -s 64M $@

# The capsule package store lives at LBA 0 of the virtio-blk image instead of
# being baked into the kernel with include_bytes!. The image itself is an
# order-only prerequisite: its rule has no prerequisites, so a normal dep would
# re-pack on every mtime bump while still not ordering creation before packing.
QEMU_BLK_STORE_STAMP := $(QEMU_BLK_IMG).store.stamp

NONOS_MEDIA_DIR := media/samples
NONOS_MEDIA_FILES := $(wildcard $(NONOS_MEDIA_DIR)/*)

$(QEMU_BLK_STORE_STAMP): $(std-proof_ARTIFACTS) $(gui_demo_ARTIFACTS) $(game_2048_ARTIFACTS) $(egui_proof_ARTIFACTS) tools/nonos-store-pack $(NONOS_MEDIA_FILES) | $(QEMU_BLK_IMG)
	@$(NONOS_PYTHON) tools/nonos-store-pack --image $(QEMU_BLK_IMG) --lba 256 \
		--entry /capsules/std_proof.elf=$(std-proof_BIN) \
		--entry /capsules/std_proof.nonos_id_cert.bin=$(std-proof_CERT) \
		--entry /capsules/std_proof.manifest.bin=$(std-proof_MANIFEST) \
		--entry /capsules/std_proof.zk_trailer.bin=$(std-proof_ATTESTATION) \
		--entry /capsules/gui_demo.elf=$(gui_demo_BIN) \
		--entry /capsules/gui_demo.nonos_id_cert.bin=$(gui_demo_CERT) \
		--entry /capsules/gui_demo.manifest.bin=$(gui_demo_MANIFEST) \
		--entry /capsules/gui_demo.zk_trailer.bin=$(gui_demo_ATTESTATION) \
		--entry /capsules/game_2048.elf=$(game_2048_BIN) \
		--entry /capsules/game_2048.nonos_id_cert.bin=$(game_2048_CERT) \
		--entry /capsules/game_2048.manifest.bin=$(game_2048_MANIFEST) \
		--entry /capsules/game_2048.zk_trailer.bin=$(game_2048_ATTESTATION) \
		--entry /capsules/egui_proof.elf=$(egui_proof_BIN) \
		--entry /capsules/egui_proof.nonos_id_cert.bin=$(egui_proof_CERT) \
		--entry /capsules/egui_proof.manifest.bin=$(egui_proof_MANIFEST) \
		--entry /capsules/egui_proof.zk_trailer.bin=$(egui_proof_ATTESTATION) \
		--entry /Movies/big_buck_bunny.avi=$(NONOS_MEDIA_DIR)/big_buck_bunny.avi \
		--entry /Movies/blender_reel_2013.mp4=$(NONOS_MEDIA_DIR)/blender_reel_2013.mp4 \
		--entry /Movies/caminandes_llamigos.avi=$(NONOS_MEDIA_DIR)/caminandes_llamigos.avi \
		--entry /Movies/elephants_dream.avi=$(NONOS_MEDIA_DIR)/elephants_dream.avi \
		--entry /Movies/sintel.avi=$(NONOS_MEDIA_DIR)/sintel.avi \
		--entry /Movies/tears_of_steel.avi=$(NONOS_MEDIA_DIR)/tears_of_steel.avi
	@touch $@

# Declared in mk/20-build.mk; this only extends its prerequisites.
nonos-mk-run-from-config: $(QEMU_BLK_STORE_STAMP)

$(QEMU_OVMF_VARS_RW): $(OVMF_VARS)
	@mkdir -p $(dir $@)
	@[ -n "$(OVMF_VARS)" ] || { echo "::error::OVMF_VARS not found"; exit 1; }
	@cp "$(OVMF_VARS)" "$@"

# Mint a throwaway developer identity so a clean checkout can attest and boot.
# Public seed, no custody: never use the resulting image for production.
nonos-mk-dev-enroll:
	@test "$(NONOS_DEV)" = 1 || { echo "use: make NONOS_DEV=1 nonos-mk-dev-enroll"; exit 1; }
	@printf "\n  *** NONOS dev identity: public seed, no custody, not for production ***\n\n"
	@$(MAKE) --no-print-directory NONOS_DEV=1 $(ZK_BOOT_ROOT) $(ZK_BOOT_COMMITMENTS) $(ZK_BOOT_SECRETS)
	@printf "  dev boot identity ready: root %s\n\n" "$$(shasum -a 256 $(ZK_BOOT_ROOT) | cut -c1-16)"

# One command for a clean checkout: enroll a dev identity, then build and boot.
# The GOP preference pins the firmware mode to the QEMU window size so the
# cocoa window opens at a size that fits the host screen; hardware builds
# never set it and keep the largest-mode pick.
nonos-mk-dev-run:
	@$(MAKE) --no-print-directory NONOS_DEV=1 nonos-mk-dev-enroll
	@$(MAKE) --no-print-directory NONOS_DEV=1 \
		NONOS_GOP_PREF=$(QEMU_XRES)x$(QEMU_YRES) nonos-mk-run

nonos-mk-run: nonos-mk-swtpm-start nonos-mk-live-production-proof nonos-mk-zerostate nonos-mk-esp $(QEMU_BLK_IMG) $(QEMU_BLK_STORE_STAMP) $(QEMU_OVMF_VARS_RW)
	@echo "Booting NONOS in QEMU..."
	@echo "  Network: $(QEMU_NET_DESC)"
	@echo "  TPM: swtpm CRB"
	@echo "  Quit: Ctrl+A then X"
	@rm -f "$(QEMU_QMP_SOCK)"
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
		-drive if=pflash,format=raw,unit=0,readonly=on,file="$(OVMF)" \
		-drive if=pflash,format=raw,unit=1,file="$(QEMU_OVMF_VARS_RW)" \
		$(QEMU_BLK) $(QEMU_GPU) $(QEMU_NET) $(QEMU_USB) $(QEMU_RNG) $(QEMU_TPM) $(QEMU_AUDIO) \
		$(QEMU_QMP) \
		-serial mon:stdio -vga none -display $(QEMU_DISPLAY) -no-reboot

nonos-mk-run-net:
	@$(MAKE) --no-print-directory QEMU_NET_MODE=hostfwd nonos-mk-run

nonos-mk-run-nat:
	@$(MAKE) --no-print-directory QEMU_NET_MODE=nat nonos-mk-run

nonos-mk-swtpm-stop:
	@pkill -f "$(SWTPM)" 2>/dev/null || true

nonos-mk-swtpm-start: nonos-mk-swtpm-stop
	@command -v "$(SWTPM)" >/dev/null || { echo "swtpm not found (brew install swtpm)"; exit 1; }
	@rm -rf "$(SWTPM_STATE)"
	@mkdir -p "$(SWTPM_STATE)"
	@# Give the emulated part an endorsement key before the daemon takes the
	@# state directory. Without one every EK read answers TPM_RC_HANDLE, so the
	@# boot chain measures but never binds to a hardware identity, and the one
	@# path that matters most goes untested. Skipped rather than fatal when the
	@# tool is absent: measured boot still works, EK binding does not.
	@# --create-ek-cert is what makes swtpm_setup generate the key; the
	@# certificate it then tries to issue needs a local CA that a dev box has no
	@# reason to own, so that step is expected to fail and the key it already
	@# wrote is what matters. ReadPublic needs the key, not a certificate.
	@if command -v "$(SWTPM_SETUP)" >/dev/null; then \
		"$(SWTPM_SETUP)" --tpm2 --tpmstate "$(SWTPM_STATE)" \
			--create-ek-cert --overwrite --config /dev/null 2>&1 \
			| grep -q 'created RSA .* EK' \
			&& echo "  swtpm: endorsement key provisioned at 0x81010001" \
			|| echo "  swtpm: no endorsement key; EK binding will be skipped"; \
	else \
		echo "  swtpm_setup not found; EK binding will be skipped"; \
	fi
	@# swtpm_setup runs its own swtpm to author the state and normally tears it
	@# down, but the failing certificate step aborts it first and leaves that
	@# daemon holding the state lock. The real daemon below then cannot lock and
	@# the boot dies, so reap it by the pidfile only setup uses.
	@pkill -f 'swtpm_setup.pidfile' >/dev/null 2>&1 || true
	@while pgrep -f 'swtpm_setup.pidfile' >/dev/null 2>&1; do \
		perl -e 'select(undef,undef,undef,0.1)'; \
	done
	@rm -f "$(SWTPM_STATE)/.lock"
	@$(SWTPM) socket --tpm2 --tpmstate dir="$(SWTPM_STATE)" --ctrl type=unixio,path="$(SWTPM_SOCK)" --flags startup-clear --daemon
	@while [ ! -S "$(SWTPM_SOCK)" ]; do perl -e 'select(undef,undef,undef,0.1)'; done

nonos-mk-run-wizard: nonos-mk-setup-wizard-esp $(QEMU_BLK_IMG) $(QEMU_OVMF_VARS_RW)
	@echo "Booting NONOS (first-boot setup wizard) in QEMU..."
	@echo "  Network: $(QEMU_NET_DESC)"
	@echo "  Drive it with the host keyboard; Quit: Ctrl+A then X"
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(TARGET_DIR)/esp-setup-wizard" \
		-drive if=pflash,format=raw,unit=0,readonly=on,file="$(OVMF)" \
		-drive if=pflash,format=raw,unit=1,file="$(QEMU_OVMF_VARS_RW)" \
		$(QEMU_BLK) $(QEMU_GPU) $(QEMU_NET) $(QEMU_USB) $(QEMU_RNG) \
		-serial mon:stdio -vga none -no-reboot

nonos-mk-terminal-only-run: nonos-mk-terminal-only-prod nonos-mk-esp $(QEMU_OVMF_VARS_RW)
	@echo "Booting NONOS terminal-only in QEMU..."
	@echo "  Quit: Ctrl+A then X"
	@$(QEMU) -m 1G -cpu $(QEMU_CPU) -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
		-drive if=pflash,format=raw,unit=0,readonly=on,file="$(OVMF)" \
		-drive if=pflash,format=raw,unit=1,file="$(QEMU_OVMF_VARS_RW)" \
		$(QEMU_GPU) $(QEMU_RNG) \
		-serial mon:stdio -no-reboot

nonos-mk-run-serial: nonos-mk-desktop-gui-prod nonos-mk-esp $(QEMU_BLK_IMG) $(QEMU_BLK_STORE_STAMP)
	@echo "Booting NONOS serial console in QEMU..."
	@echo "  Network: $(QEMU_NET_DESC)"
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
		-drive if=pflash,format=raw,readonly=on,file="$(OVMF)" \
		$(QEMU_BLK) $(QEMU_GPU) $(QEMU_NET) $(QEMU_USB) $(QEMU_RNG) \
		-serial mon:stdio -display none -no-reboot

nonos-mk-run-serial-net:
	@$(MAKE) --no-print-directory QEMU_NET_MODE=hostfwd nonos-mk-run-serial

nonos-mk-run-serial-nat:
	@$(MAKE) --no-print-directory QEMU_NET_MODE=nat nonos-mk-run-serial

nonos-mk-run-serial-log: nonos-mk-desktop-gui-prod nonos-mk-esp $(QEMU_BLK_IMG) $(QEMU_BLK_STORE_STAMP)
	@mkdir -p $(dir $(QEMU_SERIAL_LOG))
	@echo "Booting NONOS serial console in QEMU..."
	@echo "  Network: $(QEMU_NET_DESC)"
	@echo "  Serial log: $(QEMU_SERIAL_LOG)"
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
		-drive if=pflash,format=raw,readonly=on,file="$(OVMF)" \
		$(QEMU_BLK) $(QEMU_GPU) $(QEMU_NET) $(QEMU_USB) $(QEMU_RNG) \
		-serial "file:$(QEMU_SERIAL_LOG)" -display none -no-reboot

nonos-mk-run-input-probe-inject-serial-log: nonos-mk-input-probe-inject-esp $(QEMU_BLK_IMG) $(QEMU_OVMF_VARS_RW)
	@mkdir -p $(dir $(QEMU_SERIAL_LOG))
	@echo "Booting NONOS input-probe inject serial console in QEMU..."
	@echo "  Network: $(QEMU_NET_DESC)"
	@echo "  Serial log: $(QEMU_SERIAL_LOG)"
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(NONOS_INPUT_PROBE_INJECT_ESP)" \
		-drive if=pflash,format=raw,readonly=on,file="$(OVMF)" \
		$(QEMU_BLK) $(QEMU_GPU) $(QEMU_NET) $(QEMU_USB) $(QEMU_RNG) \
		-serial "file:$(QEMU_SERIAL_LOG)" -display none -no-reboot

nonos-mk-debug: nonos-mk-desktop-gui-prod nonos-mk-esp
	@echo "QEMU listening for GDB on :1234   (gdb -ex 'target remote :1234')"
	@$(QEMU) -m $(QEMU_MEM) -cpu $(QEMU_CPU) -smp $(QEMU_SMP) -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
		-drive if=pflash,format=raw,readonly=on,file="$(OVMF)" \
		$(QEMU_NET) $(QEMU_RNG) \
		-serial mon:stdio -vga std -s -S -no-reboot

.PHONY: nonos-mk-tamper nonos-mk-tamper-run nonos-mk-tamper-restore nonos-mk-tamper-status
nonos-mk-tamper:
	@nonos-utils/tamper_kernel.sh tamper $(ESP_DIR)/EFI/nonos/kernel.bin

nonos-mk-tamper-restore:
	@nonos-utils/tamper_kernel.sh restore $(ESP_DIR)/EFI/nonos/kernel.bin

nonos-mk-tamper-status:
	@nonos-utils/tamper_kernel.sh status $(ESP_DIR)/EFI/nonos/kernel.bin

nonos-mk-tamper-run:
	@echo "Booting current ESP as-is (no rebuild)..."
	@$(QEMU) -m $(QEMU_MEM) -cpu $(QEMU_CPU) -smp $(QEMU_SMP) -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
		-drive if=pflash,format=raw,readonly=on,file="$(OVMF)" \
		$(QEMU_NET) $(QEMU_RNG) \
		-serial mon:stdio -vga std -no-reboot

# Boot-test harnesses

nonos-mk-boot-ramfs:
	@./tests/boot/ramfs_round_trip.sh

nonos-mk-boot-hda-playback:
	@./tests/boot/hda_playback.sh

nonos-mk-boot-audio-player:
	@./tests/boot/audio_player_playback.sh

nonos-mk-boot-keyring:
	@./tests/boot/keyring_round_trip.sh

nonos-mk-boot-entropy:
	@./tests/boot/entropy_round_trip.sh

nonos-mk-boot-crypto-hash:
	@./tests/boot/crypto_hash_round_trip.sh

nonos-mk-boot-vfs:
	@./tests/boot/vfs_round_trip.sh

nonos-mk-boot-ps2-input:
	@./tests/boot/ps2_input_round_trip.sh

nonos-mk-boot-xhci:
	@./tests/boot/xhci_round_trip.sh

nonos-mk-boot-usb-hid:
	@./tests/boot/usb_hid_enum.sh

nonos-mk-boot-input-e2e-ps2:
	@./tests/boot/input_e2e_ps2.sh

nonos-mk-boot-input-e2e-xhci:
	@./tests/boot/input_e2e_xhci.sh

nonos-mk-boot-desktop-gui:
	@./tests/boot/desktop_gui_boot.sh

nonos-mk-boot-ntp:
	@./tests/boot/ntp_round_trip.sh

.PHONY: nonos-mk-boot-image-viewer
nonos-mk-boot-image-viewer:
	@./tests/boot/image_viewer_round_trip.sh; rc=$$?; \
		echo "Restoring GUI image_viewer capsule (undo smoketest artifact)..."; \
		$(MAKE) -B nonos-mk-image-viewer-sign >/dev/null 2>&1; \
		exit $$rc

.PHONY: nonos-mk-boot-video-player
nonos-mk-boot-video-player:
	@./tests/boot/video_player.sh; rc=$$?; \
		echo "Restoring GUI video_player capsule (undo smoketest artifact)..."; \
		$(MAKE) -B nonos-mk-video-player-sign >/dev/null 2>&1; \
		exit $$rc

.PHONY: nonos-mk-pack-install-test
nonos-mk-pack-install-test: $(NONOS_PACK_BIN)
	@./tests/boot/pack_install_boot.sh

.PHONY: nonos-mk-boot-terminal
nonos-mk-boot-terminal:
	@./tests/boot/terminal_round_trip.sh; rc=$$?; \
		echo "Restoring GUI terminal capsule (undo autorun-selftest artifact)..."; \
		$(MAKE) -B nonos-mk-terminal-sign >/dev/null 2>&1; \
		exit $$rc

nonos-mk-plan-a-runtime: nonos-mk-desktop-gui-prod nonos-mk-esp $(QEMU_BLK_IMG) $(QEMU_BLK_STORE_STAMP) $(QEMU_OVMF_VARS_RW)
	@QEMU="$(QEMU)" OVMF="$(OVMF)" OVMF_VARS="$(OVMF_VARS)" \
		QEMU_OVMF_VARS_RW="$(QEMU_OVMF_VARS_RW)" QEMU_BLK_IMG="$(QEMU_BLK_IMG)" \
		ESP_DIR="$(ESP_DIR)" ./nonos-ci/plan-a-runtime.sh

# Verify

# Capability parity runs first and on its own. A capsule that declares a bit
# the kernel means differently is granted the wrong permission with every layer
# below agreeing, so this fails before anything is built or signed.
nonos-mk-check-caps:
	@$(NONOS_PYTHON) scripts/check_cap_parity.py

nonos-mk-static: nonos-mk-check-caps
	@./nonos-ci/run-static-checks.sh

MICROKERNEL_BIN := $(TARGET_DIR)/x86_64-nonos/release/nonos-kernel

# Patterns are matched against demangled `nm` output, so each entry is
# a qualified module-path fragment, not a raw substring. Raw substrings
# false-match the v0 mangling: `ext4` would hit `process::context::full`
# because the mangled name contains `7context4full`. The leading and
# trailing `::` anchors match the path separator emitted by --demangle.
MICROKERNEL_FORBIDDEN_SYMBOLS := \
  ::ext4:: ::fat32:: ::btrfs:: ::xfs:: ::f2fs:: ::squashfs:: ::overlayfs:: ::nfs:: \
  ::ahci:: ::nvme:: ::virtio_blk:: ::dm_crypt:: ::md_raid:: ::zswap:: ::hibernate:: \
  ::desktop:: ::graphics:: ::shell:: ::apps_service:: ::agents_service:: ::network_service::

nonos-mk-scan:
	@echo "Scanning microkernel image for legacy symbols..."
	@if [ ! -f "$(MICROKERNEL_BIN)" ]; then \
		echo "FAIL: microkernel binary not found at $(MICROKERNEL_BIN)"; \
		echo "      build first via 'make nonos-mk-capsules'"; \
		exit 1; \
	fi
	@dump=$$(mktemp); \
	if nm --demangle "$(MICROKERNEL_BIN)" >$$dump 2>/dev/null; then :; \
	else nm "$(MICROKERNEL_BIN)" 2>/dev/null >$$dump; fi; \
	fail=0; \
	for sym in $(MICROKERNEL_FORBIDDEN_SYMBOLS); do \
		hits=$$(grep -F "$$sym" $$dump \
			| grep -v 'syscall::contract::cap_table::graphics' \
			| grep -v 'syscall::dispatch::router::graphics' \
			| head -3); \
		if [ -n "$$hits" ]; then \
			echo "FAIL: image contains symbol matching '$$sym':"; \
			echo "$$hits"; \
			fail=1; \
		fi; \
	done; \
	rm -f $$dump; \
	if [ $$fail -ne 0 ]; then exit 1; fi
	@bash nonos-ci/scan-microkernel-symbols.sh "$(MICROKERNEL_BIN)"
	@echo "PASS: no legacy-tree symbols"

# Fast lane: static gates only, no kernel build.
nonos-mk-verify-fast: nonos-mk-static

# Full lane: static gates, then production desktop trust gates, then scan.
nonos-mk-verify: nonos-mk-static
	@$(MAKE) nonos-mk-verify-trust
	@$(MAKE) nonos-mk-scan

# Full test: verify + required QEMU boot harnesses.
nonos-mk-test: nonos-mk-verify nonos-mk-boot-ramfs nonos-mk-boot-keyring nonos-mk-boot-desktop-gui
