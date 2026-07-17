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
#   make nonos-mk-run          full OS under QEMU + OVMF + TPM + NAT
#   make nonos-mk-verify       static + trust gates + desktop build + symbol scan
#
# A few old names (`kernel-capsules`, `kernel-with-keyring`,
# etc.) forward to `nonos-mk-*` for transitional compatibility. See the
# bottom of the file.

.PHONY: help

# Public nonos-mk-* targets
.PHONY: nonos-mk
.PHONY: nonos-mk-check nonos-mk-check-ramfs-keys nonos-mk-core nonos-mk-core-attested nonos-mk-menuconfig nonos-mk-from-config nonos-mk-capsules nonos-mk-terminal-test
.PHONY: nonos-mk-proof-io-prod nonos-mk-ramfs-prod nonos-mk-keyring-prod nonos-mk-entropy-prod nonos-mk-crypto-prod nonos-mk-vfs-prod nonos-mk-market-prod nonos-mk-driver-virtio-rng-prod nonos-mk-driver-virtio-blk-prod nonos-mk-driver-virtio-gpu-prod nonos-mk-driver-virtio-net-prod nonos-mk-driver-iwlwifi-prod nonos-mk-driver-rtl8821ce-prod nonos-mk-driver-i2c-pci-prod nonos-mk-driver-i2c-hid-prod nonos-mk-driver-ps2-input-prod nonos-mk-driver-xhci-prod nonos-mk-driver-usb-hid-prod nonos-mk-driver-usb-msc-prod nonos-mk-driver-e1000-prod nonos-mk-driver-rtl8139-prod nonos-mk-driver-rtl8169-prod nonos-mk-driver-ahci-prod nonos-mk-driver-hda-prod nonos-mk-driver-nvme-prod nonos-mk-net-l2-prod nonos-mk-net-core-prod nonos-mk-net-ip-prod nonos-mk-net-udp-prod nonos-mk-net-dhcp-prod nonos-mk-net-nym-prod nonos-mk-net-sockets-prod nonos-mk-desktop-gui-prod nonos-mk-zerostate
.PHONY: nonos-mk-libc nonos-mk-proof-io nonos-mk-proof-io-sign nonos-mk-check-trust-keys nonos-mk-check-trust-manifest nonos-mk-trust-policy nonos-mk-host-trust-verify nonos-mk-verify-trust nonos-mk-ramfs nonos-mk-ramfs-sign nonos-mk-keyring nonos-mk-entropy nonos-mk-crypto nonos-mk-vfs nonos-mk-virtio-rng nonos-mk-virtio-rng-sign nonos-mk-check-virtio-rng-keys nonos-mk-virtio-blk nonos-mk-virtio-blk-sign nonos-mk-check-virtio-blk-keys nonos-mk-driver-virtio-gpu nonos-mk-driver-virtio-gpu-sign nonos-mk-check-driver-virtio-gpu-keys nonos-mk-virtio-net nonos-mk-virtio-net-sign nonos-mk-check-virtio-net-keys nonos-mk-driver-iwlwifi nonos-mk-driver-iwlwifi-sign nonos-mk-check-driver-iwlwifi-keys nonos-mk-driver-rtl8821ce nonos-mk-driver-rtl8821ce-sign nonos-mk-check-driver-rtl8821ce-keys nonos-mk-driver-i2c-pci nonos-mk-driver-i2c-pci-sign nonos-mk-check-driver-i2c-pci-keys nonos-mk-driver-i2c-hid nonos-mk-driver-i2c-hid-sign nonos-mk-check-driver-i2c-hid-keys nonos-mk-ps2-input nonos-mk-ps2-input-sign nonos-mk-check-ps2-input-keys nonos-mk-xhci nonos-mk-xhci-sign nonos-mk-check-xhci-keys nonos-mk-driver-usb-msc nonos-mk-driver-usb-msc-sign nonos-mk-check-driver-usb-msc-keys nonos-mk-driver-e1000 nonos-mk-driver-e1000-sign nonos-mk-check-driver-e1000-keys nonos-mk-driver-rtl8139 nonos-mk-driver-rtl8139-sign nonos-mk-check-driver-rtl8139-keys nonos-mk-driver-rtl8169 nonos-mk-driver-rtl8169-sign nonos-mk-check-driver-rtl8169-keys nonos-mk-driver-ahci nonos-mk-driver-ahci-sign nonos-mk-check-driver-ahci-keys nonos-mk-driver-hda nonos-mk-driver-hda-sign nonos-mk-check-driver-hda-keys nonos-mk-driver-nvme nonos-mk-driver-nvme-sign nonos-mk-check-driver-nvme-keys nonos-mk-wallpaper nonos-mk-marketplace-abi nonos-mk-market nonos-mk-marketplace-index-tool
.PHONY: nonos-mk-userland-clean
.PHONY: nonos-mk-bootloader nonos-mk-sign nonos-mk-attest nonos-mk-esp nonos-mk-usb-img nonos-mk-usb-run
.PHONY: nonos-mk-run nonos-mk-run-nat nonos-mk-run-net nonos-mk-run-serial nonos-mk-run-serial-nat nonos-mk-run-serial-net nonos-mk-run-serial-log nonos-mk-run-input-probe-inject-serial-log nonos-mk-debug nonos-mk-plan-a-runtime nonos-mk-swtpm-start nonos-mk-swtpm-stop
.PHONY: nonos-mk-static nonos-mk-scan
.PHONY: nonos-mk-verify nonos-mk-verify-fast
.PHONY: nonos-mk-release-audit nonos-mk-claims-check nonos-mk-qemu-net-audit
.PHONY: nonos-mk-bench nonos-mk-bench-host nonos-mk-bench-boot-log
.PHONY: nonos-mk-bench-collect nonos-mk-bench-compare
.PHONY: ci-fast ci-security ci-release ci-soak nonos-mk-no-telemetry-capture nonos-mk-boot-evidence nonos-mk-hardware-dossier nonos-mk-validate-machine-metadata
.PHONY: nonos-mk-release
.PHONY: nonos-mk-clean nonos-mk-clean-all nonos-mk-distclean
.PHONY: nonos-mk-fmt
.PHONY: nonos-mk-toolchain nonos-mk-check-deps
.PHONY: nonos-mk-ensure-signing-key nonos-mk-ensure-zk-keys nonos-mk-zk-tools nonos-mk-all-capsules-attested nonos-mk-verify-capsule-attest nonos-mk-zk-report nonos-mk-zk-verify-live nonos-mk-live-production-proof nonos-mk-attestation nonos-mk-attestation-receipt

# Compatibility aliases (transitional, do not remove until callers move)
.PHONY: kernel-capsules kernel-with-keyring
.PHONY: check-static clean-kernel-only microkernel-symbol-scan

# Default target: print help, never build silently.

.DEFAULT_GOAL := help

# User build configuration written by the menuconfig-style tool. Optional: the
# dash means a missing file is not an error, so the fixed targets work without
# it. When present it defines NONOS_PROFILE and the security toggles that
# "make from-config" reads.
-include .nonos-config

# Configuration

BOOTLOADER_DIR := nonos-bootloader
TARGET_DIR     := target
ESP_DIR        := $(TARGET_DIR)/esp
USB_IMG        := $(TARGET_DIR)/nonos.img
# Total image size in MiB. The kernel alone is ~68 MB, so keep generous headroom.
USB_IMG_MB     ?= 256
KEYS_DIR       := $(BOOTLOADER_DIR)/keys

export SOURCE_DATE_EPOCH ?= $(shell git log -1 --format=%ct 2>/dev/null || date +%s)
export CARGO_INCREMENTAL := 0

