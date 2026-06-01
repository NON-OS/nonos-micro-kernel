# NONOS microkernel build.
#
# Public targets: `nonos-mk-*`. `make` with no args prints the help
# table; no target builds silently. The old monolithic recipes (USB,
# ISO, VirtualBox, ci-release, web-iso, release) sit untouched in
# `docs/legacy/Makefile.monolithic`. `nonos-mk-release` exits
# non-zero until the microkernel release pipeline ships.
#
# Cheat sheet:
#   make nonos-mk              microkernel-capsules runtime baseline
#   make nonos-mk-run          QEMU + OVMF
#   make nonos-mk-boot-ramfs   ramfs capsule round trip
#   make nonos-mk-boot-keyring keyring capsule round trip
#   make nonos-mk-verify       static + trust gates + desktop build + symbol scan
#   make nonos-mk-test         verify + required boot harnesses
#
# A few old names (`kernel-capsules`, `kernel-with-keyring`, `boot-test`,
# etc.) forward to `nonos-mk-*` for transitional compatibility. See the
# bottom of the file.

.PHONY: help

# Public nonos-mk-* targets
.PHONY: nonos-mk
.PHONY: nonos-mk-check nonos-mk-check-ramfs-keys nonos-mk-core nonos-mk-capsules nonos-mk-driver-virtio-rng-test
.PHONY: nonos-mk-proof-io-prod nonos-mk-ramfs-prod nonos-mk-keyring-prod nonos-mk-entropy-prod nonos-mk-crypto-prod nonos-mk-vfs-prod nonos-mk-market-prod nonos-mk-driver-virtio-rng-prod nonos-mk-driver-virtio-blk-prod nonos-mk-driver-virtio-gpu-prod nonos-mk-driver-virtio-net-prod nonos-mk-driver-iwlwifi-prod nonos-mk-driver-i2c-pci-prod nonos-mk-driver-i2c-hid-prod nonos-mk-driver-ps2-input-prod nonos-mk-driver-xhci-prod nonos-mk-driver-usb-hid-prod nonos-mk-driver-usb-msc-prod nonos-mk-driver-e1000-prod nonos-mk-driver-rtl8139-prod nonos-mk-driver-rtl8169-prod nonos-mk-driver-ahci-prod nonos-mk-driver-hda-prod nonos-mk-driver-nvme-prod nonos-mk-net-l2-prod nonos-mk-net-ip-prod nonos-mk-net-udp-prod nonos-mk-net-dhcp-prod nonos-mk-net-nym-prod nonos-mk-net-sockets-prod nonos-mk-desktop-gui-prod
.PHONY: nonos-mk-ramfs-test nonos-mk-keyring-test nonos-mk-entropy-test nonos-mk-crypto-hash-test nonos-mk-vfs-test nonos-mk-market-test nonos-mk-market-smoke nonos-mk-market-fixtures nonos-mk-driver-virtio-blk-test nonos-mk-virtio-blk-test-image nonos-mk-driver-virtio-net-test nonos-mk-driver-ps2-input-test nonos-mk-driver-xhci-test nonos-mk-wallpaper-test
.PHONY: nonos-mk-libc nonos-mk-proof-io nonos-mk-proof-io-sign nonos-mk-check-trust-keys nonos-mk-check-trust-manifest nonos-mk-trust-policy nonos-mk-host-trust-test nonos-mk-verify-trust nonos-mk-ramfs nonos-mk-ramfs-sign nonos-mk-keyring nonos-mk-entropy nonos-mk-crypto nonos-mk-vfs nonos-mk-virtio-rng nonos-mk-virtio-rng-sign nonos-mk-check-virtio-rng-keys nonos-mk-virtio-blk nonos-mk-virtio-blk-sign nonos-mk-check-virtio-blk-keys nonos-mk-driver-virtio-gpu nonos-mk-driver-virtio-gpu-sign nonos-mk-check-driver-virtio-gpu-keys nonos-mk-virtio-net nonos-mk-virtio-net-sign nonos-mk-check-virtio-net-keys nonos-mk-driver-iwlwifi nonos-mk-driver-iwlwifi-sign nonos-mk-check-driver-iwlwifi-keys nonos-mk-driver-i2c-pci nonos-mk-driver-i2c-pci-sign nonos-mk-check-driver-i2c-pci-keys nonos-mk-driver-i2c-hid nonos-mk-driver-i2c-hid-sign nonos-mk-check-driver-i2c-hid-keys nonos-mk-ps2-input nonos-mk-ps2-input-sign nonos-mk-check-ps2-input-keys nonos-mk-xhci nonos-mk-xhci-sign nonos-mk-check-xhci-keys nonos-mk-driver-usb-msc nonos-mk-driver-usb-msc-sign nonos-mk-check-driver-usb-msc-keys nonos-mk-driver-e1000 nonos-mk-driver-e1000-sign nonos-mk-check-driver-e1000-keys nonos-mk-driver-rtl8139 nonos-mk-driver-rtl8139-sign nonos-mk-check-driver-rtl8139-keys nonos-mk-driver-rtl8169 nonos-mk-driver-rtl8169-sign nonos-mk-check-driver-rtl8169-keys nonos-mk-driver-ahci nonos-mk-driver-ahci-sign nonos-mk-check-driver-ahci-keys nonos-mk-driver-hda nonos-mk-driver-hda-sign nonos-mk-check-driver-hda-keys nonos-mk-driver-nvme nonos-mk-driver-nvme-sign nonos-mk-check-driver-nvme-keys nonos-mk-wallpaper nonos-mk-marketplace-abi nonos-mk-market nonos-mk-marketplace-index-tool
.PHONY: nonos-mk-userland-clean
.PHONY: nonos-mk-bootloader nonos-mk-sign nonos-mk-attest nonos-mk-esp
.PHONY: nonos-mk-run nonos-mk-run-serial nonos-mk-debug nonos-mk-plan-a-runtime
.PHONY: nonos-mk-boot-ramfs nonos-mk-boot-keyring nonos-mk-boot-entropy nonos-mk-boot-crypto-hash nonos-mk-boot-vfs nonos-mk-boot-ps2-input nonos-mk-boot-xhci nonos-mk-boot-desktop-gui
.PHONY: nonos-mk-input-e2e-ps2-test nonos-mk-input-e2e-ps2-esp nonos-mk-input-e2e-xhci-test nonos-mk-input-e2e-xhci-esp nonos-mk-boot-input-e2e-ps2 nonos-mk-boot-input-e2e-xhci nonos-mk-input-probe-test nonos-mk-input-probe-esp nonos-mk-input-probe-run nonos-mk-input-probe-run-serial nonos-mk-input-probe-inject-test
.PHONY: nonos-mk-static nonos-mk-scan
.PHONY: nonos-mk-verify nonos-mk-verify-fast
.PHONY: nonos-mk-test nonos-mk-host-test
.PHONY: nonos-mk-release
.PHONY: nonos-mk-clean nonos-mk-clean-all nonos-mk-distclean
.PHONY: nonos-mk-fmt
.PHONY: nonos-mk-toolchain nonos-mk-check-deps
.PHONY: nonos-mk-ensure-signing-key nonos-mk-ensure-zk-keys nonos-mk-zk-tools

# Compatibility aliases (transitional, do not remove until callers move)
.PHONY: kernel-capsules kernel-keyring-smoketest kernel-ramfs-smoketest
.PHONY: kernel-with-keyring kernel-microkernel-keyring-smoketest
.PHONY: boot-test ramfs-boot-test
.PHONY: check-static clean-kernel-only microkernel-symbol-scan

# Default target: print help, never build silently.

.DEFAULT_GOAL := help

# Configuration

BOOTLOADER_DIR := nonos-bootloader
TARGET_DIR     := target
ESP_DIR        := $(TARGET_DIR)/esp
KEYS_DIR       := $(BOOTLOADER_DIR)/keys

