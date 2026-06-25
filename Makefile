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
#   make nonos-mk-verify       static + trust gates + desktop build + symbol scan
#
# A few old names (`kernel-capsules`, `kernel-with-keyring`,
# etc.) forward to `nonos-mk-*` for transitional compatibility. See the
# bottom of the file.

.PHONY: help

# Public nonos-mk-* targets
.PHONY: nonos-mk
.PHONY: nonos-mk-check nonos-mk-check-ramfs-keys nonos-mk-core nonos-mk-capsules nonos-mk-terminal-test
.PHONY: nonos-mk-proof-io-prod nonos-mk-ramfs-prod nonos-mk-keyring-prod nonos-mk-entropy-prod nonos-mk-crypto-prod nonos-mk-vfs-prod nonos-mk-market-prod nonos-mk-driver-virtio-rng-prod nonos-mk-driver-virtio-blk-prod nonos-mk-driver-virtio-gpu-prod nonos-mk-driver-virtio-net-prod nonos-mk-driver-iwlwifi-prod nonos-mk-driver-i2c-pci-prod nonos-mk-driver-i2c-hid-prod nonos-mk-driver-ps2-input-prod nonos-mk-driver-xhci-prod nonos-mk-driver-usb-hid-prod nonos-mk-driver-usb-msc-prod nonos-mk-driver-e1000-prod nonos-mk-driver-rtl8139-prod nonos-mk-driver-rtl8169-prod nonos-mk-driver-ahci-prod nonos-mk-driver-hda-prod nonos-mk-driver-nvme-prod nonos-mk-net-l2-prod nonos-mk-net-ip-prod nonos-mk-net-udp-prod nonos-mk-net-dhcp-prod nonos-mk-net-nym-prod nonos-mk-net-sockets-prod nonos-mk-desktop-gui-prod
.PHONY: nonos-mk-libc nonos-mk-proof-io nonos-mk-proof-io-sign nonos-mk-check-trust-keys nonos-mk-check-trust-manifest nonos-mk-trust-policy nonos-mk-host-trust-verify nonos-mk-verify-trust nonos-mk-ramfs nonos-mk-ramfs-sign nonos-mk-keyring nonos-mk-entropy nonos-mk-crypto nonos-mk-vfs nonos-mk-virtio-rng nonos-mk-virtio-rng-sign nonos-mk-check-virtio-rng-keys nonos-mk-virtio-blk nonos-mk-virtio-blk-sign nonos-mk-check-virtio-blk-keys nonos-mk-driver-virtio-gpu nonos-mk-driver-virtio-gpu-sign nonos-mk-check-driver-virtio-gpu-keys nonos-mk-virtio-net nonos-mk-virtio-net-sign nonos-mk-check-virtio-net-keys nonos-mk-driver-iwlwifi nonos-mk-driver-iwlwifi-sign nonos-mk-check-driver-iwlwifi-keys nonos-mk-driver-i2c-pci nonos-mk-driver-i2c-pci-sign nonos-mk-check-driver-i2c-pci-keys nonos-mk-driver-i2c-hid nonos-mk-driver-i2c-hid-sign nonos-mk-check-driver-i2c-hid-keys nonos-mk-ps2-input nonos-mk-ps2-input-sign nonos-mk-check-ps2-input-keys nonos-mk-xhci nonos-mk-xhci-sign nonos-mk-check-xhci-keys nonos-mk-driver-usb-msc nonos-mk-driver-usb-msc-sign nonos-mk-check-driver-usb-msc-keys nonos-mk-driver-e1000 nonos-mk-driver-e1000-sign nonos-mk-check-driver-e1000-keys nonos-mk-driver-rtl8139 nonos-mk-driver-rtl8139-sign nonos-mk-check-driver-rtl8139-keys nonos-mk-driver-rtl8169 nonos-mk-driver-rtl8169-sign nonos-mk-check-driver-rtl8169-keys nonos-mk-driver-ahci nonos-mk-driver-ahci-sign nonos-mk-check-driver-ahci-keys nonos-mk-driver-hda nonos-mk-driver-hda-sign nonos-mk-check-driver-hda-keys nonos-mk-driver-nvme nonos-mk-driver-nvme-sign nonos-mk-check-driver-nvme-keys nonos-mk-wallpaper nonos-mk-marketplace-abi nonos-mk-market nonos-mk-marketplace-index-tool
.PHONY: nonos-mk-userland-clean
.PHONY: nonos-mk-bootloader nonos-mk-sign nonos-mk-attest nonos-mk-esp
.PHONY: nonos-mk-run nonos-mk-run-nat nonos-mk-run-net nonos-mk-run-serial nonos-mk-run-serial-nat nonos-mk-run-serial-net nonos-mk-run-serial-log nonos-mk-debug nonos-mk-plan-a-runtime
.PHONY: nonos-mk-static nonos-mk-scan
.PHONY: nonos-mk-verify nonos-mk-verify-fast
.PHONY: nonos-mk-release-audit nonos-mk-claims-check nonos-mk-qemu-net-audit
.PHONY: ci-fast ci-security ci-release ci-soak nonos-mk-no-telemetry-capture nonos-mk-boot-evidence
.PHONY: nonos-mk-release
.PHONY: nonos-mk-clean nonos-mk-clean-all nonos-mk-distclean
.PHONY: nonos-mk-fmt
.PHONY: nonos-mk-toolchain nonos-mk-check-deps
.PHONY: nonos-mk-ensure-signing-key nonos-mk-ensure-zk-keys nonos-mk-zk-tools nonos-mk-all-capsules-attested nonos-mk-verify-capsule-attest nonos-mk-zk-report nonos-mk-zk-ceremony nonos-mk-zk-verify-live nonos-mk-live-production-proof nonos-mk-attestation nonos-mk-attestation-receipt

# Compatibility aliases (transitional, do not remove until callers move)
.PHONY: kernel-capsules kernel-with-keyring
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
NONOS_TRUST_DIR := nonos-data/trust

# ZK ceremony paths.
ZK_CIRCUIT_DIR   := $(BOOTLOADER_DIR)/tools/nonos-attestation-circuit
ZK_KEYS_DIR      := $(ZK_CIRCUIT_DIR)/generated_keys
ZK_PROVING_KEY   := $(ZK_KEYS_DIR)/attestation_proving_key.bin
ZK_VERIFYING_KEY := $(ZK_KEYS_DIR)/attestation_verifying_key.bin
ZK_KEY_SEED      := nonos-production-attestation-v1-2026
ZK_TOOL          := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/generate-keys
ZK_PROOF_TOOL    := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/generate-proof
ZK_VERIFY_TOOL   := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/verify-proof
ZK_PROOF_SCENE_TOOL := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/capsule-attest-proof
ZK_FLEET_TOOL    := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/capsule-attest-fleet
ZK_CEREMONY_TOOL := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/run_ceremony
ZK_POLICY_TOOL   := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/policy-root
ZK_CEREMONY_DIR  := $(ZK_KEYS_DIR)/ceremony
ZK_CEREMONY_ROUNDS := 5
ZK_POLICY_FILE   := $(TARGET_DIR)/capsule-attest/policy.tsv
ZK_POLICY_ROOT   = $(NONOS_TRUST_DIR)/policy/zk_capsule_policy_root.bin
EMBED_TOOL       := $(BOOTLOADER_DIR)/tools/embed-zk-proof/target/$(HOST_TARGET)/release/embed-zk-proof