export PATH := $(HOME)/.cargo/bin:$(PATH)
TOOLCHAIN := nightly-2026-01-16
CARGO     := $(HOME)/.cargo/bin/cargo
RUSTUP    := $(HOME)/.cargo/bin/rustup
NONOS_PYTHON ?= /usr/bin/python3
NONOS_BENCH_OUT ?=
NONOS_BENCH_BUILD_CMD ?= make nonos-mk-verify-fast
NONOS_BENCH_SKIP_BUILD ?= 0
NONOS_BENCH_SKIP_BOOT ?= 0
NONOS_BENCH_STRICT ?= 0
NONOS_BENCH_BOOT_TIMEOUT ?= 360
NONOS_BENCH_BOOT_CMD ?=
NONOS_BENCH_HOST_OUT ?= target/bench/host
NONOS_BENCH_BOOT_JSON ?= target/bench/boot-log.json
NONOS_BENCH_SERIAL_LOG ?=
NONOS_BENCH_BASELINE ?=
NONOS_BENCH_CANDIDATE ?=
NONOS_BENCH_REGRESSION_LIMIT ?= 15

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
KERNEL_MLDSA65_PREFIX ?= $(KEYS_DIR)/kernel_mldsa65
KERNEL_MLDSA65_KEY ?= $(KERNEL_MLDSA65_PREFIX).seed
KERNEL_MLDSA65_PUB ?= $(KERNEL_MLDSA65_PREFIX).pub
NONOS_TRUST_DIR := nonos-data/trust

# Transparent ZK attestation paths.
ZK_CIRCUIT_DIR   := $(BOOTLOADER_DIR)/tools/nonos-attestation-circuit
ZK_KEYS_DIR      := $(ZK_CIRCUIT_DIR)/generated_keys
ZK_ENROLL_TOOL   := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/transparent-enroll
ZK_TRANSPARENT_PROVE_TOOL := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/transparent-prove
ZK_TRANSPARENT_VERIFY_TOOL := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/transparent-verify
ZK_CAPSULE_PROOF_TOOL := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/capsule-attest-proof
# Transparent post-quantum STARK enrollment. Produces the capsule policy root
# and every capsule's money-grade membership trailer, verified by the same
# nonos-stark the kernel and bootloader link. This is the production capsule
# attestation path; the curve-based capsule-attest-proof above is retired.
NONOS_STARK_ENROLL := nonos-stark-enroll/target/$(HOST_TARGET)/release/nonos-stark-enroll
CAPSULE_SIGN_BIN := nonos-sign/target/release/capsule-sign
ZK_BOOT_ROOT     ?= $(NONOS_TRUST_DIR)/zk/device_root.bin
ZK_BOOT_COMMITMENTS ?= $(NONOS_TRUST_DIR)/zk/device_commitments.bin
ZK_BOOT_LABELS   ?= $(NONOS_TRUST_DIR)/zk/device_labels.txt
ZK_BOOT_SECRETS  ?= $(NONOS_TRUST_DIR)/zk/device_secrets.txt
ZK_BOOT_ENROLL_SEED ?=
ZK_BOOT_INDEX    ?=
ZK_BOOT_SECRET_X ?=
ZK_BOOT_SECRET_R ?=
ZK_BOOT_NONCE    ?=
ZK_BOOT_MACHINE_ID ?=
ZK_BOOT_TIMESTAMP ?=
ZK_BOOT_NONCE_SEED ?=
ZK_BOOT_CHALLENGE ?=
ZK_BOOT_SIDECAR  ?= $(ESP_DIR)/EFI/nonos/boot.zkp
ZK_CAPSULE_ROOT  ?= $(NONOS_TRUST_DIR)/policy/zk_capsule_policy_root.bin
# The kernel embeds this root at compile time (security/capsule_attest), so an
# attested kernel build depends on the capsule enrollment having run first.
ZK_POLICY_ROOT   ?= $(ZK_CAPSULE_ROOT)
# Kernel self-attestation: the bootloader embeds this root and verifies the
# kernel's own STARK membership trailer before the jump. Provisioned by the
# kernel enrollment step below. Opt in with NONOS_STARK_KERNEL_ATTEST=1.
KERNEL_ATTEST_ROOT_BIN ?= $(NONOS_TRUST_DIR)/policy/kernel_attest_root.bin
KERNEL_ATTEST_TRAILER  ?= $(TARGET_DIR)/kernel-attest/kernel.zk_trailer.bin
KERNEL_ATTEST_ELF      ?= $(TARGET_DIR)/x86_64-nonos/release/nonos-kernel
_boot_comma := ,
BOOT_STARK_FEATURE := $(if $(NONOS_STARK_KERNEL_ATTEST),$(_boot_comma)stark-kernel-attest)
ZK_CAPSULE_LABELS ?= $(TARGET_DIR)/capsule-attest/capsule_labels.txt
ZK_CAPSULE_SECRETS ?= $(TARGET_DIR)/capsule-attest/capsule_secrets.txt
ZK_CAPSULE_COMMITMENTS ?= $(TARGET_DIR)/capsule-attest/capsule_commitments.bin
ZK_CAPSULE_ENROLL_SEED ?=
ZK_CAPSULE_NONCE_SEED ?=
ZK_CAPSULE_EPOCH ?= 1
EMBED_TOOL       := $(BOOTLOADER_DIR)/tools/embed-zk-proof/target/$(HOST_TARGET)/release/embed-zk-proof
SIGN_TOOL        := $(BOOTLOADER_DIR)/tools/sign-kernel/target/$(HOST_TARGET)/release/sign-kernel

# Anti-rollback index baked into and signed over the kernel image. Starts at 1,
# the minimum the bootloader accepts (index 0 means unset and is rejected); bump
# only for security-critical releases. The TPM monotonic counter enforces it as
# a floor so an older signed kernel cannot be rolled back onto a device.
NONOS_ROLLBACK_INDEX ?= 1

# Changing the index must force a re-sign: the signed kernel does not otherwise
# depend on the value, so it keys on this per-index stamp.
NONOS_ROLLBACK_STAMP := $(TARGET_DIR)/.rollback-index.$(NONOS_ROLLBACK_INDEX)

# Developer evaluation bootstrap (NOT production custody).
#
# A clean checkout has no enrolled device identity: device_secrets.txt and
# device_root.bin are gitignored on purpose, because attestation proves
# knowledge of that secret and shipping it would let anyone forge proofs.
# Set NONOS_DEV=1 (or use the nonos-mk-dev-* targets) to mint a clearly
# marked throwaway identity from a fixed public seed so the image boots out
# of the box. Production deployers leave NONOS_DEV unset and pass their own
# enrolled secrets; with it unset none of the assignments below apply and the
# build stays fail-closed.
ifeq ($(NONOS_DEV),1)
ifeq ($(strip $(ZK_BOOT_ENROLL_SEED)),)
ZK_BOOT_ENROLL_SEED := nonos-public-dev-enroll-seed-not-for-production
endif
ifeq ($(strip $(ZK_CAPSULE_ENROLL_SEED)),)
ZK_CAPSULE_ENROLL_SEED := nonos-public-dev-capsule-enroll-not-for-production
endif
ifeq ($(strip $(ZK_CAPSULE_NONCE_SEED)),)
ZK_CAPSULE_NONCE_SEED := nonos-public-dev-capsule-nonce-not-for-production
endif
ifeq ($(strip $(ZK_BOOT_NONCE_SEED)),)
ZK_BOOT_NONCE_SEED := deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
endif
ifeq ($(strip $(ZK_BOOT_INDEX)),)
ZK_BOOT_INDEX := 0
endif
# Read the boot secret back from the enrolled identity file so it always
# matches the generated root, whatever seed produced it. Lazy (=) so it
# resolves after the enroll rule has written the file.
ifeq ($(strip $(ZK_BOOT_SECRET_X)),)
ZK_BOOT_SECRET_X = $(shell awk 'NR==1{print $$2}' $(ZK_BOOT_SECRETS) 2>/dev/null)
endif
ifeq ($(strip $(ZK_BOOT_SECRET_R)),)
ZK_BOOT_SECRET_R = $(shell awk 'NR==1{print $$3}' $(ZK_BOOT_SECRETS) 2>/dev/null)
endif
endif