export SOURCE_DATE_EPOCH ?= $(shell git log -1 --format=%ct 2>/dev/null || date +%s)
export CARGO_INCREMENTAL := 0

export PATH := $(HOME)/.cargo/bin:$(PATH)
TOOLCHAIN := nightly-2026-01-16
CARGO     := $(HOME)/.cargo/bin/cargo
RUSTUP    := $(HOME)/.cargo/bin/rustup

UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)

ifeq ($(UNAME_S),Darwin)
    ifeq ($(UNAME_M),arm64)
        HOST_TARGET := aarch64-apple-darwin
    else
        HOST_TARGET := x86_64-apple-darwin
    endif
    SDK_PATH   := /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk
    SDK_EXISTS := $(shell test -d $(SDK_PATH) && echo yes)
    ifeq ($(SDK_EXISTS),yes)
        SDK_FLAGS := SDKROOT=$(SDK_PATH) \
                     AR=/Library/Developer/CommandLineTools/usr/bin/ar \
                     CC=/Library/Developer/CommandLineTools/usr/bin/clang \
                     PATH="/Library/Developer/CommandLineTools/usr/bin:$$PATH"
    else
        SDK_FLAGS :=
    endif
    SHA256 := shasum -a 256
else ifeq ($(UNAME_S),Linux)
    HOST_TARGET := x86_64-unknown-linux-gnu
    SDK_FLAGS   :=
    SHA256      := sha256sum
else
    $(error Unsupported host platform: $(UNAME_S))
endif

# Signing key. Auto-generated on first use.
SIGNING_KEY ?= $(KEYS_DIR)/signing_key_v1.bin

# ZK ceremony paths.
ZK_CIRCUIT_DIR   := $(BOOTLOADER_DIR)/tools/nonos-attestation-circuit
ZK_KEYS_DIR      := $(ZK_CIRCUIT_DIR)/generated_keys
ZK_PROVING_KEY   := $(ZK_KEYS_DIR)/attestation_proving_key.bin
ZK_VERIFYING_KEY := $(ZK_KEYS_DIR)/attestation_verifying_key.bin
ZK_KEY_SEED      := nonos-production-attestation-v1-2026
ZK_TOOL          := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/generate-keys
EMBED_TOOL       := $(BOOTLOADER_DIR)/tools/embed-zk-proof/target/$(HOST_TARGET)/release/embed-zk-proof

# QEMU + OVMF discovery.
QEMU := qemu-system-x86_64
ifeq ($(UNAME_S),Darwin)
    OVMF ?= $(shell \
        if [ -f firmware/OVMF.fd ]; then echo firmware/OVMF.fd; \
        elif [ -f /opt/homebrew/share/qemu/edk2-x86_64-code.fd ]; then echo /opt/homebrew/share/qemu/edk2-x86_64-code.fd; \
        elif [ -f /usr/local/share/qemu/edk2-x86_64-code.fd ]; then echo /usr/local/share/qemu/edk2-x86_64-code.fd; \
        fi)
    OVMF_VARS ?= $(shell \
        if [ -f firmware/OVMF_VARS.fd ]; then echo firmware/OVMF_VARS.fd; \
        elif [ -f /opt/homebrew/share/qemu/edk2-x86_64-vars.fd ]; then echo /opt/homebrew/share/qemu/edk2-x86_64-vars.fd; \
        elif [ -f /usr/local/share/qemu/edk2-x86_64-vars.fd ]; then echo /usr/local/share/qemu/edk2-x86_64-vars.fd; \
        elif [ -f /opt/homebrew/share/qemu/edk2-i386-vars.fd ]; then echo /opt/homebrew/share/qemu/edk2-i386-vars.fd; \
        elif [ -f /usr/local/share/qemu/edk2-i386-vars.fd ]; then echo /usr/local/share/qemu/edk2-i386-vars.fd; \
        fi)
else
    # Walk the candidate list shipped by the major Linux distros.
    OVMF ?= $(shell \
        for f in \
            /usr/share/OVMF/OVMF_CODE_4M.fd \
            /usr/share/OVMF/OVMF_CODE.fd \
            /usr/share/qemu/OVMF.fd \
            /usr/share/ovmf/OVMF.fd \
            /usr/share/edk2-ovmf/x64/OVMF_CODE.fd \
            /usr/share/edk2/ovmf/OVMF_CODE.fd ; do \
            if [ -r $$f ]; then echo $$f; exit 0; fi; \
        done)
    OVMF_VARS ?= $(shell \
        for f in \
            /usr/share/OVMF/OVMF_VARS_4M.fd \
            /usr/share/OVMF/OVMF_VARS.fd \
            /usr/share/qemu/OVMF_VARS.fd \
            /usr/share/ovmf/OVMF_VARS.fd \
            /usr/share/edk2-ovmf/x64/OVMF_VARS.fd \
            /usr/share/edk2/ovmf/OVMF_VARS.fd ; do \
            if [ -r $$f ]; then echo $$f; exit 0; fi; \
        done)
endif

QEMU_MEM := 2G
QEMU_CPU := max
QEMU_SMP := 2
QEMU_HOST_SSH_PORT ?= 2222
QEMU_HOST_HTTP_PORT ?= 8080
QEMU_BLK_IMG := $(TARGET_DIR)/qemu-virtio-blk.img
QEMU_OVMF_VARS_RW := $(TARGET_DIR)/qemu-OVMF_VARS.fd
QEMU_NET := -device virtio-net-pci,netdev=net0 -netdev user,id=net0,hostfwd=tcp::$(QEMU_HOST_SSH_PORT)-:22,hostfwd=tcp::$(QEMU_HOST_HTTP_PORT)-:80
QEMU_BLK := -drive "file=$(QEMU_BLK_IMG),if=none,id=vd0,format=raw" -device virtio-blk-pci,drive=vd0
QEMU_GPU := -device virtio-vga,disable-modern=on,vectors=0,xres=1024,yres=768
QEMU_USB := -device qemu-xhci,id=xhci -device usb-tablet,bus=xhci.0
QEMU_RNG := -device virtio-rng-pci