# Header and status line, shown once per invocation. Fields are read from
# the working tree at parse time.
NONOS_BRANCH    := $(shell git rev-parse --abbrev-ref HEAD 2>/dev/null || echo detached)
NONOS_COMMIT    := $(shell git rev-parse --short HEAD 2>/dev/null || echo none)
NONOS_DIRTY     := $(shell test -n "$$(git status --porcelain 2>/dev/null)" && echo '*' || echo '')
NONOS_ATTESTED  := $(shell ls nonos-data/trust/capsules/*.zk_trailer.bin 2>/dev/null | wc -l | tr -d ' ')
NONOS_SIGNED    := $(shell ls nonos-data/trust/capsules/*.manifest.bin 2>/dev/null | wc -l | tr -d ' ')
NONOS_VK_FPR    := $(shell test -f $(ZK_VERIFYING_KEY) && shasum -a 256 $(ZK_VERIFYING_KEY) 2>/dev/null | cut -c1-16 || echo unkeyed)
NONOS_ENFORCE   := $(shell grep -q 'nonos-production = \["nonos-zk-enforce"\]' Cargo.toml && echo on || echo log-only)

define NONOS_BANNER

  ███╗   ██╗ ██████╗ ███╗   ██╗ ██████╗ ███████╗
  ████╗  ██║██╔═══██╗████╗  ██║██╔═══██╗██╔════╝
  ██╔██╗ ██║██║   ██║██╔██╗ ██║██║   ██║███████╗
  ██║╚██╗██║██║   ██║██║╚██╗██║██║   ██║╚════██║
  ██║ ╚████║╚██████╔╝██║ ╚████║╚██████╔╝███████║
  ╚═╝  ╚═══╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ ╚══════╝
endef
export NONOS_BANNER

$(info $(NONOS_BANNER))

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
QEMU_NET_MODE ?= hostfwd
QEMU_NET_CAPTURE ?=
QEMU_SERIAL_LOG ?= $(TARGET_DIR)/qemu-serial.log
QEMU_BLK_IMG := $(TARGET_DIR)/qemu-virtio-blk.img
QEMU_OVMF_VARS_RW := $(TARGET_DIR)/qemu-OVMF_VARS.fd
QEMU_BLK := -drive "file=$(QEMU_BLK_IMG),if=none,id=vd0,format=raw" -device virtio-blk-pci,drive=vd0
QEMU_XRES ?= 1920
QEMU_YRES ?= 1080
QEMU_GPU := -device virtio-vga,disable-modern=on,vectors=0,xres=$(QEMU_XRES),yres=$(QEMU_YRES)
# Keyboard/mouse via the q35 i8042 (PS/2). USB HID interrupt-IN transfers
# are not serviced under macOS hvf, so usb-kbd/usb-mouse never deliver input
# there; the xHCI controller stays for the USB stack/storage paths.
QEMU_USB := -device qemu-xhci,id=xhci
QEMU_RNG := -device virtio-rng-pci

ifeq ($(QEMU_NET_MODE),off)
    QEMU_NET :=
    QEMU_NET_DESC := disabled
else ifeq ($(QEMU_NET_MODE),nat)
    QEMU_NET := -device virtio-net-pci,netdev=net0 -netdev user,id=net0
    QEMU_NET_DESC := nat outbound virtio-net
    ifneq ($(QEMU_NET_CAPTURE),)
        QEMU_NET += -object filter-dump,id=net0dump,netdev=net0,file=$(QEMU_NET_CAPTURE)
        QEMU_NET_DESC := $(QEMU_NET_DESC) capture=$(QEMU_NET_CAPTURE)
    endif
else ifeq ($(QEMU_NET_MODE),hostfwd)
    QEMU_NET := -device virtio-net-pci,netdev=net0 -netdev user,id=net0,hostfwd=tcp::$(QEMU_HOST_SSH_PORT)-:22,hostfwd=tcp::$(QEMU_HOST_HTTP_PORT)-:80
    QEMU_NET_DESC := hostfwd ssh=$(QEMU_HOST_SSH_PORT) http=$(QEMU_HOST_HTTP_PORT)
    ifneq ($(QEMU_NET_CAPTURE),)
        QEMU_NET += -object filter-dump,id=net0dump,netdev=net0,file=$(QEMU_NET_CAPTURE)
        QEMU_NET_DESC := $(QEMU_NET_DESC) capture=$(QEMU_NET_CAPTURE)
    endif
else
    $(error Unsupported QEMU_NET_MODE=$(QEMU_NET_MODE). Use off, nat, or hostfwd)
endif

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

$(ZK_TOOL) $(ZK_PROOF_TOOL) $(ZK_VERIFY_TOOL) $(ZK_PROOF_SCENE_TOOL) $(ZK_FLEET_TOOL) $(ZK_CEREMONY_TOOL) $(ZK_POLICY_TOOL): nonos-mk-check-deps
	@echo "Building ZK attestation tools..."
	@cd $(ZK_CIRCUIT_DIR) && RUSTFLAGS="" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --bin generate-keys --bin generate-proof --bin verify-proof --bin capsule-attest-proof --bin capsule-attest-fleet --bin run_ceremony --bin policy-root --target $(HOST_TARGET)

# Trusted setup via the Groth16 phase-2 (BGM17) ceremony: no seed, real OS
# entropy per round, each contribution's secret destroyed. The composed delta
# is unknown as long as one round is honest.
# A fresh checkout installs the canonical ceremony from the trust keystore
# (nonos-data/zk, distributed with the submodule) and re-verifies its
# transcript, so every clone proves against the same verifying key. The
# local bootstrap ceremony only runs when no canonical bundle exists, which
# is the new-circuit maintainer path. Use nonos-mk-zk-ceremony to
# deliberately re-run a setup.
# Order-only prerequisites: the tool and signing key must exist, but their
# timestamps must NOT invalidate the keys. The ceremony runs once; rebuilding
# the tool must never silently regenerate keys and strand already-attested
# capsules.
NONOS_ZK_DIST := nonos-data/trust/zk
$(ZK_PROVING_KEY): | $(ZK_CEREMONY_TOOL) $(SIGNING_KEY)
	@if [ -f $(NONOS_ZK_DIST)/attestation_proving_key.bin ]; then \
		echo "Installing canonical ceremony keys from the trust keystore..."; \
		mkdir -p $(ZK_KEYS_DIR); \
		cp -R $(NONOS_ZK_DIST)/. $(ZK_KEYS_DIR)/; \
		$(ZK_CEREMONY_TOOL) verify --transcript $(ZK_KEYS_DIR)/ceremony_transcript.json; \
	else \
		echo "Running Groth16 phase-2 ceremony (seedless, $(ZK_CEREMONY_ROUNDS) rounds)..."; \
		mkdir -p $(ZK_KEYS_DIR) $(ZK_CEREMONY_DIR); \
		$(ZK_CEREMONY_TOOL) init --circuit attestation --output $(ZK_CEREMONY_DIR)/params_0.bin; \
		prev=$(ZK_CEREMONY_DIR)/params_0.bin; \
		for n in $$(seq 1 $(ZK_CEREMONY_ROUNDS)); do \
			$(ZK_CEREMONY_TOOL) contribute --input $$prev \
				--output $(ZK_CEREMONY_DIR)/params_$$n.bin \
				--name "NONOS:bootstrap:round$$n" --entropy system || exit 1; \
			prev=$(ZK_CEREMONY_DIR)/params_$$n.bin; \
		done; \
		$(ZK_CEREMONY_TOOL) assemble --meta $(ZK_CEREMONY_DIR)/params_0.bin.meta.json \
			--output $(ZK_CEREMONY_DIR)/transcript.json \
			$(ZK_CEREMONY_DIR)/params_*.bin.contribution.json; \
		$(ZK_CEREMONY_TOOL) finalize --input $(ZK_CEREMONY_DIR)/params_$(ZK_CEREMONY_ROUNDS).bin \
			--output $(ZK_KEYS_DIR) --transcript $(ZK_CEREMONY_DIR)/transcript.json; \
		$(ZK_CEREMONY_TOOL) verify --transcript $(ZK_KEYS_DIR)/ceremony_transcript.json; \
		cp $(ZK_VERIFYING_KEY) $(ZK_KEYS_DIR)/vk_attestation_program.bin; \
		cp $(ZK_VERIFYING_KEY) $(ZK_KEYS_DIR)/vk_boot_authority.bin; \
		cp $(ZK_VERIFYING_KEY) $(ZK_KEYS_DIR)/vk_update_authority.bin; \
		cp $(ZK_VERIFYING_KEY) $(ZK_KEYS_DIR)/vk_recovery_key.bin; \
	fi

$(ZK_VERIFYING_KEY): $(ZK_PROVING_KEY)

nonos-mk-ensure-zk-keys: $(ZK_PROVING_KEY) $(ZK_VERIFYING_KEY)
nonos-mk-zk-tools: $(ZK_TOOL) $(ZK_PROOF_TOOL) $(ZK_VERIFY_TOOL) $(ZK_PROOF_SCENE_TOOL) $(ZK_FLEET_TOOL) $(ZK_POLICY_TOOL)

# Force a fresh trusted-setup ceremony, discarding the existing keys. The
# capsule trailers are invalid after the VK changes, so this target deletes
# stale proofs, re-attests the full fleet, verifies it, then prints the report.
nonos-mk-zk-ceremony: $(ZK_CEREMONY_TOOL) $(SIGNING_KEY)
	@rm -f $(ZK_PROVING_KEY) $(ZK_VERIFYING_KEY) $(ZK_KEYS_DIR)/vk_*.bin
	@rm -rf $(ZK_CEREMONY_DIR) $(TARGET_DIR)/capsule-attest
	@rm -f $(ZK_POLICY_ROOT)
	@rm -f $(NONOS_TRUST_DIR)/capsules/*.zk_trailer.bin
	@$(MAKE) --no-print-directory $(ZK_PROVING_KEY)
	@$(MAKE) --no-print-directory nonos-mk-verify-capsule-attest
	@$(MAKE) --no-print-directory nonos-mk-zk-report

nonos-mk-verify-capsule-attest: nonos-mk-all-capsules-attested $(ZK_FLEET_TOOL) $(ZK_VERIFYING_KEY)
	@$(ZK_FLEET_TOOL) \
		--verifying-key $(ZK_VERIFYING_KEY) \
		--capsule-dir target/capsule-attest \
		--sidecar-dir $(NONOS_TRUST_DIR)/capsules

# Per-capsule attestation facts, parsed live from the NZKCAPS1 sidecars. The
# fixed columns (192-byte proof, 7 public inputs) are constant by construction;
# the three shown here are the per-capsule bindings carried in the public
# inputs: the granted capability mask, the blake3 of the capsule bytes, and the
# bound commitment. No two rows match.
nonos-mk-zk-report: nonos-mk-all-capsules-attested $(ZK_VERIFYING_KEY) $(ZK_POLICY_ROOT)
	@printf '\n  attestation report   groth16 / bls12-381   192-byte proofs   7 public inputs   vk %s\n\n' "$(NONOS_VK_FPR)"
	@printf '  %-22s  %-18s  %-18s  %-18s  %-18s\n' capsule 'caps mask' 'capsule blake3' 'policy root' commitment
	@printf '  %s\n' '-------------------------------------------------------------------------------------------------------------'
	@for t in $(NONOS_TRUST_DIR)/capsules/*.zk_trailer.bin; do \
		[ -e "$$t" ] || continue; \
		name=$$(basename "$$t" .zk_trailer.bin); \
		caps=$$(od -An -tx1 -j 360 -N 8 "$$t" | tr -d ' \n'); \
		chash=$$(od -An -tx1 -j 224 -N 8 "$$t" | tr -d ' \n'); \
		proot=$$(od -An -tx1 -j 272 -N 8 "$$t" | tr -d ' \n'); \
		comm=$$(od -An -tx1 -j 432 -N 8 "$$t" | tr -d ' \n'); \
		printf '  %-22s  0x%-16s  %-18s  %-18s  %-18s\n' "$$name" "$$caps" "$$chash" "$$proot" "$$comm"; \
	done
	@printf '\n  %s capsules   every row distinct   each proof binds these inputs in zero knowledge\n\n' "$(NONOS_ATTESTED)"

# Run a real Groth16 pairing check on every bundled capsule, one at a time,
# against the current verifying key. This is the actual math, not a parse:
# each line is a full verification as it completes. Needs a build first
# (target/capsule-attest/*.capsule.zk), and exits non-zero if any proof fails.
nonos-mk-zk-verify-live: nonos-mk-all-capsules-attested $(ZK_VERIFY_TOOL) $(ZK_VERIFYING_KEY)
	@printf '\n  live groth16 verification   bls12-381   vk %s\n\n' "$(NONOS_VK_FPR)"
	@pass=0; fail=0; \
	any=0; \
	for z in target/capsule-attest/*.capsule.zk; do \
		[ -e "$$z" ] || { printf '  no bundled capsules found; build first: make nonos-mk-desktop-gui-prod\n\n'; exit 1; }; \
		any=1; \
		name=$$(basename "$$z" .capsule.zk); \
		printf '  verifying %-24s ' "$$name"; \
		if $(ZK_VERIFY_TOOL) --verifying-key $(ZK_VERIFYING_KEY) --capsule "$$z" >/dev/null 2>&1; then \
			printf 'PASS\n'; pass=$$((pass + 1)); \
		else \
			printf 'FAIL\n'; fail=$$((fail + 1)); \
		fi; \
	done; \
	printf '\n  %s passed   %s failed   real pairing checks against vk %s\n\n' "$$pass" "$$fail" "$(NONOS_VK_FPR)"; \
	[ "$$fail" -eq 0 ]

nonos-mk-live-production-proof: nonos-mk-zk-verify-live nonos-mk-zk-report
	@echo "Production proof ready: full capsule fleet signed, attested, and pairing-verified."

# Verify capsule attestation with the real Groth16 pairing checks, live.
# Default: walk every capsule. CAP=<name>: full single-capsule view (proof
# points and all seven public inputs). SLOW sets the per-step delay.
CAP  ?=
SLOW ?= 0.5
nonos-mk-attestation: nonos-mk-all-capsules-attested $(ZK_VERIFY_TOOL) $(ZK_VERIFYING_KEY)
	@if [ -n "$(CAP)" ]; then \
		t=$(NONOS_TRUST_DIR)/capsules/$(CAP).zk_trailer.bin; \
		z=target/capsule-attest/$(CAP).capsule.zk; \
		{ [ -e "$$t" ] && [ -e "$$z" ]; } || { printf '\n  no artifacts for %s; build first: make nonos-mk-desktop-gui-prod\n\n' "$(CAP)"; exit 1; }; \
		printf '\n  groth16 capsule attestation   %s   bls12-381   vk %s\n\n' "$(CAP)" "$(NONOS_VK_FPR)"; sleep $(SLOW); \
		printf '  proof pi-A  (G1, 48 bytes)\n    %s\n' "$$(od -An -tx1 -j 12 -N 48 "$$t" | tr -d ' \n')"; sleep $(SLOW); \
		printf '  proof pi-B  (G2, 96 bytes)\n    %s\n' "$$(od -An -tx1 -j 60 -N 96 "$$t" | tr -d ' \n')"; sleep $(SLOW); \
		printf '  proof pi-C  (G1, 48 bytes)\n    %s\n\n' "$$(od -An -tx1 -j 156 -N 48 "$$t" | tr -d ' \n')"; sleep $(SLOW); \
		printf '  public inputs (7 field elements bound by the proof):\n'; \
		i=0; for l in capsule_hash_hi capsule_hash_lo policy_root policy_epoch capability_mask commitment_hi commitment_lo; do \
			off=$$((208 + i * 32)); \
			v=$$(od -An -tx1 -j $$off -N 32 "$$t" | tr -d ' \n'); \
			printf '    [%d] %-16s %s\n' "$$i" "$$l" "$$v"; \
			i=$$((i + 1)); sleep $(SLOW); \
		done; \
		printf '\n  e(A,B) == e(alpha,beta) . e(L,gamma) . e(C,delta)   ... '; sleep $(SLOW); \
		if $(ZK_VERIFY_TOOL) --verifying-key $(ZK_VERIFYING_KEY) --capsule "$$z" >/dev/null 2>&1; then \
			printf 'PASS\n\n  the proof satisfies the pairing equation under vk %s\n\n' "$(NONOS_VK_FPR)"; \
		else printf 'FAIL\n\n'; exit 1; fi; \
	else \
		total=$$(ls $(NONOS_TRUST_DIR)/capsules/*.zk_trailer.bin 2>/dev/null | wc -l | tr -d ' '); \
		printf '\n  groth16 capsule attestation   %s capsules   bls12-381   vk %s\n' "$$total" "$(NONOS_VK_FPR)"; \
		n=0; pass=0; fail=0; \
		for t in $(NONOS_TRUST_DIR)/capsules/*.zk_trailer.bin; do \
			[ -e "$$t" ] || continue; \
			name=$$(basename "$$t" .zk_trailer.bin); \
			z=target/capsule-attest/$$name.capsule.zk; \
			n=$$((n + 1)); \
			a=$$(od -An -tx1 -j 12 -N 24 "$$t" | tr -d ' \n'); \
			caps=$$(od -An -tx1 -j 360 -N 8 "$$t" | tr -d ' \n'); \
			chash=$$(od -An -tx1 -j 224 -N 16 "$$t" | tr -d ' \n'); \
			comm=$$(od -An -tx1 -j 432 -N 16 "$$t" | tr -d ' \n'); \
			printf '\n  [%2d/%s] %s\n' "$$n" "$$total" "$$name"; \
			printf '         proof-A %s..   caps 0x%s\n' "$$a" "$$caps"; \
			printf '         hash %s..   commit %s..\n' "$$chash" "$$comm"; \
			printf '         e(A,B)==e(a,b).e(L,g).e(C,d)  ... '; \
			if $(ZK_VERIFY_TOOL) --verifying-key $(ZK_VERIFYING_KEY) --capsule "$$z" >/dev/null 2>&1; then \
				printf 'PASS\n'; pass=$$((pass + 1)); \
			else printf 'FAIL\n'; fail=$$((fail + 1)); fi; \
			sleep $(SLOW); \
		done; \
		printf '\n  %s proofs verified   %s failed   every capsule, real pairing checks\n\n' "$$pass" "$$fail"; \
		[ "$$fail" -eq 0 ]; \
	fi

# Verify-only: re-run a real Groth16 pairing check on every committed
# capsule proof trailer against the committed public verifying key. This
# is the path a fresh clone runs to check the official release: it needs
# no signing keys, builds nothing, and re-signs nothing. Each trailer
# carries the 192-byte proof at offset 12 and the 7 public inputs at
# offset 208, the layout the kernel and the bootloader both parse.
NONOS_DIST_VK := $(NONOS_TRUST_DIR)/zk/attestation_verifying_key.bin
nonos-mk-verify-attestation: $(ZK_VERIFY_TOOL)
	@test -f $(NONOS_DIST_VK) || { printf '\n  committed verifying key missing: %s\n  clone with --recursive so the trust keystore is present.\n\n' "$(NONOS_DIST_VK)"; exit 1; }
	@mkdir -p $(dir $(NONOS_RECEIPT)); \
	fpr=$$(shasum -a 256 $(NONOS_DIST_VK) | cut -c1-16); \
	printf '\n  verifying the committed fleet against the public key %s\n  groth16 / bls12-381, no signing keys, nothing rebuilt\n\n' "$$fpr"; \
	{ printf '# NONOS capsule attestation receipt\n'; \
	  printf '# scheme groth16  curve bls12-381  proof 192B  public-inputs 7\n'; \
	  printf '# verifying-key %s\n#\n' "$$fpr"; } > $(NONOS_RECEIPT); \
	pass=0; fail=0; any=0; tmp=$$(mktemp -d); \
	for t in $(NONOS_TRUST_DIR)/capsules/*.zk_trailer.bin; do \
		[ -e "$$t" ] || continue; any=1; \
		name=$$(basename "$$t" .zk_trailer.bin); \
		dd if="$$t" of="$$tmp/p" bs=1 skip=12 count=192 2>/dev/null; \
		dd if="$$t" of="$$tmp/pi" bs=1 skip=208 count=224 2>/dev/null; \
		printf '  %-26s ' "$$name"; \
		if $(ZK_VERIFY_TOOL) --verifying-key $(NONOS_DIST_VK) --proof "$$tmp/p" --public-inputs "$$tmp/pi" >/dev/null 2>&1; then \
			printf 'verified\n'; printf '  %-26s verified\n' "$$name" >> $(NONOS_RECEIPT); pass=$$((pass + 1)); \
		else printf 'FAILED\n'; printf '  %-26s FAILED\n' "$$name" >> $(NONOS_RECEIPT); fail=$$((fail + 1)); fi; \
	done; \
	rm -rf "$$tmp"; \
	[ "$$any" -eq 1 ] || { printf '\n  no committed trailers found under %s\n\n' "$(NONOS_TRUST_DIR)/capsules"; exit 1; }; \
	printf '#\n# %s verified, %s failed against verifying-key %s\n' "$$pass" "$$fail" "$$fpr" >> $(NONOS_RECEIPT); \
	printf '\n  %s verified, %s failed against verifying-key %s\n  receipt: %s\n\n' "$$pass" "$$fail" "$$fpr" "$(NONOS_RECEIPT)"; \
	[ "$$fail" -eq 0 ]

# Emit an attestation receipt: one verifiable record per capsule with the public
# inputs, the proof digest, the verifying-key fingerprint, and the live verdict.
# A third party with the public verifying key can re-run the check from this.
NONOS_RECEIPT ?= target/attestation-receipt.txt
nonos-mk-attestation-receipt: nonos-mk-all-capsules-attested $(ZK_VERIFY_TOOL) $(ZK_VERIFYING_KEY)
	@mkdir -p $(dir $(NONOS_RECEIPT)); \
	{ printf '# NONOS capsule attestation receipt\n'; \
	  printf '# scheme groth16  curve bls12-381  proof 192B  public-inputs 7\n'; \
	  printf '# verifying-key %s\n#\n' "$(NONOS_VK_FPR)"; \
	  printf '# %-22s %-18s %-34s %-34s %s\n' capsule caps capsule_hash commitment verdict; \
	  pass=0; fail=0; \
	  for t in $(NONOS_TRUST_DIR)/capsules/*.zk_trailer.bin; do \
		[ -e "$$t" ] || continue; \
		name=$$(basename "$$t" .zk_trailer.bin); \
		z=target/capsule-attest/$$name.capsule.zk; \
		caps=$$(od -An -tx1 -j 360 -N 8 "$$t" | tr -d ' \n'); \
		chash=$$(od -An -tx1 -j 224 -N 16 "$$t" | tr -d ' \n')$$(od -An -tx1 -j 256 -N 1 "$$t" | tr -d ' \n'); \
		comm=$$(od -An -tx1 -j 432 -N 17 "$$t" | tr -d ' \n'); \
		if $(ZK_VERIFY_TOOL) --verifying-key $(ZK_VERIFYING_KEY) --capsule "$$z" >/dev/null 2>&1; then \
			v=verified; pass=$$((pass + 1)); else v=FAILED; fail=$$((fail + 1)); fi; \
		printf '  %-22s 0x%-16s %-34s %-34s %s\n' "$$name" "$$caps" "$$chash" "$$comm" "$$v"; \
	  done; \
	  printf '#\n# %s verified, %s failed against verifying-key %s\n' "$$pass" "$$fail" "$(NONOS_VK_FPR)"; \
	} > $(NONOS_RECEIPT); \
	cat $(NONOS_RECEIPT); \
	printf '\n  receipt written: %s\n\n' "$(NONOS_RECEIPT)"

# NOX contribution rail: canonical receipts for verifiable work, an
# independent re-checker, epoch claim trees, and the circuit registry
# export. nonos-mk-nox is the contributor entry point: one command from
# a clean tree to a verified, claimable receipt.
NOX_RECEIPT_TOOL  := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/nox-zk-receipt
NOX_WORK_TOOL     := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/nox-work-receipt
NOX_VERIFY_TOOL   := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/nox-receipt-verify
NOX_MERKLE_TOOL   := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/nox-merkle
NOX_REGISTRY_TOOL := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/nox-circuit-registry
ZK_TRANSCRIPT     := $(ZK_KEYS_DIR)/ceremony_transcript.json
NOX_DIR           := $(TARGET_DIR)/nox
NOX_SUBMIT_URL    ?= http://146.103.41.45:8484/submit
CONTRIB  ?=
EPOCH    ?= 1
KIND     ?= FLEET_VERIFICATION
ARTIFACT ?= $(NONOS_RECEIPT)
URI      ?=

$(NOX_RECEIPT_TOOL) $(NOX_WORK_TOOL) $(NOX_VERIFY_TOOL) $(NOX_MERKLE_TOOL) $(NOX_REGISTRY_TOOL): nonos-mk-check-deps
	@echo "Building NOX receipt tools..."
	@cd $(ZK_CIRCUIT_DIR) && RUSTFLAGS="" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --bin nox-zk-receipt --bin nox-work-receipt --bin nox-receipt-verify --bin nox-merkle --bin nox-circuit-registry --target $(HOST_TARGET)

nonos-mk-nox-tools: $(NOX_WORK_TOOL)

# Emit one receipt for verifiable work. KIND selects the evidence the
# emitter validates before issuing anything: FLEET_VERIFICATION (the
# attestation receipt), RUNTIME_BOOT / HARDWARE_BOOT (a serial boot log
# with enforced [ZK-ATTEST] lines), CIRCUIT_AUDIT / CAPSULE_AUDIT (a
# written report), CAPSULE_BUILD (a bundled capsule, re-proved live).
nonos-mk-nox-receipt: $(NOX_WORK_TOOL) nonos-mk-ensure-zk-keys
	@test -n "$(CONTRIB)" || { printf '\n  usage: make nonos-mk-nox-receipt CONTRIB=0x<your address> [KIND=%s] [ARTIFACT=path] [EPOCH=%s]\n\n' "$(KIND)" "$(EPOCH)"; exit 1; }
	@mkdir -p $(NOX_DIR)/receipts
	@$(NOX_WORK_TOOL) --kind $(KIND) --artifact $(ARTIFACT) \
		--verifying-key $(ZK_VERIFYING_KEY) --transcript $(ZK_TRANSCRIPT) \
		--contributor $(CONTRIB) --epoch $(EPOCH) --uri "$(URI)" \
		--out $(NOX_DIR)/receipts/$(KIND)-epoch$(EPOCH).json
	@printf '\n  receipt written: %s\n\n' "$(NOX_DIR)/receipts/$(KIND)-epoch$(EPOCH).json"

# Re-check a receipt from the raw artifacts, the way an authorizer does
# before signing anything. Prints the verifier hash on success.
nonos-mk-nox-verify: $(NOX_VERIFY_TOOL) nonos-mk-ensure-zk-keys
	@test -n "$(RECEIPT)" || { printf '\n  usage: make nonos-mk-nox-verify RECEIPT=path [ARTIFACT=path]\n\n'; exit 1; }
	@$(NOX_VERIFY_TOOL) --receipt $(RECEIPT) \
		--verifying-key $(ZK_VERIFYING_KEY) --transcript $(ZK_TRANSCRIPT) \
		$(if $(ARTIFACT),--artifact $(ARTIFACT),)

# Build the epoch claim tree from accepted receipts. CLAIMS is a JSON
# allocation file: {"epoch":N,"pool_id":"0x..","claims":[{"receipt":path,
# "amount":"wei"}]}. Writes root.json and claims.json with Merkle proofs
# that NoxRewardPool.claim verifies on-chain.
nonos-mk-nox-merkle: $(NOX_MERKLE_TOOL)
	@test -n "$(CLAIMS)" || { printf '\n  usage: make nonos-mk-nox-merkle CLAIMS=allocations.json [EPOCH=%s]\n\n' "$(EPOCH)"; exit 1; }
	@mkdir -p $(NOX_DIR)/epoch$(EPOCH)
	@$(NOX_MERKLE_TOOL) --claims $(CLAIMS) --out-dir $(NOX_DIR)/epoch$(EPOCH) \
		--deployment abi/nox_deployment.json
	@printf '\n  root and claims written: %s/\n\n' "$(NOX_DIR)/epoch$(EPOCH)"

# Send a receipt and its artifact to the submission endpoint, which
# re-runs the same verification library before accepting anything.
# Failure is never fatal: the receipt stays local and can be resent.
nonos-mk-nox-submit:
	@test -n "$(RECEIPT)" || { printf '\n  usage: make nonos-mk-nox-submit RECEIPT=path ARTIFACT=path\n\n'; exit 1; }
	@command -v curl >/dev/null || { printf '  curl not found; submit later\n'; exit 0; }
	@tmp=$$(mktemp); \
	{ printf '{"receipt":'; cat $(RECEIPT); printf ',"artifact_b64":"'; \
	  test -n "$(ARTIFACT)" && base64 < $(ARTIFACT) | tr -d '\n'; printf '"}'; } > $$tmp; \
	resp=$$(curl -s --max-time 60 -X POST --data-binary @$$tmp $(NOX_SUBMIT_URL)) || resp=unreachable; \
	rm -f $$tmp; \
	case "$$resp" in \
	*'"accepted":true'*) printf '\n  receipt submitted and verified: %s\n\n' "$$resp";; \
	*) printf '\n  submission not accepted (endpoint said: %s)\n  your receipt is safe locally; retry with:\n    make nonos-mk-nox-submit RECEIPT=$(RECEIPT) ARTIFACT=$(ARTIFACT)\n\n' "$$resp";; \
	esac

# Export the canonical circuit registry entry for NoxZkCircuitRegistry.
nonos-mk-nox-registry: $(NOX_REGISTRY_TOOL) nonos-mk-ensure-zk-keys
	@mkdir -p $(NOX_DIR)
	@$(NOX_REGISTRY_TOOL) --circuit-dir $(ZK_CIRCUIT_DIR) \
		--cargo-lock $(ZK_CIRCUIT_DIR)/Cargo.lock \
		--verifying-key $(ZK_VERIFYING_KEY) --transcript $(ZK_TRANSCRIPT) \
		--uri "$(URI)" --out $(NOX_DIR)/circuit.json
	@printf '\n  registry entry written: %s\n\n' "$(NOX_DIR)/circuit.json"

# The contributor command. One invocation from a clean checkout: builds
# the toolchain and every capsule, runs the live Groth16 pairing check on
# the whole fleet, issues a FLEET_VERIFICATION receipt for your address,
# re-verifies that receipt independently, and exports the circuit
# registry entry. Everything lands under target/nox/.
nonos-mk-nox:
	@test -n "$(CONTRIB)" || { printf '\n  usage: make nonos-mk-nox CONTRIB=0x<your reward address> [EPOCH=%s]\n\n' "$(EPOCH)"; exit 1; }
	@printf '\n  NONOS x NOX contributor run   groth16 / bls12-381\n\n'
	@$(MAKE) nonos-mk-verify-attestation
	@$(MAKE) nonos-mk-nox-receipt CONTRIB=$(CONTRIB) KIND=FLEET_VERIFICATION ARTIFACT=$(NONOS_RECEIPT) EPOCH=$(EPOCH)
	@$(MAKE) nonos-mk-nox-verify RECEIPT=$(NOX_DIR)/receipts/FLEET_VERIFICATION-epoch$(EPOCH).json ARTIFACT=$(NONOS_RECEIPT)
	@$(MAKE) nonos-mk-nox-registry
	@$(MAKE) nonos-mk-nox-submit RECEIPT=$(NOX_DIR)/receipts/FLEET_VERIFICATION-epoch$(EPOCH).json ARTIFACT=$(NONOS_RECEIPT)
	@printf '\n  done. your receipt: %s\n' "$(NOX_DIR)/receipts/FLEET_VERIFICATION-epoch$(EPOCH).json"
	@printf '  registry entry:    %s\n' "$(NOX_DIR)/circuit.json"
	@printf '  the receipt was also submitted for the epoch root. see CONTRIBUTING-ZK.md\n\n'

.PHONY: nonos-mk-nox nonos-mk-nox-tools nonos-mk-nox-receipt nonos-mk-nox-verify nonos-mk-nox-merkle nonos-mk-nox-registry nonos-mk-nox-submit

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
	@# rust-src is mandatory: -Zbuild-std cannot build core/alloc without it.
	@$(RUSTUP) component add rust-src --toolchain $(TOOLCHAIN) || { \
		echo "::error::failed to add rust-src to $(TOOLCHAIN); -Zbuild-std needs it"; exit 1; }
	@# clippy and rustfmt are best-effort (lint/format only).
	@$(RUSTUP) component add clippy rustfmt --toolchain $(TOOLCHAIN) 2>/dev/null || true
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
# The std startup object lives in the build tree, not /tmp. It must be an
# absolute path because capsules build from their own subdirectories and pass
# it to the linker as -Clink-arg. cargo clean removes it; the rule below
# rebuilds it. Override NONOS_RT_OBJ only if you have a reason to.
NONOS_RT_OBJ  ?= $(abspath $(TARGET_DIR))/nonos_rt.o
USERLAND_LIBC_SRCS := $(shell find $(USERLAND_DIR)/libc/src -name '*.rs') \
                      $(USERLAND_DIR)/libc/Cargo.toml \
                      $(USERLAND_DIR)/x86_64-nonos-user.json
NONOS_RT_SRCS := $(shell find toolchain/nonos-rt/src -name '*.rs' 2>/dev/null) \
                 toolchain/nonos-rt/Cargo.toml \
                 toolchain/nonos-rt/Cargo.lock \
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

# Order-only dependency on the toolchain stamp: -Zbuild-std needs the
# rust-src component, and a make-level RUSTUP_TOOLCHAIN override skips the
# component list in rust-toolchain.toml, so the stamp must install it
# before the first userland build a fresh checkout ever runs.
$(USERLAND_LIBC): $(USERLAND_LIBC_SRCS) | $(TARGET_DIR)/.nonos-toolchain.stamp
	@echo "Building userland libc..."
	@cd $(USERLAND_DIR)/libc && \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --target ../x86_64-nonos-user.json \
		-Zbuild-std=core
	@touch $@

nonos-mk-libc: $(USERLAND_LIBC)

$(NONOS_RT_OBJ): $(NONOS_RT_SRCS) | $(TARGET_DIR)/.nonos-toolchain.stamp
	@echo "Building NONOS std startup object..."
	@cd toolchain/nonos-rt && \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) rustc --release --target ../../userland/x86_64-nonos-user.json \
		-Zbuild-std=core -- --emit obj=$(NONOS_RT_OBJ)

# std platform layer: patch the pinned rust-src so -Zbuild-std=std turns
# unmodified `use std::...` crates into NONOS binaries. Stamped and keyed on
# the PAL sources + apply.sh, so a clean checkout (or a rustup update that
# reset rust-src) re-applies it before any std capsule builds. Without this
# the patch was a manual prerequisite — hidden state that broke fresh trees.
NONOS_STD_PAL_SRCS  := $(shell find toolchain/nonos-std -type f 2>/dev/null)
NONOS_STD_PAL_STAMP := $(TARGET_DIR)/.nonos-std-pal.stamp
$(NONOS_STD_PAL_STAMP): $(NONOS_STD_PAL_SRCS) | $(TARGET_DIR)/.nonos-toolchain.stamp
	@echo "Applying NONOS std platform layer to rust-src..."
	@PATH="$(HOME)/.cargo/bin:$$PATH" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		toolchain/nonos-std/apply.sh
	@mkdir -p $(TARGET_DIR)
	@touch $@

.PHONY: nonos-mk-apply-std
nonos-mk-apply-std: $(NONOS_STD_PAL_STAMP)

# Unmodified crates.io binaries, built for the NONOS target through the std
# PAL + nonos-rt start object. This is the reproducible source of the
# ripgrep ELF the kernel mirror and the VFS bootstrap store embed; it
# replaces a hand-built prebuilt that had no rule (so `cargo clean` bricked
# the tree). cargo install pulls the pinned version from crates.io and runs
# its own upstream main; no source is patched.
UPSTREAM_RIPGREP_VERSION := 14.1.1
UPSTREAM_RIPGREP_BIN     := $(TARGET_DIR)/upstream-ripgrep/rg
$(UPSTREAM_RIPGREP_BIN): $(NONOS_RT_OBJ) $(NONOS_STD_PAL_STAMP) \
		userland/x86_64-nonos-user.json | $(TARGET_DIR)/.nonos-toolchain.stamp
	@echo "Building upstream ripgrep $(UPSTREAM_RIPGREP_VERSION) for NONOS (unmodified crates.io source)..."
	@RUSTUP_TOOLCHAIN=$(TOOLCHAIN) RUSTFLAGS="-Clink-arg=$(abspath $(NONOS_RT_OBJ))" \
		$(CARGO) install ripgrep --version $(UPSTREAM_RIPGREP_VERSION) \
		--target $(abspath userland/x86_64-nonos-user.json) \
		-Zbuild-std=std,panic_abort -Zbuild-std-features=compiler-builtins-mem \
		--root $(abspath $(TARGET_DIR)/upstream-ripgrep) --no-track --force --bin rg
	@cp $(TARGET_DIR)/upstream-ripgrep/bin/rg $@

.PHONY: nonos-mk-upstream-ripgrep
nonos-mk-upstream-ripgrep: $(UPSTREAM_RIPGREP_BIN)

$(CAPSULE_SIGN_BIN):
	@echo "Building capsule-sign host tool..."
	@cd nonos-sign && cargo build --release --bin capsule-sign

.PHONY: nonos-mk-host-trust-elfs
nonos-mk-host-trust-elfs: $(USERLAND_LIBC) $(MARKETPLACE_ABI_LIB) \
		$(proof-io_BIN) $(ramfs_BIN) $(keyring_BIN) $(entropy_BIN) \
		$(crypto_BIN) $(vfs_BIN) $(market_BIN) $(driver-virtio-rng_BIN) \
		$(driver-virtio-gpu_BIN) $(driver-ps2-input_BIN) $(driver-virtio-blk_BIN) \
		$(driver-virtio-net_BIN) $(driver-xhci_BIN) $(driver-usb-hid_BIN) \
		$(driver-e1000_BIN) \
		$(net-l2_BIN) $(net-ip_BIN) $(net-udp_BIN) $(net-dhcp_BIN) \
		$(input-router_BIN) $(compositor_BIN) $(wm_BIN) $(desktop-shell_BIN) \
		$(image-codec_BIN) $(clipboard_BIN) $(login_BIN) $(wallpaper_BIN) \
		$(toolkit_BIN) $(about_BIN) $(calculator_BIN) $(snake_BIN) $(terminal_BIN) \
		$(file-manager_BIN) $(text-editor_BIN) $(settings_BIN) $(process-manager_BIN)
	@echo "Capsule ELFs built for the host-trust artifact proof."

nonos-mk-host-trust-verify: nonos-mk-host-trust-elfs nonos-mk-check-trust-manifest
	@echo "Capsule trust artifacts are present and match the baked SHA-256 ledger."

nonos-mk-check-trust-manifest:
	@echo "Verifying baked trust artifact SHA-256 ledger..."
	@cd $(NONOS_TRUST_DIR) && $(SHA256) -c MANIFEST.sha256

nonos-mk-verify-trust: nonos-mk-desktop-gui-prod
	@$(MAKE) nonos-mk-host-trust-verify
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
include userland/capsule_std_proof/Capsule.mk
include userland/capsule_ripgrep/Capsule.mk
include userland/capsule_ramfs/Capsule.mk
include userland/capsule_keyring/Capsule.mk
include userland/capsule_entropy/Capsule.mk
include userland/capsule_crypto/Capsule.mk
include userland/compositor/Capsule.mk
include userland/capsule_input_router/Capsule.mk
include userland/capsule_input_proof/Capsule.mk
include userland/capsule_input_probe/Capsule.mk
include userland/capsule_setup_wizard/Capsule.mk
include userland/capsule_wm/Capsule.mk
include userland/capsule_desktop_shell/Capsule.mk
include userland/capsule_image_codec/Capsule.mk
include userland/capsule_clipboard/Capsule.mk
include userland/capsule_login/Capsule.mk
include userland/toolkit/Capsule.mk
include userland/capsule_about/Capsule.mk
include userland/capsule_hello/Capsule.mk
include userland/capsule_boot_splash/Capsule.mk
include userland/capsule_calculator/Capsule.mk
include userland/capsule_snake/Capsule.mk
include userland/capsule_wallet_nonos/Capsule.mk
include userland/capsule_terminal/Capsule.mk
include userland/capsule_file_manager/Capsule.mk
include userland/capsule_text_editor/Capsule.mk
include userland/capsule_policy/Capsule.mk
include userland/capsule_wallpaper_catalog/Capsule.mk
include userland/capsule_settings/Capsule.mk
include userland/capsule_process_manager/Capsule.mk
include userland/capsule_vfs/Capsule.mk
include userland/capsule_market/Capsule.mk
include userland/capsule_payment/Capsule.mk
include userland/capsule_installer/Capsule.mk
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
include userland/capsule_net_core/Capsule.mk
include userland/capsule_net_sockets/Capsule.mk
include userland/capsule_net_nym/Capsule.mk
include userland/capsule_wallpaper/Capsule.mk
include userland/capsule_attest/Capsule.mk
include userland/capsule_power/Capsule.mk

# Orchestration helper: union of every verified capsule's artifact
# triple. Smoke and test targets that need proof_io plus another
# capsule depend on `$(proof-io_ARTIFACTS)` directly.
NONOS_VERIFIED_ARTIFACTS = $(foreach slug,$(NONOS_VERIFIED_CAPSULES),$($(slug)_ARTIFACTS))
NONOS_ZK_POLICY_INPUTS = $(foreach slug,$(NONOS_VERIFIED_CAPSULES),$($(slug)_BIN) $($(slug)_CAPSULE_MK))

$(ZK_POLICY_FILE): $(NONOS_ZK_POLICY_INPUTS)
	@mkdir -p $(dir $@)
	@{ \
	$(foreach slug,$(NONOS_VERIFIED_CAPSULES),printf '%s\t%s\t%s\n' '$($(slug)_BIN_NAME)' '$($(slug)_BIN)' '$($(slug)_REQUIRED_CAPS)';) \
	} > $@

$(ZK_POLICY_ROOT): $(ZK_POLICY_FILE) $(ZK_POLICY_TOOL)
	@mkdir -p $(dir $@)
	@$(ZK_POLICY_TOOL) --policy-file $(ZK_POLICY_FILE) --output $@ >/dev/null

nonos-mk-all-capsules-attested: $(NONOS_VERIFIED_ARTIFACTS)
	@echo "Signed and attested $(words $(NONOS_VERIFIED_CAPSULES)) included capsules."

NONOS_DESKTOP_GUI_CAPSULE_CHECKS = \
	$(proof-io_VERIFY) $(std-proof_VERIFY) $(ripgrep_VERIFY) \
	$(ramfs_VERIFY) $(keyring_VERIFY) \
	$(entropy_VERIFY) $(crypto_VERIFY) $(vfs_VERIFY) \
	$(driver-virtio-rng_VERIFY) $(driver-virtio-blk_VERIFY) \
	$(driver-virtio-gpu_VERIFY) $(driver-virtio-net_VERIFY) \
	$(driver-ps2-input_VERIFY) $(driver-xhci_VERIFY) \
	$(driver-usb-hid_VERIFY) \
	$(net-l2_VERIFY) $(net-ip_VERIFY) $(net-udp_VERIFY) \
	$(net-dhcp_VERIFY) $(net-tcp_VERIFY) $(net-dns_VERIFY) \
	$(net-sockets_VERIFY) $(net-nym_VERIFY) \
	$(policy_VERIFY) $(wallpaper_catalog_VERIFY) \
	$(installer_VERIFY) \
	$(input-router_VERIFY) $(compositor_VERIFY) $(wm_VERIFY) \
	$(desktop-shell_VERIFY) $(image-codec_VERIFY) $(clipboard_VERIFY) \
	$(login_VERIFY) $(wallpaper_VERIFY) $(toolkit_VERIFY) \
	$(boot-splash_VERIFY) $(about_VERIFY) $(calculator_VERIFY) \
	$(snake_VERIFY) $(wallet-nonos_VERIFY) $(terminal_VERIFY) \
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

nonos-mk-std-proof-prod: $(proof-io_ARTIFACTS) $(std-proof_ARTIFACTS) nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-proof-io + std_proof)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-proof-io,nonos-capsule-std-proof

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

nonos-mk-desktop-gui-prod: $(proof-io_ARTIFACTS) \
		$(std-proof_ARTIFACTS) $(ripgrep_ARTIFACTS) $(ramfs_ARTIFACTS) \
		$(keyring_ARTIFACTS) $(entropy_ARTIFACTS) $(crypto_ARTIFACTS) \
		$(vfs_ARTIFACTS) $(driver-virtio-rng_ARTIFACTS) \
		$(driver-virtio-blk_ARTIFACTS) $(driver-virtio-gpu_ARTIFACTS) \
		$(driver-virtio-net_ARTIFACTS) $(driver-ps2-input_ARTIFACTS) \
		$(driver-xhci_ARTIFACTS) $(driver-usb-hid_ARTIFACTS) \
		$(net-l2_ARTIFACTS) \
		$(net-ip_ARTIFACTS) $(net-udp_ARTIFACTS) $(net-dhcp_ARTIFACTS) \
		$(net-tcp_ARTIFACTS) $(net-dns_ARTIFACTS) $(net-sockets_ARTIFACTS) \
		$(net-nym_ARTIFACTS) \
		$(policy_ARTIFACTS) $(wallpaper_catalog_ARTIFACTS) \
		$(installer_ARTIFACTS) \
		$(input-router_ARTIFACTS) $(compositor_ARTIFACTS) \
		$(wm_ARTIFACTS) $(desktop-shell_ARTIFACTS) \
		$(image-codec_ARTIFACTS) $(clipboard_ARTIFACTS) \
		$(login_ARTIFACTS) $(wallpaper_ARTIFACTS) \
		$(toolkit_ARTIFACTS) $(about_ARTIFACTS) $(boot-splash_ARTIFACTS) \
		$(calculator_ARTIFACTS) $(snake_ARTIFACTS) \
		$(wallet-nonos_ARTIFACTS) $(terminal_ARTIFACTS) \
		$(file-manager_ARTIFACTS) $(text-editor_ARTIFACTS) \
		$(settings_ARTIFACTS) $(process-manager_ARTIFACTS) \
		$(attest_ARTIFACTS) $(power_ARTIFACTS) \
		$(ZK_POLICY_ROOT) \
		nonos-mk-verify-desktop-gui-capsules \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-desktop-gui)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-desktop-gui

nonos-mk-full-gui-prod: nonos-mk-all-capsules-attested \
		nonos-mk-verify-desktop-gui-capsules \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-full-gui)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-full-gui

nonos-mk-setup-wizard-prod: $(proof-io_ARTIFACTS) $(ramfs_ARTIFACTS) \
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
		$(toolkit_ARTIFACTS) $(about_ARTIFACTS) $(boot-splash_ARTIFACTS) \
		$(calculator_ARTIFACTS) $(snake_ARTIFACTS) $(terminal_ARTIFACTS) \
		$(file-manager_ARTIFACTS) $(text-editor_ARTIFACTS) \
		$(settings_ARTIFACTS) $(process-manager_ARTIFACTS) \
		$(attest_ARTIFACTS) $(power_ARTIFACTS) \
		$(setup-wizard_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-setup-wizard)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-setup-wizard

nonos-mk-setup-wizard-inject-prod: $(proof-io_ARTIFACTS) $(ramfs_ARTIFACTS) \
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
		$(toolkit_ARTIFACTS) $(about_ARTIFACTS) $(boot-splash_ARTIFACTS) \
		$(calculator_ARTIFACTS) $(snake_ARTIFACTS) $(terminal_ARTIFACTS) \
		$(file-manager_ARTIFACTS) $(text-editor_ARTIFACTS) \
		$(settings_ARTIFACTS) $(process-manager_ARTIFACTS) \
		$(attest_ARTIFACTS) $(power_ARTIFACTS) \
		$(setup-wizard_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-setup-wizard + inject)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-setup-wizard,input-probe-inject

nonos-mk-toolkit-prod: nonos-mk-desktop-gui-prod
nonos-mk-about-prod: nonos-mk-desktop-gui-prod
nonos-mk-calculator-prod: nonos-mk-desktop-gui-prod
nonos-mk-snake-prod: nonos-mk-desktop-gui-prod
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

nonos-mk-terminal-test: $(proof-io_ARTIFACTS) $(ramfs_ARTIFACTS) $(keyring_ARTIFACTS) \
		$(entropy_ARTIFACTS) $(crypto_ARTIFACTS) $(vfs_ARTIFACTS) \
		$(driver-virtio-rng_ARTIFACTS) nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building terminal capsule (autorun-selftest)..."
	@$(MAKE) -B terminal_CARGO_FEATURES=nonos-autorun-selftest nonos-mk-terminal-sign
	@echo "Building kernel (microkernel-terminal-smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-terminal-smoketest

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

nonos-mk-run: nonos-mk-live-production-proof nonos-mk-desktop-gui-prod nonos-mk-esp $(QEMU_BLK_IMG) $(QEMU_OVMF_VARS_RW)
	@echo "Booting NONOS in QEMU..."
	@echo "  Network: $(QEMU_NET_DESC)"
	@echo "  Quit: Ctrl+A then X"
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
		-drive if=pflash,format=raw,unit=0,readonly=on,file="$(OVMF)" \
		-drive if=pflash,format=raw,unit=1,file="$(QEMU_OVMF_VARS_RW)" \
		$(QEMU_BLK) $(QEMU_GPU) $(QEMU_NET) $(QEMU_USB) $(QEMU_RNG) \
		-serial mon:stdio -vga none -display cocoa,zoom-to-fit=on -no-reboot

nonos-mk-run-net:
	@$(MAKE) --no-print-directory QEMU_NET_MODE=hostfwd nonos-mk-run

nonos-mk-run-nat:
	@$(MAKE) --no-print-directory QEMU_NET_MODE=nat nonos-mk-run

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

nonos-mk-run-serial: nonos-mk-desktop-gui-prod nonos-mk-esp
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

nonos-mk-run-serial-log: nonos-mk-desktop-gui-prod nonos-mk-esp
	@mkdir -p $(dir $(QEMU_SERIAL_LOG))
	@echo "Booting NONOS serial console in QEMU..."
	@echo "  Network: $(QEMU_NET_DESC)"
	@echo "  Serial log: $(QEMU_SERIAL_LOG)"
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
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

.PHONY: nonos-mk-boot-terminal
nonos-mk-boot-terminal:
	@./tests/boot/terminal_round_trip.sh; rc=$$?; \
		echo "Restoring GUI terminal capsule (undo autorun-selftest artifact)..."; \
		$(MAKE) -B nonos-mk-terminal-sign >/dev/null 2>&1; \
		exit $$rc

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
	@echo "  make nonos-mk-run             QEMU + OVMF, network disabled by default"
	@echo "  make nonos-mk-run-nat         QEMU + OVMF with outbound NAT network"
	@echo "  make nonos-mk-run-net         QEMU + OVMF with explicit hostfwd network"
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