# Header and status line, shown once per invocation. Fields are read from
# the working tree at parse time.
NONOS_BRANCH    := $(shell git rev-parse --abbrev-ref HEAD 2>/dev/null || echo detached)
NONOS_COMMIT    := $(shell git rev-parse --short HEAD 2>/dev/null || echo none)
NONOS_DIRTY     := $(shell test -n "$$(git status --porcelain 2>/dev/null)" && echo '*' || echo '')
NONOS_ATTESTED  := $(shell ls nonos-data/trust/capsules/*.zk_trailer.bin 2>/dev/null | wc -l | tr -d ' ')
NONOS_SIGNED    := $(shell ls nonos-data/trust/capsules/*.manifest.bin 2>/dev/null | wc -l | tr -d ' ')
NONOS_ZK_ROOT_FPR := $(shell test -f $(ZK_BOOT_ROOT) && shasum -a 256 $(ZK_BOOT_ROOT) 2>/dev/null | cut -c1-16 || echo unenrolled)
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
QEMU_NET_MODE ?= nat
QEMU_NET_CAPTURE ?=
QEMU_SERIAL_LOG ?= $(TARGET_DIR)/qemu-serial.log
QEMU_BLK_IMG := $(TARGET_DIR)/qemu-virtio-blk.img
QEMU_OVMF_VARS_RW := $(TARGET_DIR)/qemu-OVMF_VARS.fd
QEMU_BLK := -drive "file=$(QEMU_BLK_IMG),if=none,id=vd0,format=raw" -device virtio-blk-pci,drive=vd0
QEMU_XRES ?= 1920
QEMU_YRES ?= 1080
# QEMU_GL=1 swaps the display device for virtio-vga-gl (modern transport,
# virglrenderer backend) so the guest can negotiate the 3D command set; the
# cocoa display then needs a GL context. Default stays the plain 2D device.
ifeq ($(QEMU_GL),1)
QEMU_GPU := -device virtio-vga-gl,xres=$(QEMU_XRES),yres=$(QEMU_YRES)
QEMU_DISPLAY := cocoa,gl=es,zoom-to-fit=on
else
QEMU_GPU := -device virtio-vga,disable-modern=on,vectors=0,xres=$(QEMU_XRES),yres=$(QEMU_YRES)
QEMU_DISPLAY := cocoa,zoom-to-fit=on
endif
# Keyboard/mouse via the q35 i8042 (PS/2). USB HID interrupt-IN transfers
# are not serviced under macOS hvf, so usb-kbd/usb-mouse never deliver input
# there; the xHCI controller stays for the USB stack/storage paths.
QEMU_USB := -device qemu-xhci,id=xhci
QEMU_RNG := -device virtio-rng-pci

# Software TPM 2.0 for measured boot. The guest reaches it by direct MMIO at
# 0xFED40000 (tpm-crb), so no TCG2 firmware is needed; swtpm backs it over a
# unix socket.
SWTPM ?= swtpm
SWTPM_STATE ?= /tmp/nonos-swtpm
SWTPM_SOCK ?= $(SWTPM_STATE)/swtpm-sock
QEMU_TPM := -chardev socket,id=chrtpm,path=$(SWTPM_SOCK) -tpmdev emulator,id=tpm0,chardev=chrtpm -device tpm-crb,tpmdev=tpm0

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
	@echo "  make nonos-mk-run           full OS under QEMU + OVMF + TPM + NAT"
	@echo "  make nonos-mk-dev-run       clean-clone: mint a dev identity then boot"
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

$(KERNEL_MLDSA65_KEY): $(CAPSULE_SIGN_BIN)
	@echo "Generating kernel signing key (ML-DSA-65)..."
	@mkdir -p $(KEYS_DIR)
	@$(CAPSULE_SIGN_BIN) keygen --alg mldsa65 --out $(KERNEL_MLDSA65_PREFIX)

$(KERNEL_MLDSA65_PUB): $(KERNEL_MLDSA65_KEY)
	@test -f $@

nonos-mk-ensure-signing-key: $(SIGNING_KEY) $(KERNEL_MLDSA65_KEY) $(KERNEL_MLDSA65_PUB)

# ZK attestation: transparent enrolled-secret tools

$(ZK_ENROLL_TOOL) $(ZK_TRANSPARENT_PROVE_TOOL) $(ZK_TRANSPARENT_VERIFY_TOOL) $(ZK_CAPSULE_PROOF_TOOL): nonos-mk-check-deps
	@echo "Building transparent ZK attestation tools..."
	@cd $(ZK_CIRCUIT_DIR) && RUSTFLAGS="" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --bin transparent-enroll --bin transparent-prove --bin transparent-verify --bin capsule-attest-proof --target $(HOST_TARGET)

nonos-mk-zk-tools: $(ZK_ENROLL_TOOL) $(ZK_TRANSPARENT_PROVE_TOOL) $(ZK_TRANSPARENT_VERIFY_TOOL) $(ZK_CAPSULE_PROOF_TOOL)

# Transparent STARK enrollment tool, the production capsule attestation prover.
$(NONOS_STARK_ENROLL): nonos-mk-check-deps
	@echo "Building transparent STARK enrollment tool..."
	@cd nonos-stark-enroll && RUSTFLAGS="" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --target $(HOST_TARGET)

nonos-mk-ensure-zk-keys: $(ZK_BOOT_ROOT) $(ZK_BOOT_COMMITMENTS)

nonos-mk-verify-capsule-attest: nonos-mk-all-capsules-attested
	@printf "\n  transparent capsule attestation is kernel-verified at spawn\n\n"

nonos-mk-zk-report: $(ZK_BOOT_ROOT) $(ZK_BOOT_COMMITMENTS)
	@printf "\n  transparent enrolled-secret root   %s\n\n" "$(NONOS_ZK_ROOT_FPR)"

nonos-mk-zk-verify-live: $(ZK_BOOT_ROOT) $(ZK_BOOT_COMMITMENTS)
	@printf "\n  transparent ZK verifier selected   root %s\n\n" "$(NONOS_ZK_ROOT_FPR)"

nonos-mk-live-production-proof: nonos-mk-zk-verify-live nonos-mk-zk-report
	@echo "Transparent production proof path ready."

nonos-mk-attestation: $(TARGET_DIR)/kernel_attested.bin

NONOS_RECEIPT ?= target/attestation-receipt.txt
nonos-mk-attestation-receipt: $(TARGET_DIR)/kernel_attested.bin
	@mkdir -p $(dir $(NONOS_RECEIPT))
	@printf "# NONOS transparent attestation\nroot %s\nkernel %s\n" \
		"$(NONOS_ZK_ROOT_FPR)" "$(TARGET_DIR)/kernel_attested.bin" > $(NONOS_RECEIPT)
	@cat $(NONOS_RECEIPT)

$(EMBED_TOOL): nonos-mk-check-deps
	@echo "Building ZK embed tool..."
	@cd $(BOOTLOADER_DIR)/tools/embed-zk-proof && RUSTFLAGS="" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --target $(HOST_TARGET)

$(SIGN_TOOL): nonos-mk-check-deps
	@echo "Building kernel signing tool..."
	@cd $(BOOTLOADER_DIR)/tools/sign-kernel && RUSTFLAGS="" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --target $(HOST_TARGET)

$(ZK_BOOT_LABELS):
	@test "$(NONOS_DEV)" = 1 || { echo "$@ is required"; exit 1; }
	@mkdir -p $(dir $@)
	@printf "nonos-dev-device\n" > $@

$(ZK_BOOT_ROOT) $(ZK_BOOT_COMMITMENTS) $(ZK_BOOT_SECRETS): $(ZK_BOOT_LABELS) $(ZK_ENROLL_TOOL)
	@echo "Enrolling transparent boot attestation identities..."
	@test -n "$(ZK_BOOT_ENROLL_SEED)" || { echo "ZK_BOOT_ENROLL_SEED is required"; exit 1; }
	@mkdir -p $(dir $(ZK_BOOT_ROOT))
	@$(ZK_ENROLL_TOOL) \
		--seed "$(ZK_BOOT_ENROLL_SEED)" \
		--labels $(ZK_BOOT_LABELS) \
		--root-out $(ZK_BOOT_ROOT) \
		--secrets-out $(ZK_BOOT_SECRETS) \
		--commitments-out $(ZK_BOOT_COMMITMENTS)

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

# Bootloader security policy, selected at compile time. The loader refuses to
# build without an explicit policy (no silent default), so this is never
# ambiguous. `standard-qemu` boots under emulation while still enforcing the
# kernel signature and ZK attestation; a real production image MUST be built
# with BOOTLOADER_POLICY=production (Hardened: Secure Boot + TPM measured boot
# are mandatory and a missing one halts the boot).
BOOTLOADER_POLICY ?= standard-qemu

# When NONOS_TRUST_ANCHOR_PUBKEY points at a 32-byte public key, the bootloader
# bakes the trust anchor from it directly and the private signing seed is never
# read at build time. Production build/CI machines set this and hold no secret;
# only the offline signer's machine holds the seed for the sign step.
NONOS_TRUST_ANCHOR_PUBKEY ?=

# The GOP preference is baked into the binary via option_env, so a change to
# it must retrigger the cargo build; the stamp file's name carries the value.
GOP_PREF_STAMP := $(TARGET_DIR)/.gop-pref-$(if $(NONOS_GOP_PREF),$(NONOS_GOP_PREF),none).stamp
$(GOP_PREF_STAMP):
	@mkdir -p $(TARGET_DIR)
	@rm -f $(TARGET_DIR)/.gop-pref-*.stamp
	@touch $@

$(BOOTLOADER_DIR)/target/x86_64-unknown-uefi/release/nonos_boot.efi: \
		$(BOOTLOADER_SRCS) \
		$(if $(NONOS_TRUST_ANCHOR_PUBKEY),$(NONOS_TRUST_ANCHOR_PUBKEY),$(SIGNING_KEY)) \
		$(KERNEL_MLDSA65_PUB) \
		$(ZK_BOOT_ROOT) \
		$(if $(NONOS_STARK_KERNEL_ATTEST),$(KERNEL_ATTEST_ROOT_BIN)) \
		$(GOP_PREF_STAMP) \
		$(TARGET_DIR)/.nonos-toolchain.stamp
	@echo "Building UEFI bootloader (policy: $(BOOTLOADER_POLICY))..."
	$(eval SIGNING_KEY_ABS := $(if $(filter /%,$(SIGNING_KEY)),$(SIGNING_KEY),$(shell pwd)/$(SIGNING_KEY)))
	@cd $(BOOTLOADER_DIR) && \
		$(if $(NONOS_TRUST_ANCHOR_PUBKEY),NONOS_TRUST_ANCHOR_PUBKEY=$(abspath $(NONOS_TRUST_ANCHOR_PUBKEY)),NONOS_SIGNING_KEY=$(SIGNING_KEY_ABS)) \
		NONOS_MLDSA65_PUBKEY=$(shell pwd)/$(KERNEL_MLDSA65_PUB) \
		NONOS_ZK_DEVICE_ROOT=$(shell pwd)/$(ZK_BOOT_ROOT) \
		$(if $(NONOS_STARK_KERNEL_ATTEST),NONOS_KERNEL_ATTEST_ROOT=$(shell pwd)/$(KERNEL_ATTEST_ROOT_BIN)) \
		$(if $(NONOS_GOP_PREF),NONOS_GOP_PREF=$(NONOS_GOP_PREF)) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --target x86_64-unknown-uefi --release \
			--features zk-transparent,$(BOOTLOADER_POLICY)$(BOOT_STARK_FEATURE)

nonos-mk-bootloader: $(BOOTLOADER_DIR)/target/x86_64-unknown-uefi/release/nonos_boot.efi

# Builds the bootloader twice from identical inputs and proves the two outputs
# are byte-identical. Reproducibility is what lets anyone rebuild the loader
# from public source and confirm a published hash, so a single signing key
# cannot ship a hidden change. SOURCE_DATE_EPOCH is pinned by this Makefile.
nonos-mk-verify-reproducible-boot:
	@echo "Reproducible bootloader build check (policy: $(BOOTLOADER_POLICY))..."
	@touch $(BOOTLOADER_DIR)/build.rs
	@$(MAKE) --no-print-directory nonos-mk-bootloader
	@cp $(NONOS_BOOT_EFI) $(TARGET_DIR)/nonos_boot.repro-a.efi
	@touch $(BOOTLOADER_DIR)/build.rs
	@$(MAKE) --no-print-directory nonos-mk-bootloader
	@if cmp -s $(TARGET_DIR)/nonos_boot.repro-a.efi $(NONOS_BOOT_EFI); then \
		echo "  REPRODUCIBLE: byte-identical  sha256=$$(shasum -a256 $(NONOS_BOOT_EFI) | cut -c1-32)"; \
	else \
		echo "  gap: bootloader build is NOT reproducible"; exit 1; \
	fi

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

# Order-only dependency on the toolchain stamp: -Zbuild-std needs the
# rust-src component, and a make-level RUSTUP_TOOLCHAIN override skips the
# component list in rust-toolchain.toml, so the stamp must install it
# before the first userland build a fresh checkout ever runs.
$(USERLAND_LIBC): $(USERLAND_LIBC_SRCS) | $(TARGET_DIR)/.nonos-toolchain.stamp
	@echo "Building userland libc..."
	@cd $(USERLAND_DIR)/libc && \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) rustc --release --target ../x86_64-nonos-user.json \
		-Zbuild-std=core --crate-type staticlib
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
		$(toolkit_BIN) $(about_BIN) $(calculator_BIN) $(clock_BIN) $(snake_BIN) $(terminal_BIN) \
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
include userland/capsule_image_viewer/Capsule.mk
include userland/capsule_clipboard/Capsule.mk
include userland/capsule_login/Capsule.mk
include userland/toolkit/Capsule.mk
include userland/capsule_about/Capsule.mk
include userland/capsule_hello/Capsule.mk
include userland/capsule_boot_splash/Capsule.mk
include userland/capsule_calculator/Capsule.mk
include userland/capsule_clock/Capsule.mk
include userland/capsule_browser/Capsule.mk
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
include userland/capsule_driver_rtl8821ce/Capsule.mk
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
include userland/capsule_net_core/Capsule.mk
include userland/capsule_net_ip/Capsule.mk
include userland/capsule_net_udp/Capsule.mk
include userland/capsule_net_dhcp/Capsule.mk
include userland/capsule_net_tcp/Capsule.mk
include userland/capsule_net_dns/Capsule.mk
include userland/capsule_net_ntp/Capsule.mk
include userland/capsule_net_sockets/Capsule.mk
include userland/capsule_net_nym/Capsule.mk
include userland/capsule_wallpaper/Capsule.mk
include userland/capsule_attest/Capsule.mk
include userland/capsule_power/Capsule.mk

# Orchestration helper: union of every verified capsule's artifact
# triple. Smoke and test targets that need proof_io plus another
# capsule depend on `$(proof-io_ARTIFACTS)` directly.
NONOS_VERIFIED_ARTIFACTS = $(foreach slug,$(NONOS_VERIFIED_CAPSULES),$($(slug)_ARTIFACTS))
NONOS_VERIFIED_CAPSULE_MKS = $(foreach slug,$(NONOS_VERIFIED_CAPSULES),$($(slug)_CAPSULE_MK))

$(ZK_CAPSULE_LABELS): $(NONOS_VERIFIED_CAPSULE_MKS) Makefile
	@mkdir -p $(dir $@)
	@for label in $(foreach slug,$(NONOS_VERIFIED_CAPSULES),$($(slug)_BIN_NAME)); do \
		printf "%s\n" "$$label"; \
	done > $@

# Capsule attestation policy, transparent post-quantum STARK. The enrollment
# produces the policy root over the actual capsule measurements and every
# capsule's NZKSTRK1 trailer together, each re-checked against the exact
# spawn-gate parse before it is written. The nonos-mk/capsule.mk companion
# depends each trailer on this rule, so building any capsule's artifacts
# triggers the single enrollment. This replaces the curve enrolled-secret
# pipeline, which was not post-quantum.
$(ZK_CAPSULE_ROOT): $(NONOS_STARK_ENROLL) \
		$(foreach s,$(NONOS_VERIFIED_CAPSULES),$($(s)_BIN) $($(s)_MANIFEST))
	@echo "Enrolling $(words $(NONOS_VERIFIED_CAPSULES)) capsules under one transparent STARK policy root..."
	@mkdir -p $(dir $(ZK_CAPSULE_ROOT)) $(NONOS_BAKED_TRUST_DIR)/capsules
	@$(NONOS_STARK_ENROLL) capsules $(ZK_CAPSULE_ROOT) \
		$(foreach s,$(NONOS_VERIFIED_CAPSULES),$($(s)_REQUIRED_CAPS):$($(s)_BIN):$($(s)_ATTESTATION))

.PHONY: nonos-mk-stark-enroll-capsules
nonos-mk-stark-enroll-capsules: $(ZK_CAPSULE_ROOT)
	@echo "Enrolled the capsule set under the STARK policy root $(ZK_CAPSULE_ROOT)"

nonos-mk-all-capsules-attested: $(NONOS_VERIFIED_ARTIFACTS)
	@echo "Signed and attested $(words $(NONOS_VERIFIED_CAPSULES)) included capsules."

# Enroll the kernel's own measurement and emit its self-attestation root and
# trailer. The bootloader embeds the root (NONOS_KERNEL_ATTEST_ROOT) and verifies
# the trailer, carried in the kernel image footer, before jumping. The tool
# re-checks the trailer against the same verifier the bootloader runs.
$(KERNEL_ATTEST_ROOT_BIN) $(KERNEL_ATTEST_TRAILER): $(KERNEL_ATTEST_ELF) $(NONOS_STARK_ENROLL)
	@echo "Enrolling the kernel self-attestation..."
	@mkdir -p $(dir $(KERNEL_ATTEST_ROOT_BIN)) $(dir $(KERNEL_ATTEST_TRAILER))
	@$(NONOS_STARK_ENROLL) kernel $(KERNEL_ATTEST_ELF) \
		$(KERNEL_ATTEST_ROOT_BIN) $(KERNEL_ATTEST_TRAILER)

nonos-mk-kernel-attest: $(KERNEL_ATTEST_ROOT_BIN) $(KERNEL_ATTEST_TRAILER)
	@echo "Kernel self-attestation enrolled: root $(KERNEL_ATTEST_ROOT_BIN)"

NONOS_DESKTOP_GUI_CAPSULE_CHECKS = \
	$(proof-io_VERIFY) $(std-proof_VERIFY) $(ripgrep_VERIFY) \
	$(ramfs_VERIFY) $(keyring_VERIFY) \
	$(entropy_VERIFY) $(crypto_VERIFY) $(vfs_VERIFY) \
	$(driver-virtio-rng_VERIFY) $(driver-virtio-blk_VERIFY) \
	$(driver-virtio-gpu_VERIFY) $(driver-virtio-net_VERIFY) \
	$(driver-ps2-input_VERIFY) $(driver-xhci_VERIFY) \
	$(driver-usb-hid_VERIFY) \
	$(net-core_VERIFY) $(net-sockets_VERIFY) $(net-nym_VERIFY) \
	$(policy_VERIFY) $(wallpaper_catalog_VERIFY) \
	$(installer_VERIFY) \
	$(input-router_VERIFY) $(compositor_VERIFY) $(wm_VERIFY) \
	$(desktop-shell_VERIFY) $(image-codec_VERIFY) $(image-viewer_VERIFY) $(clipboard_VERIFY) \
	$(login_VERIFY) $(wallpaper_VERIFY) $(toolkit_VERIFY) \
	$(boot-splash_VERIFY) $(about_VERIFY) $(calculator_VERIFY) $(clock_VERIFY) \
	$(browser_VERIFY) $(web_VERIFY) \
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

# nonos_kernel_build: compile the kernel with one cargo feature set. Every
# profile target is one call to this, so the build command lives in exactly one
# place; only the feature string differs. $(1) is the human label for the log,
# $(2) is the comma-separated feature list passed to cargo.
define nonos_kernel_build
	@echo "Building kernel ($(1))..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features $(2)
endef

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
	$(call nonos_kernel_build,microkernel-core,microkernel-core)

# Core kernel with the transparent STARK attestation backend selected. The
# spawn gate verifies a money-grade membership proof instead of the pairing
# check, and the kernel self-attestation module is compiled in. Enabling this
# for a capsule-bearing build additionally requires every capsule to ship a
# STARK membership trailer enrolled under the policy root; until the signing
# pipeline emits those, keep it on the core lane.
nonos-mk-core-attested: nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-core + nonos-stark-attest,microkernel-core$(_boot_comma)nonos-stark-attest)

# -----------------------------------------------------------------------------
# Configurable build: choose a profile with the menuconfig-style tool, then
# build exactly that. This is the front door for a user assembling their own
# kernel; the fixed -prod targets above are the curated, release-tested ones.
# -----------------------------------------------------------------------------

# The resolved feature set from .nonos-config: the chosen profile, plus the
# attestation backend when the config asked for it.
FROM_CONFIG_FEATURES = $(NONOS_PROFILE)$(if $(filter 1,$(NONOS_ATTEST)),$(_boot_comma)nonos-stark-attest)

# Open the interactive configurator and write .nonos-config.
nonos-mk-menuconfig:
	@./tools/nonos-config

# Build the kernel described by .nonos-config. Fails clearly if no config has
# been written yet, rather than silently building the default.
nonos-mk-from-config: nonos-mk-check-deps nonos-mk-ensure-signing-key
	@test -f .nonos-config || { echo "no .nonos-config; run 'make menuconfig' first"; exit 1; }
	@echo "Building from .nonos-config (rollback index $(NONOS_ROLLBACK_INDEX))..."
	$(call nonos_kernel_build,$(NONOS_PROFILE) from .nonos-config,$(FROM_CONFIG_FEATURES))

# Short aliases, so a user types what the help prints.
.PHONY: menuconfig from-config
menuconfig: nonos-mk-menuconfig
from-config: nonos-mk-from-config

nonos-mk-capsules: $(proof-io_ARTIFACTS) $(ramfs_BIN) $(keyring_BIN) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-capsules,microkernel-capsules)

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
NONOS_INPUT_PROBE_INJECT_ESP := $(TARGET_DIR)/esp-input-probe-inject

# Per-capsule production kernel builds. Each `-prod` target builds
# the kernel under the `nonos-production` posture with proof_io and
# the named capsule baked in. Dependencies require the capsule's
# signed bundle (cert + manifest + ELF) so the kernel's
# `include_bytes!` resolves a verified payload. The macro-emitted
# `nonos-mk-<slug>` target stays as the userland-ELF builder; the
# `-prod` suffix is the only kernel build path under production
# posture.

nonos-mk-proof-io-prod: $(proof-io_ARTIFACTS) nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-proof-io,microkernel-proof-io)

nonos-mk-std-proof-prod: $(proof-io_ARTIFACTS) $(std-proof_ARTIFACTS) nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-proof-io + nonos-capsule-std-proof,microkernel-proof-io$(_boot_comma)nonos-capsule-std-proof)

nonos-mk-ramfs-prod: $(proof-io_ARTIFACTS) $(ramfs_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-ramfs,microkernel-ramfs)

nonos-mk-keyring-prod: $(proof-io_ARTIFACTS) $(keyring_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-keyring,microkernel-keyring)

nonos-mk-entropy-prod: $(proof-io_ARTIFACTS) $(entropy_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-entropy,microkernel-entropy)

nonos-mk-crypto-prod: $(proof-io_ARTIFACTS) $(crypto_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-crypto,microkernel-crypto)

nonos-mk-vfs-prod: $(proof-io_ARTIFACTS) $(vfs_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-vfs,microkernel-vfs)

nonos-mk-market-prod: $(proof-io_ARTIFACTS) $(market_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-market,microkernel-market)

nonos-mk-driver-virtio-rng-prod: $(proof-io_ARTIFACTS) $(driver-virtio-rng_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-virtio-rng,microkernel-driver-virtio-rng)

nonos-mk-driver-virtio-blk-prod: $(proof-io_ARTIFACTS) $(driver-virtio-blk_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-virtio-blk,microkernel-driver-virtio-blk)

nonos-mk-driver-virtio-gpu-prod: $(proof-io_ARTIFACTS) $(driver-virtio-gpu_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-virtio-gpu,microkernel-driver-virtio-gpu)

nonos-mk-driver-virtio-net-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-virtio-net,microkernel-driver-virtio-net)

nonos-mk-driver-iwlwifi-prod: $(proof-io_ARTIFACTS) $(driver-iwlwifi_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-iwlwifi,microkernel-driver-iwlwifi)

nonos-mk-driver-rtl8821ce-prod: $(proof-io_ARTIFACTS) $(driver-rtl8821ce_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-rtl8821ce,microkernel-driver-rtl8821ce)

nonos-mk-driver-i2c-pci-prod: $(proof-io_ARTIFACTS) $(driver-i2c-pci_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-i2c-pci,microkernel-driver-i2c-pci)

nonos-mk-driver-i2c-hid-prod: $(proof-io_ARTIFACTS) $(driver-i2c-pci_ARTIFACTS) $(driver-i2c-hid_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-i2c-hid,microkernel-driver-i2c-hid)

nonos-mk-driver-ps2-input-prod: $(proof-io_ARTIFACTS) $(driver-ps2-input_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-ps2-input,microkernel-driver-ps2-input)

nonos-mk-driver-xhci-prod: $(proof-io_ARTIFACTS) $(driver-xhci_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-xhci,microkernel-driver-xhci)

nonos-mk-driver-usb-hid-prod: $(proof-io_ARTIFACTS) $(driver-xhci_ARTIFACTS) \
		$(driver-usb-hid_ARTIFACTS) nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-usb-hid,microkernel-driver-usb-hid)

nonos-mk-driver-usb-msc-prod: $(proof-io_ARTIFACTS) $(driver-xhci_ARTIFACTS) \
		$(driver-usb-msc_ARTIFACTS) nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-usb-msc,microkernel-driver-usb-msc)

nonos-mk-driver-e1000-prod: $(proof-io_ARTIFACTS) $(driver-e1000_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-e1000,microkernel-driver-e1000)

nonos-mk-driver-rtl8139-prod: $(proof-io_ARTIFACTS) $(driver-rtl8139_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-rtl8139,microkernel-driver-rtl8139)

nonos-mk-driver-rtl8169-prod: $(proof-io_ARTIFACTS) $(driver-rtl8169_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-rtl8169,microkernel-driver-rtl8169)

nonos-mk-driver-ahci-prod: $(proof-io_ARTIFACTS) $(driver-ahci_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-ahci,microkernel-driver-ahci)

nonos-mk-driver-hda-prod: $(proof-io_ARTIFACTS) $(driver-hda_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-hda,microkernel-driver-hda)

nonos-mk-driver-nvme-prod: $(proof-io_ARTIFACTS) $(driver-nvme_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-driver-nvme,microkernel-driver-nvme)

# Net-service capsule kernel profiles. Each stacks on the lower
# layer so a microkernel-net-dhcp build pulls in net.l2 + net.ip +
# net.udp + net.dhcp and the underlying virtio-net driver.

nonos-mk-net-l2-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-l2_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-net-l2,microkernel-net-l2)

nonos-mk-net-core-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-core_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-net-core,microkernel-net-core)

nonos-mk-net-ip-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-l2_ARTIFACTS) $(net-ip_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-net-ip,microkernel-net-ip)

nonos-mk-net-udp-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-l2_ARTIFACTS) $(net-ip_ARTIFACTS) $(net-udp_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-net-udp,microkernel-net-udp)

nonos-mk-net-dhcp-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-l2_ARTIFACTS) $(net-ip_ARTIFACTS) $(net-udp_ARTIFACTS) \
		$(net-dhcp_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-net-dhcp,microkernel-net-dhcp)

nonos-mk-ntp-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-l2_ARTIFACTS) $(net-ip_ARTIFACTS) $(net-udp_ARTIFACTS) \
		$(net-dhcp_ARTIFACTS) $(net-ntp_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-net-ntp,microkernel-net-ntp)

nonos-mk-ntp-sign: $(net-ntp_ARTIFACTS)

nonos-mk-ntp-test: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-l2_ARTIFACTS) $(net-ip_ARTIFACTS) $(net-udp_ARTIFACTS) \
		$(net-dhcp_ARTIFACTS) $(net-ntp_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (nonos-ntp-smoketest)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build $(KERNEL_BUILD_FLAGS) \
		--no-default-features --features nonos-ntp-smoketest

nonos-mk-image-viewer-test: $(proof-io_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building image_viewer capsule (nonos-image-viewer-smoketest)..."
	@$(MAKE) -B image-viewer_CARGO_FEATURES=nonos-image-viewer-smoketest nonos-mk-image-viewer-sign
	$(call nonos_kernel_build,microkernel-image-viewer-smoketest,microkernel-image-viewer-smoketest)

nonos-mk-net-nym-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-core_ARTIFACTS) $(net-nym_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-net-nym,microkernel-net-nym)

nonos-mk-net-sockets-prod: $(proof-io_ARTIFACTS) $(driver-virtio-net_ARTIFACTS) \
		$(net-core_ARTIFACTS) \
		$(net-sockets_ARTIFACTS) $(net-nym_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-net-sockets,microkernel-net-sockets)

nonos-mk-desktop-gui-prod: $(proof-io_ARTIFACTS) \
		$(std-proof_ARTIFACTS) $(ripgrep_ARTIFACTS) $(ramfs_ARTIFACTS) \
		$(keyring_ARTIFACTS) $(entropy_ARTIFACTS) $(crypto_ARTIFACTS) \
		$(vfs_ARTIFACTS) $(driver-virtio-rng_ARTIFACTS) \
		$(driver-virtio-blk_ARTIFACTS) $(driver-virtio-gpu_ARTIFACTS) \
		$(driver-virtio-net_ARTIFACTS) $(driver-ps2-input_ARTIFACTS) \
		$(driver-xhci_ARTIFACTS) $(driver-usb-hid_ARTIFACTS) \
		$(net-core_ARTIFACTS) \
		$(net-sockets_ARTIFACTS) $(net-nym_ARTIFACTS) \
		$(policy_ARTIFACTS) $(wallpaper_catalog_ARTIFACTS) \
		$(installer_ARTIFACTS) \
		$(input-router_ARTIFACTS) $(compositor_ARTIFACTS) \
		$(wm_ARTIFACTS) $(desktop-shell_ARTIFACTS) \
		$(image-codec_ARTIFACTS) $(image-viewer_ARTIFACTS) $(clipboard_ARTIFACTS) \
		$(login_ARTIFACTS) $(wallpaper_ARTIFACTS) \
		$(toolkit_ARTIFACTS) $(about_ARTIFACTS) $(boot-splash_ARTIFACTS) \
		$(calculator_ARTIFACTS) $(clock_ARTIFACTS) $(browser_ARTIFACTS) \
		$(snake_ARTIFACTS) \
		$(wallet-nonos_ARTIFACTS) $(terminal_ARTIFACTS) \
		$(file-manager_ARTIFACTS) $(text-editor_ARTIFACTS) \
		$(settings_ARTIFACTS) $(process-manager_ARTIFACTS) \
		$(attest_ARTIFACTS) $(power_ARTIFACTS) \
		$(ZK_POLICY_ROOT) \
		nonos-mk-verify-desktop-gui-capsules \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-desktop-gui + nonos-stark-attest,microkernel-desktop-gui$(_boot_comma)nonos-stark-attest)

# nonos-mk-zerostate: the canonical NONOS image. The whole ZeroState system in
# one build: every capsule and driver, the transparent STARK spawn gate
# enforced, dual Ed25519 + ML-DSA-65 signing, the anti-rollback index bound into
# the signature, and the TPM measured-boot path. This is the reference target;
# the narrower -prod cuts share its recipe with a smaller feature set.
nonos-mk-zerostate: nonos-mk-all-capsules-attested \
		$(driver-iwlwifi_ARTIFACTS) $(driver-rtl8821ce_ARTIFACTS) \
		nonos-mk-verify-desktop-gui-capsules \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,zerostate: microkernel-full-gui + nonos-stark-attest,microkernel-full-gui$(_boot_comma)nonos-stark-attest)

# Back-compat alias for the former name. Prefer nonos-mk-zerostate.
.PHONY: nonos-mk-full-gui-prod
nonos-mk-full-gui-prod: nonos-mk-zerostate

nonos-mk-setup-wizard-prod: $(proof-io_ARTIFACTS) $(ramfs_ARTIFACTS) \
		$(keyring_ARTIFACTS) $(entropy_ARTIFACTS) $(crypto_ARTIFACTS) \
		$(vfs_ARTIFACTS) $(driver-virtio-rng_ARTIFACTS) \
		$(driver-virtio-blk_ARTIFACTS) $(driver-virtio-gpu_ARTIFACTS) \
		$(driver-virtio-net_ARTIFACTS) $(driver-ps2-input_ARTIFACTS) \
		$(driver-xhci_ARTIFACTS) $(net-core_ARTIFACTS) \
		$(net-sockets_ARTIFACTS) $(net-nym_ARTIFACTS) \
		$(policy_ARTIFACTS) $(wallpaper_catalog_ARTIFACTS) \
		$(input-router_ARTIFACTS) $(compositor_ARTIFACTS) \
		$(wm_ARTIFACTS) $(desktop-shell_ARTIFACTS) \
		$(image-codec_ARTIFACTS) $(clipboard_ARTIFACTS) \
		$(login_ARTIFACTS) $(wallpaper_ARTIFACTS) \
		$(toolkit_ARTIFACTS) $(about_ARTIFACTS) $(boot-splash_ARTIFACTS) \
		$(calculator_ARTIFACTS) $(clock_ARTIFACTS) $(snake_ARTIFACTS) $(terminal_ARTIFACTS) \
		$(file-manager_ARTIFACTS) $(text-editor_ARTIFACTS) \
		$(settings_ARTIFACTS) $(process-manager_ARTIFACTS) \
		$(attest_ARTIFACTS) $(power_ARTIFACTS) \
		$(setup-wizard_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-setup-wizard,microkernel-setup-wizard)

nonos-mk-setup-wizard-inject-prod: $(proof-io_ARTIFACTS) $(ramfs_ARTIFACTS) \
		$(keyring_ARTIFACTS) $(entropy_ARTIFACTS) $(crypto_ARTIFACTS) \
		$(vfs_ARTIFACTS) $(driver-virtio-rng_ARTIFACTS) \
		$(driver-virtio-blk_ARTIFACTS) $(driver-virtio-gpu_ARTIFACTS) \
		$(driver-virtio-net_ARTIFACTS) $(driver-ps2-input_ARTIFACTS) \
		$(driver-xhci_ARTIFACTS) $(net-core_ARTIFACTS) \
		$(net-sockets_ARTIFACTS) $(net-nym_ARTIFACTS) \
		$(policy_ARTIFACTS) $(wallpaper_catalog_ARTIFACTS) \
		$(input-router_ARTIFACTS) $(compositor_ARTIFACTS) \
		$(wm_ARTIFACTS) $(desktop-shell_ARTIFACTS) \
		$(image-codec_ARTIFACTS) $(clipboard_ARTIFACTS) \
		$(login_ARTIFACTS) $(wallpaper_ARTIFACTS) \
		$(toolkit_ARTIFACTS) $(about_ARTIFACTS) $(boot-splash_ARTIFACTS) \
		$(calculator_ARTIFACTS) $(clock_ARTIFACTS) $(snake_ARTIFACTS) $(terminal_ARTIFACTS) \
		$(file-manager_ARTIFACTS) $(text-editor_ARTIFACTS) \
		$(settings_ARTIFACTS) $(process-manager_ARTIFACTS) \
		$(attest_ARTIFACTS) $(power_ARTIFACTS) \
		$(setup-wizard_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-setup-wizard + input-probe-inject,microkernel-setup-wizard$(_boot_comma)input-probe-inject)

nonos-mk-input-probe-inject-prod: $(proof-io_ARTIFACTS) \
		$(driver-ps2-input_ARTIFACTS) $(driver-virtio-gpu_ARTIFACTS) \
		$(input-router_ARTIFACTS) $(compositor_ARTIFACTS) \
		$(input-probe_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-input-probe + input-probe-inject,microkernel-input-probe$(_boot_comma)input-probe-inject)

nonos-mk-input-probe-inject-esp: $(NONOS_BOOT_EFI)
	@$(MAKE) --no-print-directory nonos-mk-input-probe-inject-prod
	@$(MAKE) --no-print-directory $(TARGET_DIR)/kernel_attested.bin
	@mkdir -p $(NONOS_INPUT_PROBE_INJECT_ESP)/EFI/Boot $(NONOS_INPUT_PROBE_INJECT_ESP)/EFI/nonos
	@cp $(NONOS_BOOT_EFI) $(NONOS_INPUT_PROBE_INJECT_ESP)/EFI/Boot/BOOTX64.EFI
	@cp $(TARGET_DIR)/kernel_attested.bin $(NONOS_INPUT_PROBE_INJECT_ESP)/EFI/nonos/kernel.bin
	@printf "timeout=0\ndefault=nonos\n" > $(NONOS_INPUT_PROBE_INJECT_ESP)/EFI/nonos/boot.cfg
	@echo 'fs0:\EFI\Boot\BOOTX64.EFI' > $(NONOS_INPUT_PROBE_INJECT_ESP)/startup.nsh

nonos-mk-toolkit-prod: nonos-mk-desktop-gui-prod
nonos-mk-about-prod: nonos-mk-desktop-gui-prod
nonos-mk-calculator-prod: nonos-mk-desktop-gui-prod
nonos-mk-clock-prod: nonos-mk-desktop-gui-prod
nonos-mk-browser-prod: nonos-mk-desktop-gui-prod
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
		$(driver-virtio-rng_ARTIFACTS) $(installer_ARTIFACTS) $(hello_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building terminal capsule (autorun-selftest)..."
	@$(MAKE) -B terminal_CARGO_FEATURES=nonos-autorun-selftest nonos-mk-terminal-sign
	@echo "Seeding vfs capsule store with the hello child..."
	@$(MAKE) -B vfs_CARGO_FEATURES=seed-terminal-store $(vfs_ARTIFACTS)
	$(call nonos_kernel_build,microkernel-terminal-smoketest,microkernel-terminal-smoketest)

nonos-mk-text-editor-prod: nonos-mk-desktop-gui-prod
nonos-mk-settings-prod: nonos-mk-desktop-gui-prod
nonos-mk-process-manager-prod: nonos-mk-desktop-gui-prod

# Sign + attest + ESP packaging

$(NONOS_ROLLBACK_STAMP):
	@mkdir -p $(TARGET_DIR)
	@rm -f $(TARGET_DIR)/.rollback-index.*
	@touch $@

$(TARGET_DIR)/kernel_signed.bin: $(TARGET_DIR)/x86_64-nonos/release/nonos-kernel \
		$(SIGNING_KEY) $(KERNEL_MLDSA65_KEY) $(KERNEL_MLDSA65_PUB) $(SIGN_TOOL) \
		$(NONOS_ROLLBACK_STAMP)
	@echo "Signing kernel (Ed25519 + ML-DSA-65, rollback index $(NONOS_ROLLBACK_INDEX))..."
	@mkdir -p $(TARGET_DIR)
	@$(SIGN_TOOL) --key $(KERNEL_SIGNING_KEY) --input $< --output $@ \
		--mldsa65-key $(KERNEL_MLDSA65_KEY) \
		--mldsa65-pub $(KERNEL_MLDSA65_PUB) \
		--rollback-index $(NONOS_ROLLBACK_INDEX) \
		--verify

nonos-mk-sign: $(TARGET_DIR)/kernel_signed.bin

$(TARGET_DIR)/kernel_attested.bin: $(TARGET_DIR)/kernel_signed.bin $(EMBED_TOOL) $(ZK_BOOT_ROOT) $(ZK_BOOT_COMMITMENTS)
	@echo "Embedding ZK attestation proof..."
	@test -n "$(ZK_BOOT_INDEX)" || { echo "ZK_BOOT_INDEX is required"; exit 1; }
	@test -n "$(ZK_BOOT_SECRET_X)" || { echo "ZK_BOOT_SECRET_X is required"; exit 1; }
	@test -n "$(ZK_BOOT_SECRET_R)" || { echo "ZK_BOOT_SECRET_R is required"; exit 1; }
	@test -n "$(ZK_BOOT_NONCE_SEED)" || { echo "ZK_BOOT_NONCE_SEED is required"; exit 1; }
	@$(EMBED_TOOL) --input $< --output $@ \
		--root $(ZK_BOOT_ROOT) \
		--commitments $(ZK_BOOT_COMMITMENTS) \
		--index $(ZK_BOOT_INDEX) \
		--secret-x "$(ZK_BOOT_SECRET_X)" \
		--secret-r "$(ZK_BOOT_SECRET_R)" \
		--nonce-seed "$(ZK_BOOT_NONCE_SEED)" \
		--verbose

nonos-mk-attest: $(TARGET_DIR)/kernel_attested.bin

$(ZK_BOOT_SIDECAR): $(TARGET_DIR)/kernel_signed.bin $(EMBED_TOOL) $(ZK_BOOT_ROOT) $(ZK_BOOT_COMMITMENTS)
	@echo "Writing runtime ZK boot sidecar..."
	@test -n "$(ZK_BOOT_INDEX)" || { echo "ZK_BOOT_INDEX is required"; exit 1; }
	@test -n "$(ZK_BOOT_SECRET_X)" || { echo "ZK_BOOT_SECRET_X is required"; exit 1; }
	@test -n "$(ZK_BOOT_SECRET_R)" || { echo "ZK_BOOT_SECRET_R is required"; exit 1; }
	@test -n "$(ZK_BOOT_NONCE_SEED)" || { echo "ZK_BOOT_NONCE_SEED is required"; exit 1; }
	@mkdir -p $(dir $(ZK_BOOT_SIDECAR))
	@if [ -n "$(ZK_BOOT_CHALLENGE)" ]; then \
		$(EMBED_TOOL) --input $< --output $@ --root $(ZK_BOOT_ROOT) \
			--commitments $(ZK_BOOT_COMMITMENTS) --index $(ZK_BOOT_INDEX) \
			--secret-x "$(ZK_BOOT_SECRET_X)" --secret-r "$(ZK_BOOT_SECRET_R)" \
			--challenge "$(ZK_BOOT_CHALLENGE)" \
			--nonce-seed "$(ZK_BOOT_NONCE_SEED)" --sidecar --verbose; \
	else \
		test -n "$(ZK_BOOT_NONCE)" || { echo "ZK_BOOT_NONCE is required"; exit 1; }; \
		test -n "$(ZK_BOOT_MACHINE_ID)" || { echo "ZK_BOOT_MACHINE_ID is required"; exit 1; }; \
		test -n "$(ZK_BOOT_TIMESTAMP)" || { echo "ZK_BOOT_TIMESTAMP is required"; exit 1; }; \
		$(EMBED_TOOL) --input $< --output $@ --root $(ZK_BOOT_ROOT) \
			--commitments $(ZK_BOOT_COMMITMENTS) --index $(ZK_BOOT_INDEX) \
			--secret-x "$(ZK_BOOT_SECRET_X)" --secret-r "$(ZK_BOOT_SECRET_R)" \
			--boot-nonce "$(ZK_BOOT_NONCE)" --machine-id "$(ZK_BOOT_MACHINE_ID)" \
			--timestamp "$(ZK_BOOT_TIMESTAMP)" \
			--nonce-seed "$(ZK_BOOT_NONCE_SEED)" --sidecar --verbose; \
	fi

nonos-mk-boot-zk-sidecar: $(ZK_BOOT_SIDECAR)

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

# Produce a real, flashable GPT disk image with a FAT32 EFI System Partition.
# Unlike the ESP directory (which only QEMU's virtual-FAT can boot), this
# exercises a genuine partition table and filesystem, so it boots off a USB on
# real hardware and off `-drive format=raw` in QEMU.
nonos-mk-usb-img: nonos-mk-esp
	@echo "Building GPT/FAT32 USB image $(USB_IMG) ($(USB_IMG_MB) MiB)..."
	@dd if=/dev/zero of=$(USB_IMG) bs=1048576 count=$(USB_IMG_MB) status=none 2>/dev/null \
		|| dd if=/dev/zero of=$(USB_IMG) bs=1048576 count=$(USB_IMG_MB) 2>/dev/null
	@sgdisk -Z $(USB_IMG) >/dev/null 2>&1 || true
	@sgdisk -o -n 1:2048:0 -t 1:EF00 -c 1:NONOS-ESP $(USB_IMG) >/dev/null
	@mformat -i $(USB_IMG)@@1M -F ::
	@mcopy -i $(USB_IMG)@@1M -s $(ESP_DIR)/EFI ::/EFI
	@mcopy -i $(USB_IMG)@@1M $(ESP_DIR)/startup.nsh ::/
	@echo "USB image ready at $(USB_IMG)"
	@echo "  Validate as a real disk:  make nonos-mk-usb-run"
	@echo "  Flash (macOS): sudo dd if=$(USB_IMG) of=/dev/rdiskN bs=4m && sync"
	@echo "  Flash (Linux): sudo dd if=$(USB_IMG) of=/dev/sdX  bs=4M oflag=direct && sync"

# Boot the image as a real GPT disk (NOT virtual FAT), so a pass here proves the
# partition table and ESP filesystem the USB actually boots from.
nonos-mk-usb-run: nonos-mk-usb-img $(QEMU_OVMF_VARS_RW)
	@echo "Booting $(USB_IMG) as a real GPT disk..."
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive format=raw,file=$(USB_IMG) \
		-drive if=pflash,format=raw,unit=0,readonly=on,file="$(OVMF)" \
		-drive if=pflash,format=raw,unit=1,file="$(QEMU_OVMF_VARS_RW)" \
		$(QEMU_GPU) $(QEMU_RNG) \
		-serial mon:stdio -vga none -display $(QEMU_DISPLAY) -no-reboot

# QEMU

$(QEMU_BLK_IMG):
	@mkdir -p $(dir $@)
	@truncate -s 64M $@

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

nonos-mk-run: nonos-mk-swtpm-start nonos-mk-live-production-proof nonos-mk-zerostate nonos-mk-esp $(QEMU_BLK_IMG) $(QEMU_OVMF_VARS_RW)
	@echo "Booting NONOS in QEMU..."
	@echo "  Network: $(QEMU_NET_DESC)"
	@echo "  TPM: swtpm CRB"
	@echo "  Quit: Ctrl+A then X"
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
		-drive if=pflash,format=raw,unit=0,readonly=on,file="$(OVMF)" \
		-drive if=pflash,format=raw,unit=1,file="$(QEMU_OVMF_VARS_RW)" \
		$(QEMU_BLK) $(QEMU_GPU) $(QEMU_NET) $(QEMU_USB) $(QEMU_RNG) $(QEMU_TPM) \
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

nonos-mk-run-serial-log: nonos-mk-desktop-gui-prod nonos-mk-esp $(QEMU_BLK_IMG)
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