VERSION ?= $(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")

# Top-level: nonos-mk = build the microkernel-capsules runtime baseline.

nonos-mk: nonos-mk-capsules
	@echo
	@echo "Built microkernel-capsules ($(VERSION))."
	@echo "  make nonos-mk-esp           package the ESP for QEMU"
	@echo "  make nonos-mk-run           boot under QEMU + OVMF"
	@echo "  make nonos-mk-verify        static gates + symbol scan"
	@echo "  make nonos-mk-test          verify + both boot harnesses"

# Toolchain + key bootstrap

nonos-mk-toolchain: $(TARGET_DIR)/.nonos-toolchain.stamp

nonos-mk-check-deps: $(TARGET_DIR)/.nonos-toolchain.stamp

$(SIGNING_KEY):
	@echo "Generating signing key (Ed25519 seed)..."
	@mkdir -p $(KEYS_DIR)
	@head -c 32 /dev/urandom > $@
	@echo "Wrote $@"

nonos-mk-ensure-signing-key: $(SIGNING_KEY)

# ZK attestation: ceremony tools + ceremony keys + embed tool

$(ZK_TOOL): nonos-mk-check-deps
	@echo "Building ZK attestation tools..."
	@cd $(ZK_CIRCUIT_DIR) && RUSTFLAGS="" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --bin generate-keys --bin generate-proof --target $(HOST_TARGET)

$(ZK_PROVING_KEY): $(ZK_TOOL) $(SIGNING_KEY)
	@echo "Running trusted setup for ZK circuit..."
	@mkdir -p $(ZK_KEYS_DIR)
	@$(ZK_TOOL) generate --output $(ZK_KEYS_DIR) --seed "$(ZK_KEY_SEED)" --sign-key $(SIGNING_KEY) --print-program-hash
	@cp $(ZK_VERIFYING_KEY) $(ZK_KEYS_DIR)/vk_attestation_program.bin
	@cp $(ZK_VERIFYING_KEY) $(ZK_KEYS_DIR)/vk_boot_authority.bin
	@cp $(ZK_VERIFYING_KEY) $(ZK_KEYS_DIR)/vk_update_authority.bin
	@cp $(ZK_VERIFYING_KEY) $(ZK_KEYS_DIR)/vk_recovery_key.bin

$(ZK_VERIFYING_KEY): $(ZK_PROVING_KEY)

nonos-mk-ensure-zk-keys: $(ZK_PROVING_KEY) $(ZK_VERIFYING_KEY)
nonos-mk-zk-tools: $(ZK_TOOL)

$(EMBED_TOOL): nonos-mk-check-deps
	@echo "Building ZK embed tool..."
	@cd $(BOOTLOADER_DIR)/tools/embed-zk-proof && RUSTFLAGS="" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --target $(HOST_TARGET)

# Bootloader
#
# The .efi target is a real file; it must depend only on real
# input files so make's mtime check can short-circuit re-entry.
# Phony prerequisites would force `cargo build` on every smoke
# run even when no source has changed. Toolchain and key bootstrap
# are tracked via real stamp/key files instead.

BOOTLOADER_SRCS := $(shell find $(BOOTLOADER_DIR)/src -type f -name '*.rs' 2>/dev/null) \
                   $(BOOTLOADER_DIR)/Cargo.toml \
                   $(BOOTLOADER_DIR)/Cargo.lock \
                   $(BOOTLOADER_DIR)/build.rs

$(TARGET_DIR)/.nonos-toolchain.stamp:
	@mkdir -p $(TARGET_DIR)
	@test -f $(RUSTUP) || { echo "rustup not found. Install from https://rustup.rs"; exit 1; }
	@$(RUSTUP) toolchain install $(TOOLCHAIN) 2>/dev/null || true
	@$(RUSTUP) target add x86_64-unknown-uefi --toolchain $(TOOLCHAIN) 2>/dev/null || true
	@$(RUSTUP) component add rust-src clippy rustfmt --toolchain $(TOOLCHAIN) 2>/dev/null || true
	@touch $@

$(BOOTLOADER_DIR)/target/x86_64-unknown-uefi/release/nonos_boot.efi: \
		$(BOOTLOADER_SRCS) \
		$(SIGNING_KEY) \
		$(ZK_PROVING_KEY) \
		$(ZK_VERIFYING_KEY) \
		$(TARGET_DIR)/.nonos-toolchain.stamp
	@echo "Building UEFI bootloader..."
	$(eval SIGNING_KEY_ABS := $(if $(filter /%,$(SIGNING_KEY)),$(SIGNING_KEY),$(shell pwd)/$(SIGNING_KEY)))
	@cd $(BOOTLOADER_DIR) && \
		NONOS_SIGNING_KEY=$(SIGNING_KEY_ABS) \
		NONOS_ZK_CEREMONY_DIR=$(shell pwd)/$(ZK_KEYS_DIR) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --target x86_64-unknown-uefi --release --features zk-groth16

nonos-mk-bootloader: $(BOOTLOADER_DIR)/target/x86_64-unknown-uefi/release/nonos_boot.efi

# Userland capsules

USERLAND_DIR  := userland
USERLAND_LIBC := $(USERLAND_DIR)/libc/target/x86_64-nonos-user/release/libnonos_libc.a
USERLAND_LIBC_SRCS := $(shell find $(USERLAND_DIR)/libc/src -name '*.rs') \
                      $(USERLAND_DIR)/libc/Cargo.toml \
                      $(USERLAND_DIR)/x86_64-nonos-user.json

# Trust-anchor inputs — global to all signed capsules. The two
# pubkeys (Ed25519 + ML-DSA-65) are sealed into a policy blob and
# baked into the kernel via `BAKED_TRUST_ANCHOR_POLICY`. Capsule
# certs and manifests are signed under this policy.
#
# Layout split:
#   .keys/                   gitignored; holds *.seed only
#   nonos-data/trust/keys/   committed; holds *.pub
#   nonos-data/trust/policy/ committed; holds the sealed policy
NONOS_TRUST_DIR         := nonos-data/trust
NONOS_TA_ED25519_SEED   := .keys/nonos_trust_anchor_ed25519.seed
NONOS_TA_ED25519_PUB    := $(NONOS_TRUST_DIR)/keys/nonos_trust_anchor_ed25519.pub
NONOS_TA_MLDSA65_SEED   := .keys/nonos_trust_anchor_mldsa65.seed
NONOS_TA_MLDSA65_PUB    := $(NONOS_TRUST_DIR)/keys/nonos_trust_anchor_mldsa65.pub
NONOS_TRUST_ANCHOR_POLICY_BIN := $(NONOS_TRUST_DIR)/policy/nonos_trust_anchor.policy.bin

# Trust-anchor epoch and the validity window every capsule cert
# inherits. Capsule.mk may override the cert serial / validity per
# capsule, but the policy window is one shared source of truth.
NONOS_TRUST_ANCHOR_EPOCH  := 1
NONOS_CERT_VALID_FROM_MS  := 1767225600000
NONOS_CERT_VALID_UNTIL_MS := 1893456000000

CAPSULE_SIGN_BIN := nonos-sign/target/release/capsule-sign

$(USERLAND_LIBC): $(USERLAND_LIBC_SRCS)
	@echo "Building userland libc..."
	@cd $(USERLAND_DIR)/libc && \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --target ../x86_64-nonos-user.json \
		-Zbuild-std=core
	@touch $@

nonos-mk-libc: $(USERLAND_LIBC)

$(CAPSULE_SIGN_BIN):
	@echo "Building capsule-sign host tool..."
	@cd nonos-sign && cargo build --release --bin capsule-sign

nonos-mk-host-trust-test:
	@echo "Running host trust chain integration tests..."
	@cd nonos-sign && cargo test --release --test host_trust
	@echo "Verifying on-disk capsule artifacts against baked policy..."
	@cd nonos-sign && cargo test --release --test artifacts

nonos-mk-check-trust-manifest:
	@echo "Verifying baked trust artifact SHA-256 ledger..."
	@cd $(NONOS_TRUST_DIR) && $(SHA256) -c MANIFEST.sha256

nonos-mk-verify-trust: nonos-mk-desktop-gui-prod
	@$(MAKE) nonos-mk-host-trust-test
	@$(MAKE) nonos-mk-check-trust-manifest

nonos-mk-check-trust-keys:
	@for f in $(NONOS_TA_ED25519_SEED) $(NONOS_TA_ED25519_PUB) \
	          $(NONOS_TA_MLDSA65_SEED) $(NONOS_TA_MLDSA65_PUB); do \
	    [ -f "$$f" ] || { \
	        echo "::error::missing $$f — generate via: $(CAPSULE_SIGN_BIN) keygen --alg <ed25519|mldsa65> --out <prefix>"; \
	        exit 1; \
	    }; \
	done

$(NONOS_TRUST_ANCHOR_POLICY_BIN): $(NONOS_TA_ED25519_PUB) $(NONOS_TA_MLDSA65_PUB) $(CAPSULE_SIGN_BIN) | nonos-mk-check-trust-keys
	@echo "Sealing NØNOS trust-anchor policy (Ed25519 + ML-DSA-65)..."
	@$(CAPSULE_SIGN_BIN) mk-trust-policy \
		--epoch $(NONOS_TRUST_ANCHOR_EPOCH) \
		--ta-pub ed25519=$(NONOS_TA_ED25519_PUB) \
		--ta-pub mldsa65=$(NONOS_TA_MLDSA65_PUB) \
		--valid-from-ms  $(NONOS_CERT_VALID_FROM_MS) \
		--valid-until-ms $(NONOS_CERT_VALID_UNTIL_MS) \
		--out $@

nonos-mk-trust-policy: $(NONOS_TRUST_ANCHOR_POLICY_BIN)

# Per-capsule build, cert, and manifest rules live in each
# `userland/<capsule>/Capsule.mk`. Including the file pulls the
# capsule's metadata into scope and fires `nonos-mk/capsule.mk`
# which materialises the standard target set:
#   nonos-mk-<slug>           build the userland ELF
#   nonos-mk-<slug>-sign      sign cert + manifest
#   nonos-mk-check-<slug>-keys assert publisher seeds + pubs exist
include userland/capsule_proof_io/Capsule.mk
include userland/capsule_ramfs/Capsule.mk
include userland/capsule_keyring/Capsule.mk
include userland/capsule_entropy/Capsule.mk
include userland/capsule_crypto/Capsule.mk
include userland/compositor/Capsule.mk
include userland/capsule_input_router/Capsule.mk
include userland/capsule_input_proof/Capsule.mk
include userland/capsule_input_probe/Capsule.mk
include userland/capsule_wm/Capsule.mk
include userland/capsule_desktop_shell/Capsule.mk
include userland/capsule_image_codec/Capsule.mk
include userland/capsule_clipboard/Capsule.mk
include userland/capsule_login/Capsule.mk
include userland/toolkit/Capsule.mk
include userland/capsule_about/Capsule.mk
include userland/capsule_calculator/Capsule.mk
include userland/capsule_terminal/Capsule.mk
include userland/capsule_file_manager/Capsule.mk
include userland/capsule_text_editor/Capsule.mk
include userland/capsule_policy/Capsule.mk
include userland/capsule_wallpaper_catalog/Capsule.mk
include userland/capsule_settings/Capsule.mk
include userland/capsule_process_manager/Capsule.mk
include userland/capsule_vfs/Capsule.mk
include userland/capsule_market/Capsule.mk
include userland/capsule_driver_virtio_rng/Capsule.mk
include userland/capsule_driver_ps2_input/Capsule.mk
include userland/capsule_driver_virtio_blk/Capsule.mk
include userland/capsule_driver_virtio_gpu/Capsule.mk
include userland/capsule_driver_virtio_net/Capsule.mk
include userland/capsule_driver_iwlwifi/Capsule.mk
include userland/capsule_driver_i2c_pci/Capsule.mk
include userland/capsule_driver_i2c_hid/Capsule.mk
include userland/capsule_driver_xhci/Capsule.mk
include userland/capsule_driver_usb_hid/Capsule.mk
include userland/capsule_driver_usb_msc/Capsule.mk
include userland/capsule_driver_e1000/Capsule.mk
include userland/capsule_driver_rtl8139/Capsule.mk
include userland/capsule_driver_rtl8169/Capsule.mk
include userland/capsule_driver_ahci/Capsule.mk
include userland/capsule_driver_hda/Capsule.mk
include userland/capsule_driver_nvme/Capsule.mk
include userland/capsule_net_l2/Capsule.mk
include userland/capsule_net_ip/Capsule.mk
include userland/capsule_net_udp/Capsule.mk
include userland/capsule_net_dhcp/Capsule.mk
include userland/capsule_net_tcp/Capsule.mk
include userland/capsule_net_dns/Capsule.mk
include userland/capsule_net_sockets/Capsule.mk
include userland/capsule_net_nym/Capsule.mk
include userland/capsule_wallpaper/Capsule.mk
include userland/capsule_attest/Capsule.mk
include userland/capsule_power/Capsule.mk

# Orchestration helper: union of every verified capsule's artifact
# triple. Smoke and test targets that need proof_io plus another
# capsule depend on `$(proof-io_ARTIFACTS)` directly.
NONOS_VERIFIED_ARTIFACTS = $(foreach slug,$(NONOS_VERIFIED_CAPSULES),$($(slug)_ARTIFACTS))

NONOS_DESKTOP_GUI_CAPSULE_CHECKS = \
	$(proof-io_VERIFY) $(ramfs_VERIFY) $(keyring_VERIFY) \
	$(entropy_VERIFY) $(crypto_VERIFY) $(vfs_VERIFY) \
	$(driver-virtio-rng_VERIFY) $(driver-virtio-blk_VERIFY) \
	$(driver-virtio-gpu_VERIFY) $(driver-virtio-net_VERIFY) \
	$(driver-ps2-input_VERIFY) $(driver-xhci_VERIFY) \
	$(net-l2_VERIFY) $(net-ip_VERIFY) $(net-udp_VERIFY) \
	$(net-dhcp_VERIFY) $(net-tcp_VERIFY) $(net-dns_VERIFY) \
	$(net-sockets_VERIFY) $(net-nym_VERIFY) \
	$(policy_VERIFY) $(wallpaper_catalog_VERIFY) \
	$(input-router_VERIFY) $(compositor_VERIFY) $(wm_VERIFY) \
	$(desktop-shell_VERIFY) $(image-codec_VERIFY) $(clipboard_VERIFY) \
	$(login_VERIFY) $(wallpaper_VERIFY) $(toolkit_VERIFY) \
	$(about_VERIFY) $(calculator_VERIFY) $(terminal_VERIFY) \
	$(file-manager_VERIFY) $(text-editor_VERIFY) $(settings_VERIFY) \
	$(process-manager_VERIFY) $(attest_VERIFY) $(power_VERIFY)

.PHONY: nonos-mk-verify-desktop-gui-capsules
nonos-mk-verify-desktop-gui-capsules: $(NONOS_DESKTOP_GUI_CAPSULE_CHECKS)

WALLPAPER_BIN := $(wallpaper_BIN)

MARKETPLACE_ABI_LIB := $(USERLAND_DIR)/marketplace_abi/target/x86_64-nonos-user/release/libnonos_marketplace_abi.rlib

$(MARKETPLACE_ABI_LIB):
	@echo "Building marketplace ABI rlib..."
	@cd $(USERLAND_DIR)/marketplace_abi && \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --target ../x86_64-nonos-user.json \
		-Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem

nonos-mk-marketplace-abi: $(MARKETPLACE_ABI_LIB)

# capsule_market depends on marketplace_abi as a path crate; the
# macro-emitted build rule for $(market_BIN) covers the cargo
# build, this static prerequisite line keeps the lib honestly
# rebuilt before market when the ABI changes.
$(market_BIN): $(MARKETPLACE_ABI_LIB)

# Marketplace capsule built with the publicly-known smoketest
# trust pubkey baked in. The kernel-side market smoke loads
# fixtures signed by the matching seed; without this feature
# bootstrap-trust would refuse every fixture. Production builds
# must NOT enable this feature.
nonos-mk-market-smoke: $(USERLAND_LIBC) $(MARKETPLACE_ABI_LIB)
	@echo "Building marketplace capsule (smoketest-trust)..."
	@cd $(USERLAND_DIR)/capsule_market && \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --target ../x86_64-nonos-user.json \
		--features smoketest-trust \
		-Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem

# Host-side marketplace-index CLI. This is the bridge an operator
# runs offline: read JSON, encode canonical binary, sign with the
# Ed25519 operator key, verify. The tool is host-native (no kernel
# target), pinned to the same toolchain as the rest of the tree so
# CI gets a deterministic build.
MARKETPLACE_INDEX_TOOL := nonos-mk/target/$(HOST_TARGET)/release/marketplace-index

$(MARKETPLACE_INDEX_TOOL):
	@echo "Building host marketplace-index CLI..."
	@cd nonos-mk && RUSTFLAGS="" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --bin marketplace-index --target $(HOST_TARGET)

nonos-mk-marketplace-index-tool: $(MARKETPLACE_INDEX_TOOL)

# Generate the four signed fixtures the kernel-side market smoke
# embeds. Depends on the host marketplace-index CLI. The trusted
# seed is `0x42`-repeated-32 (publicly known); the matching
# pubkey is gated into capsule_market via the `smoketest-trust`
# feature. The untrusted blob is signed by `0xAA`-repeated-32 so
# the runtime can prove the trust list refuses unknown operators.
TEST_MARKET_DIR := target/test-market
TEST_MARKET_FIXTURES := \
	$(TEST_MARKET_DIR)/empty.bin \
	$(TEST_MARKET_DIR)/preview.bin \
	$(TEST_MARKET_DIR)/mutated.bin \
	$(TEST_MARKET_DIR)/untrusted.bin

$(TEST_MARKET_FIXTURES): $(MARKETPLACE_INDEX_TOOL) nonos-ci/build-market-fixtures.sh
	@bash nonos-ci/build-market-fixtures.sh $(TEST_MARKET_DIR) $(MARKETPLACE_INDEX_TOOL)

nonos-mk-market-fixtures: $(TEST_MARKET_FIXTURES)

nonos-mk-userland-clean:
	@echo "Removing userland build state..."
	@rm -rf $(USERLAND_DIR)/libc/target \
		$(USERLAND_DIR)/capsule_proof_io/target \
		$(USERLAND_DIR)/capsule_ramfs/target \
		$(USERLAND_DIR)/capsule_keyring/target \
		$(USERLAND_DIR)/capsule_entropy/target \
		$(USERLAND_DIR)/capsule_crypto/target \
		$(USERLAND_DIR)/capsule_vfs/target \
		$(USERLAND_DIR)/capsule_driver_virtio_rng/target \
		$(USERLAND_DIR)/capsule_driver_virtio_blk/target \
		$(USERLAND_DIR)/capsule_driver_virtio_net/target \
		$(USERLAND_DIR)/capsule_driver_ahci/target \
		$(USERLAND_DIR)/capsule_driver_hda/target \
		$(USERLAND_DIR)/capsule_driver_nvme/target \
		$(USERLAND_DIR)/capsule_wallpaper/target \
		$(USERLAND_DIR)/marketplace_abi/target \
		$(USERLAND_DIR)/capsule_market/target

# Kernel — every target spells the profile out explicitly.

KERNEL_BUILD_FLAGS := --release --target x86_64-nonos.json \
		-Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem

KERNEL_SIGNING_KEY = $(if $(filter /%,$(SIGNING_KEY)),$(SIGNING_KEY),$(shell pwd)/$(SIGNING_KEY))

# Kernel ELF artefact rule, no-features default (resolves to
# microkernel-core via Cargo.toml). Phony deps stay off this rule so a
# chain walk does not invalidate kernels built with a feature variant.
$(TARGET_DIR)/x86_64-nonos/release/nonos-kernel: $(SIGNING_KEY)
	@echo "Building kernel (default = microkernel-core)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS)

nonos-mk-check: nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "cargo check (microkernel-core)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) check $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-core

nonos-mk-core: nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-core, no capsules)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-core

nonos-mk-capsules: $(proof-io_ARTIFACTS) $(ramfs_BIN) $(keyring_BIN) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-capsules: proof_io + ramfs + keyring)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-capsules

nonos-mk-driver-virtio-rng-test: $(proof-io_ARTIFACTS) $(driver-virtio-rng_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (driver-virtio-rng smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-virtio-rng-smoketest

nonos-mk-ramfs-test: $(proof-io_ARTIFACTS) $(ramfs_BIN) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (ramfs smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--features nonos-capsule-proof-io,nonos-capsule-ramfs,nonos-ramfs-smoketest

nonos-mk-keyring-test: $(proof-io_ARTIFACTS) $(ramfs_BIN) $(keyring_BIN) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-keyring-smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-keyring-smoketest

nonos-mk-entropy-test: $(proof-io_ARTIFACTS) $(entropy_BIN) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-entropy-smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-entropy-smoketest

nonos-mk-crypto-hash-test: $(proof-io_ARTIFACTS) $(crypto_BIN) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-crypto-hash-smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-crypto-hash-smoketest

nonos-mk-vfs-test: $(proof-io_ARTIFACTS) $(vfs_BIN) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-vfs-smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-vfs-smoketest

# Boot-time marketplace round trip. The kernel embeds the
# `smoketest-trust`-flavoured market binary plus the four
# fixture blobs and exercises the five accept/reject paths.
nonos-mk-market-test: $(proof-io_ARTIFACTS) nonos-mk-market-smoke nonos-mk-market-fixtures \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-market-smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-market-smoketest

# Empty 4 MiB raw disk for the virtio-blk smoke. The smoketest
# writes a deterministic pattern to LBA 64 and reads it back;
# the disk needs at least that capacity. Created with truncate
# so it stays a sparse file on disk.
TEST_VIRTIO_BLK_IMG := target/test-virtio-blk.img

$(TEST_VIRTIO_BLK_IMG):
	@mkdir -p $(dir $(TEST_VIRTIO_BLK_IMG))
	@truncate -s 4M $(TEST_VIRTIO_BLK_IMG)

nonos-mk-virtio-blk-test-image: $(TEST_VIRTIO_BLK_IMG)

# Boot-time virtio-blk round trip. The kernel embeds the
# userland virtio-blk capsule plus the smoketest harness and
# drives the device via the broker. The harness shell script
# attaches the scratch image at boot.
nonos-mk-driver-virtio-blk-test: $(proof-io_ARTIFACTS) $(driver-virtio-blk_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-virtio-blk-smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-virtio-blk-smoketest

# Boot-time virtio-net round trip. Builds the kernel with the
# microkernel-driver-virtio-net-smoketest profile; the harness
# at tests/boot/virtio_net_round_trip.sh attaches a virtio-net-pci
# device backed by a `-netdev user` interface.
nonos-mk-driver-virtio-net-test: $(proof-io_ARTIFACTS) $(driver-virtio-net_BIN) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-virtio-net-smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-virtio-net-smoketest

# Boot-time PS/2 input round trip. Builds the kernel with the
# microkernel-driver-ps2-input-smoketest profile; the harness at
# tests/boot/ps2_input_round_trip.sh boots under QEMU and uses
# the QEMU monitor `sendkey` command to inject scancodes while
# the smoke is polling.
nonos-mk-driver-ps2-input-test: $(proof-io_ARTIFACTS) $(driver-ps2-input_BIN) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-ps2-input-smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-ps2-input-smoketest

# Boot-time xHCI controller-bring-up smoke (P0). Builds the kernel
# with the microkernel-driver-xhci-smoketest profile; the harness
# at tests/boot/xhci_round_trip.sh attaches `-device qemu-xhci`.
nonos-mk-driver-xhci-test: $(proof-io_ARTIFACTS) $(driver-xhci_BIN) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-xhci-smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-xhci-smoketest

# Input stack end-to-end proof (Deliverable 2). Dedicated build + ESP
# packaging that bypasses nonos-mk-esp (which always rebuilds the
# desktop-gui kernel); these targets sign+attest the just-built
# feature kernel into a lane-specific ESP. The trimmed fleet embeds the
# whole desktop input chain plus capsule_input_proof.
NONOS_INPUT_E2E_ARTIFACTS := $(proof-io_ARTIFACTS) $(ramfs_ARTIFACTS) \
	$(keyring_ARTIFACTS) $(entropy_ARTIFACTS) $(crypto_ARTIFACTS) \
	$(vfs_ARTIFACTS) $(driver-virtio-rng_ARTIFACTS) \
	$(driver-virtio-blk_ARTIFACTS) $(driver-virtio-gpu_ARTIFACTS) \
	$(driver-virtio-net_ARTIFACTS) $(net-l2_ARTIFACTS) $(net-ip_ARTIFACTS) \
	$(net-udp_ARTIFACTS) $(net-dhcp_ARTIFACTS) $(net-tcp_ARTIFACTS) \
	$(net-dns_ARTIFACTS) $(net-sockets_ARTIFACTS) $(net-nym_ARTIFACTS) \
	$(input-router_ARTIFACTS) $(compositor_ARTIFACTS) $(wm_ARTIFACTS) \
	$(image-codec_ARTIFACTS) $(clipboard_ARTIFACTS) $(attest_ARTIFACTS) \
	$(toolkit_ARTIFACTS) $(power_ARTIFACTS) $(input-proof_ARTIFACTS)

NONOS_BOOT_EFI := $(BOOTLOADER_DIR)/target/x86_64-unknown-uefi/release/nonos_boot.efi

define NONOS_INPUT_E2E_ESP
	@echo "Signing + attesting $(1) kernel..."
	@PY=python3; [ "$$(uname -s)" = Darwin ] && PY=/usr/bin/python3; \
		$$PY nonos-utils/sign_kernel.py \
		$(TARGET_DIR)/x86_64-nonos/release/nonos-kernel $(SIGNING_KEY) \
		$(TARGET_DIR)/kernel_signed_$(1).bin
	@$(EMBED_TOOL) --input $(TARGET_DIR)/kernel_signed_$(1).bin \
		--output $(TARGET_DIR)/kernel_attested_$(1).bin \
		--proving-key $(ZK_PROVING_KEY) --seed "$(ZK_KEY_SEED)" --verbose
	@mkdir -p $(TARGET_DIR)/esp-$(1)/EFI/Boot $(TARGET_DIR)/esp-$(1)/EFI/nonos
	@cp $(NONOS_BOOT_EFI) $(TARGET_DIR)/esp-$(1)/EFI/Boot/BOOTX64.EFI
	@cp $(TARGET_DIR)/kernel_attested_$(1).bin $(TARGET_DIR)/esp-$(1)/EFI/nonos/kernel.bin
	@printf "timeout=0\ndefault=nonos\n" > $(TARGET_DIR)/esp-$(1)/EFI/nonos/boot.cfg
	@echo 'fs0:\EFI\Boot\BOOTX64.EFI' > $(TARGET_DIR)/esp-$(1)/startup.nsh
	@echo "$(1) ESP ready at $(TARGET_DIR)/esp-$(1)"
endef

nonos-mk-input-e2e-ps2-test: $(NONOS_INPUT_E2E_ARTIFACTS) \
		$(driver-ps2-input_ARTIFACTS) nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-input-e2e-ps2)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-input-e2e-ps2

nonos-mk-input-e2e-ps2-esp: nonos-mk-input-e2e-ps2-test $(NONOS_BOOT_EFI) $(EMBED_TOOL)
	$(call NONOS_INPUT_E2E_ESP,input-e2e-ps2)

NONOS_INPUT_PROBE_ARTIFACTS := $(proof-io_ARTIFACTS) \
	$(driver-ps2-input_ARTIFACTS) $(driver-virtio-gpu_ARTIFACTS) \
	$(input-router_ARTIFACTS) $(compositor_ARTIFACTS) \
	$(input-probe_ARTIFACTS)

nonos-mk-input-probe-test: $(NONOS_INPUT_PROBE_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-input-probe)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-input-probe

nonos-mk-input-probe-esp: nonos-mk-input-probe-test $(NONOS_BOOT_EFI) $(EMBED_TOOL)
	$(call NONOS_INPUT_E2E_ESP,input-probe)

nonos-mk-input-probe-inject-test: $(NONOS_INPUT_PROBE_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-input-probe,input-probe-inject)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-input-probe,input-probe-inject

nonos-mk-input-probe-run: nonos-mk-input-probe-esp $(QEMU_BLK_IMG) $(QEMU_OVMF_VARS_RW)
	@echo "Booting NONOS input-probe in QEMU (GPU window + serial)..."
	@echo "  Quit: Ctrl+A then X"
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(TARGET_DIR)/esp-input-probe" \
		-drive if=pflash,format=raw,unit=0,readonly=on,file="$(OVMF)" \
		-drive if=pflash,format=raw,unit=1,file="$(QEMU_OVMF_VARS_RW)" \
		$(QEMU_BLK) $(QEMU_GPU) $(QEMU_NET) $(QEMU_USB) $(QEMU_RNG) \
		-serial mon:stdio -vga none -no-reboot

nonos-mk-input-probe-run-serial: nonos-mk-input-probe-esp $(QEMU_OVMF_VARS_RW)
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(TARGET_DIR)/esp-input-probe" \
		-drive if=pflash,format=raw,readonly=on,file="$(OVMF)" \
		$(QEMU_BLK) $(QEMU_GPU) $(QEMU_NET) $(QEMU_USB) $(QEMU_RNG) \
		-serial mon:stdio -display none -no-reboot

nonos-mk-input-e2e-xhci-test: $(NONOS_INPUT_E2E_ARTIFACTS) \
		$(driver-xhci_ARTIFACTS) $(driver-usb-hid_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-input-e2e-xhci)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-input-e2e-xhci

nonos-mk-input-e2e-xhci-esp: nonos-mk-input-e2e-xhci-test $(NONOS_BOOT_EFI) $(EMBED_TOOL)
	$(call NONOS_INPUT_E2E_ESP,input-e2e-xhci)

# Boot-time wallpaper round trip. Kernel boots wallpaper as the
# init one-shot; the binary drives display_dimensions /
# surface_create / surface_map / surface_present_full /
# surface_destroy and emits per-stage markers on serial.
nonos-mk-wallpaper-test: $(WALLPAPER_BIN) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-wallpaper-smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-wallpaper-smoketest

# Per-capsule production kernel builds. Each `-prod` target builds
# the kernel under the `nonos-production` posture with proof_io and
# the named capsule baked in. Dependencies require the capsule's
# signed bundle (cert + manifest + ELF) so the kernel's
# `include_bytes!` resolves a verified payload. The macro-emitted
# `nonos-mk-<slug>` target stays as the userland-ELF builder; the
# `-prod` suffix is the only kernel build path under production
# posture.

nonos-mk-proof-io-prod: $(proof-io_ARTIFACTS) nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-proof-io)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-proof-io

nonos-mk-ramfs-prod: $(proof-io_ARTIFACTS) $(ramfs_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-ramfs)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-ramfs

nonos-mk-keyring-prod: $(proof-io_ARTIFACTS) $(keyring_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-keyring)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-keyring

nonos-mk-entropy-prod: $(proof-io_ARTIFACTS) $(entropy_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-entropy)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-entropy

nonos-mk-crypto-prod: $(proof-io_ARTIFACTS) $(crypto_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-crypto)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-crypto

nonos-mk-vfs-prod: $(proof-io_ARTIFACTS) $(vfs_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-vfs)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-vfs

nonos-mk-market-prod: $(proof-io_ARTIFACTS) $(market_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-market)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-market

nonos-mk-driver-virtio-rng-prod: $(proof-io_ARTIFACTS) $(driver-virtio-rng_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-virtio-rng)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-virtio-rng

nonos-mk-driver-virtio-blk-prod: $(proof-io_ARTIFACTS) $(driver-virtio-blk_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-virtio-blk)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-virtio-blk

nonos-mk-driver-virtio-gpu-prod: $(proof-io_ARTIFACTS) $(driver-virtio-gpu_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-virtio-gpu)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-virtio-gpu

nonos-mk-driver-virtio-net-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-virtio-net)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-virtio-net

nonos-mk-driver-iwlwifi-prod: $(proof-io_ARTIFACTS) $(driver-iwlwifi_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-iwlwifi)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-iwlwifi

nonos-mk-driver-i2c-pci-prod: $(proof-io_ARTIFACTS) $(driver-i2c-pci_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-i2c-pci)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-i2c-pci

nonos-mk-driver-i2c-hid-prod: $(proof-io_ARTIFACTS) $(driver-i2c-pci_ARTIFACTS) $(driver-i2c-hid_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-i2c-hid)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-i2c-hid

nonos-mk-driver-ps2-input-prod: $(proof-io_ARTIFACTS) $(driver-ps2-input_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-ps2-input)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-ps2-input

nonos-mk-driver-xhci-prod: $(proof-io_ARTIFACTS) $(driver-xhci_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-xhci)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-xhci

nonos-mk-driver-usb-hid-prod: $(proof-io_ARTIFACTS) $(driver-xhci_ARTIFACTS) \
		$(driver-usb-hid_ARTIFACTS) nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-usb-hid)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-usb-hid

nonos-mk-driver-usb-msc-prod: $(proof-io_ARTIFACTS) $(driver-xhci_ARTIFACTS) \
		$(driver-usb-msc_ARTIFACTS) nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-usb-msc)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-usb-msc

nonos-mk-driver-e1000-prod: $(proof-io_ARTIFACTS) $(driver-e1000_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-e1000)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-e1000

nonos-mk-driver-rtl8139-prod: $(proof-io_ARTIFACTS) $(driver-rtl8139_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-rtl8139)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-rtl8139

nonos-mk-driver-rtl8169-prod: $(proof-io_ARTIFACTS) $(driver-rtl8169_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-rtl8169)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-rtl8169

nonos-mk-driver-ahci-prod: $(proof-io_ARTIFACTS) $(driver-ahci_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-ahci)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-ahci

nonos-mk-driver-hda-prod: $(proof-io_ARTIFACTS) $(driver-hda_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-hda)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-hda

nonos-mk-driver-nvme-prod: $(proof-io_ARTIFACTS) $(driver-nvme_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-nvme)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-driver-nvme

# Net-service capsule kernel profiles. Each stacks on the lower
# layer so a microkernel-net-dhcp build pulls in net.l2 + net.ip +
# net.udp + net.dhcp and the underlying virtio-net driver.

nonos-mk-net-l2-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-l2_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-net-l2)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-net-l2

nonos-mk-net-ip-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-l2_ARTIFACTS) $(net-ip_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-net-ip)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-net-ip

nonos-mk-net-udp-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-l2_ARTIFACTS) $(net-ip_ARTIFACTS) $(net-udp_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-net-udp)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-net-udp

nonos-mk-net-dhcp-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-l2_ARTIFACTS) $(net-ip_ARTIFACTS) $(net-udp_ARTIFACTS) \
		$(net-dhcp_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-net-dhcp)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-net-dhcp

nonos-mk-net-nym-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-l2_ARTIFACTS) $(net-ip_ARTIFACTS) $(net-udp_ARTIFACTS) \
		$(net-dhcp_ARTIFACTS) $(net-tcp_ARTIFACTS) $(net-nym_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-net-nym)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-net-nym

nonos-mk-net-sockets-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-l2_ARTIFACTS) $(net-ip_ARTIFACTS) $(net-udp_ARTIFACTS) \
		$(net-dhcp_ARTIFACTS) $(net-tcp_ARTIFACTS) $(net-dns_ARTIFACTS) \
		$(net-sockets_ARTIFACTS) $(net-nym_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-net-sockets)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-net-sockets

nonos-mk-desktop-gui-prod: $(proof-io_ARTIFACTS) $(ramfs_ARTIFACTS) \
		$(keyring_ARTIFACTS) $(entropy_ARTIFACTS) $(crypto_ARTIFACTS) \
		$(vfs_ARTIFACTS) $(driver-virtio-rng_ARTIFACTS) \
		$(driver-virtio-blk_ARTIFACTS) $(driver-virtio-gpu_ARTIFACTS) \
		$(driver-virtio-net_ARTIFACTS) $(driver-ps2-input_ARTIFACTS) \
		$(driver-xhci_ARTIFACTS) $(net-l2_ARTIFACTS) \
		$(net-ip_ARTIFACTS) $(net-udp_ARTIFACTS) $(net-dhcp_ARTIFACTS) \
		$(net-tcp_ARTIFACTS) $(net-dns_ARTIFACTS) $(net-sockets_ARTIFACTS) \
		$(net-nym_ARTIFACTS) \
		$(policy_ARTIFACTS) $(wallpaper_catalog_ARTIFACTS) \
		$(input-router_ARTIFACTS) $(compositor_ARTIFACTS) \
		$(wm_ARTIFACTS) $(desktop-shell_ARTIFACTS) \
		$(image-codec_ARTIFACTS) $(clipboard_ARTIFACTS) \
		$(login_ARTIFACTS) $(wallpaper_ARTIFACTS) \
		$(toolkit_ARTIFACTS) $(about_ARTIFACTS) \
		$(calculator_ARTIFACTS) $(terminal_ARTIFACTS) \
		$(file-manager_ARTIFACTS) $(text-editor_ARTIFACTS) \
		$(settings_ARTIFACTS) $(process-manager_ARTIFACTS) \
		$(attest_ARTIFACTS) $(power_ARTIFACTS) \
		nonos-mk-verify-desktop-gui-capsules \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-desktop-gui)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-desktop-gui

nonos-mk-toolkit-prod: nonos-mk-desktop-gui-prod
nonos-mk-about-prod: nonos-mk-desktop-gui-prod
nonos-mk-calculator-prod: nonos-mk-desktop-gui-prod
nonos-mk-terminal-prod: nonos-mk-desktop-gui-prod
nonos-mk-file-manager-prod: nonos-mk-desktop-gui-prod

nonos-mk-terminal-only-prod: $(proof-io_ARTIFACTS) $(ramfs_ARTIFACTS) $(keyring_ARTIFACTS) \
		$(entropy_ARTIFACTS) $(crypto_ARTIFACTS) $(vfs_ARTIFACTS) \
		$(driver-virtio-rng_ARTIFACTS) $(driver-virtio-gpu_ARTIFACTS) \
		$(driver-ps2-input_ARTIFACTS) $(input-router_ARTIFACTS) \
		$(compositor_ARTIFACTS) $(wm_ARTIFACTS) $(toolkit_ARTIFACTS) \
		$(terminal_ARTIFACTS) nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-terminal-only)..."
	@RUSTUP_TOOLCHAIN=$(TOOLCHAIN) NONOS_SIGNING_KEY=$(NONOS_SIGNING_KEY) \
		$(CARGO) build --release --target $(TARGET_TRIPLE) -Zbuild-std=core,alloc \
		-Zbuild-std-features=compiler-builtins-mem \
		--no-default-features --features microkernel-terminal-only
nonos-mk-text-editor-prod: nonos-mk-desktop-gui-prod
nonos-mk-settings-prod: nonos-mk-desktop-gui-prod
nonos-mk-process-manager-prod: nonos-mk-desktop-gui-prod

# Sign + attest + ESP packaging

$(TARGET_DIR)/kernel_signed.bin: $(TARGET_DIR)/x86_64-nonos/release/nonos-kernel $(SIGNING_KEY)
	@echo "Signing kernel (Ed25519)..."
	@mkdir -p $(TARGET_DIR)
ifeq ($(UNAME_S),Darwin)
	@/usr/bin/python3 nonos-utils/sign_kernel.py $< $(SIGNING_KEY) $@
else
	@python3 nonos-utils/sign_kernel.py $< $(SIGNING_KEY) $@
endif

nonos-mk-sign: $(TARGET_DIR)/kernel_signed.bin

$(TARGET_DIR)/kernel_attested.bin: $(TARGET_DIR)/kernel_signed.bin $(EMBED_TOOL) $(ZK_PROVING_KEY)
	@echo "Embedding ZK attestation proof..."
	@$(EMBED_TOOL) --input $< --output $@ --proving-key $(ZK_PROVING_KEY) --seed "$(ZK_KEY_SEED)" --verbose

nonos-mk-attest: $(TARGET_DIR)/kernel_attested.bin

nonos-mk-esp: \
		$(BOOTLOADER_DIR)/target/x86_64-unknown-uefi/release/nonos_boot.efi \
		$(TARGET_DIR)/kernel_attested.bin
	@echo "Packaging EFI System Partition..."
	@mkdir -p $(ESP_DIR)/EFI/Boot $(ESP_DIR)/EFI/nonos
	@cp $(BOOTLOADER_DIR)/target/x86_64-unknown-uefi/release/nonos_boot.efi $(ESP_DIR)/EFI/Boot/BOOTX64.EFI
	@cp $(TARGET_DIR)/kernel_attested.bin $(ESP_DIR)/EFI/nonos/kernel.bin
	@printf "timeout=0\ndefault=nonos\n" > $(ESP_DIR)/EFI/nonos/boot.cfg
	@echo 'fs0:\EFI\Boot\BOOTX64.EFI' > $(ESP_DIR)/startup.nsh
	@echo "ESP ready at $(ESP_DIR)"

# QEMU

$(QEMU_BLK_IMG):
	@mkdir -p $(dir $@)
	@truncate -s 64M $@

$(QEMU_OVMF_VARS_RW): $(OVMF_VARS)
	@mkdir -p $(dir $@)
	@[ -n "$(OVMF_VARS)" ] || { echo "::error::OVMF_VARS not found"; exit 1; }
	@cp "$(OVMF_VARS)" "$@"

nonos-mk-run: nonos-mk-desktop-gui-prod nonos-mk-esp $(QEMU_BLK_IMG) $(QEMU_OVMF_VARS_RW)
	@echo "Booting NONOS in QEMU..."
	@echo "  SSH:  ssh -p $(QEMU_HOST_SSH_PORT) localhost"
	@echo "  HTTP: http://localhost:$(QEMU_HOST_HTTP_PORT)"
	@echo "  Quit: Ctrl+A then X"
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
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

nonos-mk-run-serial: nonos-mk-desktop-gui-prod nonos-mk-esp
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
		-drive if=pflash,format=raw,readonly=on,file="$(OVMF)" \
		$(QEMU_BLK) $(QEMU_GPU) $(QEMU_NET) $(QEMU_USB) $(QEMU_RNG) \
		-serial mon:stdio -display none -no-reboot

nonos-mk-debug: nonos-mk-desktop-gui-prod nonos-mk-esp
	@echo "QEMU listening for GDB on :1234   (gdb -ex 'target remote :1234')"
	@$(QEMU) -m $(QEMU_MEM) -cpu $(QEMU_CPU) -smp $(QEMU_SMP) -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
		-drive if=pflash,format=raw,readonly=on,file="$(OVMF)" \
		$(QEMU_NET) $(QEMU_RNG) \
		-serial mon:stdio -vga std -s -S -no-reboot

# Boot-test harnesses

nonos-mk-boot-ramfs:
	@./tests/boot/ramfs_round_trip.sh

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

nonos-mk-boot-input-e2e-ps2:
	@./tests/boot/input_e2e_ps2.sh

nonos-mk-boot-input-e2e-xhci:
	@./tests/boot/input_e2e_xhci.sh

nonos-mk-boot-desktop-gui:
	@./tests/boot/desktop_gui_boot.sh

nonos-mk-plan-a-runtime: nonos-mk-desktop-gui-prod nonos-mk-esp $(QEMU_BLK_IMG) $(QEMU_OVMF_VARS_RW)
	@QEMU="$(QEMU)" OVMF="$(OVMF)" OVMF_VARS="$(OVMF_VARS)" \
		QEMU_OVMF_VARS_RW="$(QEMU_OVMF_VARS_RW)" QEMU_BLK_IMG="$(QEMU_BLK_IMG)" \
		ESP_DIR="$(ESP_DIR)" ./nonos-ci/plan-a-runtime.sh

# Verify

nonos-mk-static:
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

# Host-mode crate tests (currently flaky on TSC; tracked in
# docs/production-roadmap/master-execution-checklist.md F1).
nonos-mk-host-test:
	@RUSTUP_TOOLCHAIN=$(TOOLCHAIN) $(CARGO) test --lib --features std --target $(HOST_TARGET)
	@cd $(BOOTLOADER_DIR) && RUSTUP_TOOLCHAIN=$(TOOLCHAIN) $(CARGO) test

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
	@rm -f $(SIGNING_KEY)
	@rm -rf $(ZK_KEYS_DIR)

nonos-mk-fmt:
	@RUSTUP_TOOLCHAIN=$(TOOLCHAIN) $(CARGO) fmt
	@cd $(BOOTLOADER_DIR) && RUSTUP_TOOLCHAIN=$(TOOLCHAIN) $(CARGO) fmt

# Help (default target)

help:
	@echo "NONOS microkernel build"
	@echo
	@echo "Build:"
	@echo "  make nonos-mk                 microkernel-capsules runtime baseline"
	@echo "  make nonos-mk-core            kernel only (microkernel-core, no capsules)"
	@echo "  make nonos-mk-check           cargo check (microkernel-core)"
	@echo "  make nonos-mk-capsules        microkernel-capsules build"
	@echo "  make nonos-mk-ramfs-test      ramfs smoketest profile"
	@echo "  make nonos-mk-keyring-test    keyring smoketest profile"
	@echo "  make nonos-mk-entropy-test    entropy smoketest profile"
	@echo "  make nonos-mk-crypto-hash-test crypto hash smoketest profile"
	@echo "  make nonos-mk-vfs-test        vfs smoketest profile"
	@echo
	@echo "Userland capsules:"
	@echo "  make nonos-mk-libc nonos-mk-proof-io nonos-mk-ramfs nonos-mk-keyring"
	@echo "  make nonos-mk-userland-clean"
	@echo
	@echo "Sign / attest / package:"
	@echo "  make nonos-mk-sign            Ed25519 manifest signature"
	@echo "  make nonos-mk-attest          Groth16 attestation proof"
	@echo "  make nonos-mk-bootloader      UEFI bootloader"
	@echo "  make nonos-mk-esp             EFI System Partition for QEMU"
	@echo
	@echo "Run:"
	@echo "  make nonos-mk-run             QEMU + OVMF (SSH:2222, HTTP:8080)"
	@echo "  make nonos-mk-run-serial      headless serial-only"
	@echo "  make nonos-mk-debug           QEMU + GDB on :1234"
	@echo
	@echo "Verify:"
	@echo "  make nonos-mk-static          CI static gates"
	@echo "  make nonos-mk-scan            symbol scan over kernel image"
	@echo "  make nonos-mk-verify-fast     static gates only (no kernel build)"
	@echo "  make nonos-mk-verify          static gates + capsules build + scan"
	@echo "  make nonos-mk-boot-ramfs      ramfs capsule round trip under QEMU"
	@echo "  make nonos-mk-boot-keyring    keyring capsule round trip under QEMU"
	@echo "  make nonos-mk-boot-entropy    entropy capsule round trip under QEMU"
	@echo "  make nonos-mk-boot-crypto-hash crypto hash round trip under QEMU"
	@echo "  make nonos-mk-boot-vfs        vfs capsule round trip under QEMU"
	@echo "  make nonos-mk-boot-input-e2e-ps2  input stack e2e proof (PS/2) under QEMU"
	@echo "  make nonos-mk-boot-input-e2e-xhci input stack e2e proof (xHCI HID) under QEMU"
	@echo "  make nonos-mk-test            verify + both boot harnesses"
	@echo "  make nonos-mk-host-test       host-mode cargo tests (flaky; see roadmap)"
	@echo
	@echo "Release:"
	@echo "  make nonos-mk-release         (paused; see message)"
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
kernel-keyring-smoketest:              nonos-mk-keyring-test
kernel-microkernel-keyring-smoketest:  nonos-mk-keyring-test
kernel-ramfs-smoketest:                nonos-mk-ramfs-test
ramfs-boot-test:                       nonos-mk-boot-ramfs
boot-test:                             nonos-mk-boot-ramfs
check-static:                          nonos-mk-static
clean-kernel-only:                     nonos-mk-clean
microkernel-symbol-scan:               nonos-mk-scan
