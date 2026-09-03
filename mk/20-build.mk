# The build pipeline. Userland capsules (via userland/*/Capsule.mk), the kernel
# itself, dual Ed25519 + ML-DSA-65 signing, and the transparent post-quantum
# STARK attestation for the kernel and every capsule. This is where make,
# make qemu, and make from-config all resolve their real work.

.PHONY: nonos-mk-check-driver-ahci-keys nonos-mk-check-driver-e1000-keys nonos-mk-check-driver-hda-keys nonos-mk-check-driver-i2c-hid-keys nonos-mk-check-driver-i2c-pci-keys nonos-mk-check-driver-iwlwifi-keys nonos-mk-check-driver-nvme-keys nonos-mk-check-driver-rtl8139-keys nonos-mk-check-driver-rtl8169-keys nonos-mk-check-driver-rtl8821ce-keys nonos-mk-check-driver-usb-msc-keys nonos-mk-check-driver-virtio-gpu-keys nonos-mk-check-ps2-input-keys nonos-mk-check-ramfs-keys nonos-mk-check-virtio-blk-keys nonos-mk-check-virtio-net-keys nonos-mk-check-virtio-rng-keys nonos-mk-check-xhci-keys nonos-mk-crypto nonos-mk-driver-ahci nonos-mk-driver-ahci-sign nonos-mk-driver-e1000 nonos-mk-driver-e1000-sign nonos-mk-driver-hda nonos-mk-driver-hda-sign nonos-mk-driver-i2c-hid nonos-mk-driver-i2c-hid-sign nonos-mk-driver-i2c-pci nonos-mk-driver-i2c-pci-sign nonos-mk-driver-iwlwifi nonos-mk-driver-iwlwifi-sign nonos-mk-driver-nvme nonos-mk-driver-nvme-sign nonos-mk-driver-rtl8139 nonos-mk-driver-rtl8139-sign nonos-mk-driver-rtl8169 nonos-mk-driver-rtl8169-sign nonos-mk-driver-rtl8821ce nonos-mk-driver-rtl8821ce-sign nonos-mk-driver-usb-msc nonos-mk-driver-usb-msc-sign nonos-mk-driver-virtio-gpu nonos-mk-driver-virtio-gpu-sign nonos-mk-entropy nonos-mk-keyring nonos-mk-market nonos-mk-proof-io nonos-mk-proof-io-sign nonos-mk-ps2-input nonos-mk-ps2-input-sign nonos-mk-ramfs nonos-mk-ramfs-sign nonos-mk-vfs nonos-mk-virtio-blk nonos-mk-virtio-blk-sign nonos-mk-virtio-net nonos-mk-virtio-net-sign nonos-mk-virtio-rng nonos-mk-virtio-rng-sign nonos-mk-wallpaper nonos-mk-xhci nonos-mk-xhci-sign nonos-mk-all-capsules-attested nonos-mk-attest nonos-mk-attestation nonos-mk-attestation-receipt nonos-mk-bootloader nonos-mk-capsules nonos-mk-check nonos-mk-check-trust-keys nonos-mk-check-trust-manifest nonos-mk-core nonos-mk-core-attested nonos-mk-desktop-gui-prod nonos-mk-ensure-zk-keys nonos-mk-esp nonos-mk-from-config nonos-mk-host-trust-verify nonos-mk-libc nonos-mk-live-production-proof nonos-mk-marketplace-abi nonos-mk-marketplace-index-tool nonos-mk-menuconfig nonos-mk-sign nonos-mk-terminal-test nonos-mk-trust-policy nonos-mk-usb-img nonos-mk-userland-clean nonos-mk-verify-capsule-attest nonos-mk-verify-trust nonos-mk-zerostate nonos-mk-zk-report nonos-mk-zk-tools nonos-mk-zk-verify-live

# ZK attestation: transparent enrolled-secret tools

# Every host tool tracks its sources. A restored cache hands back old
# binaries, and a rule with no file prerequisites would treat them as
# final; the sources keep cargo the authority on staleness.
ZK_TOOL_SRCS := $(shell find $(ZK_CIRCUIT_DIR)/src -type f -name '*.rs' 2>/dev/null) \
                $(ZK_CIRCUIT_DIR)/Cargo.toml
$(ZK_ENROLL_TOOL) $(ZK_TRANSPARENT_PROVE_TOOL) $(ZK_TRANSPARENT_VERIFY_TOOL) $(ZK_CAPSULE_PROOF_TOOL): nonos-mk-check-deps $(ZK_TOOL_SRCS)
	@echo "Building transparent ZK attestation tools..."
	@cd $(ZK_CIRCUIT_DIR) && RUSTFLAGS="" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --bin transparent-enroll --bin transparent-prove --bin transparent-verify --bin capsule-attest-proof --target $(HOST_TARGET)

nonos-mk-zk-tools: $(ZK_ENROLL_TOOL) $(ZK_TRANSPARENT_PROVE_TOOL) $(ZK_TRANSPARENT_VERIFY_TOOL) $(ZK_CAPSULE_PROOF_TOOL)

# Transparent STARK enrollment tool, the production capsule attestation prover.
STARK_ENROLL_SRCS := $(shell find nonos-stark-enroll/src -type f -name '*.rs' 2>/dev/null) \
                     nonos-stark-enroll/Cargo.toml
$(NONOS_STARK_ENROLL): nonos-mk-check-deps $(STARK_ENROLL_SRCS)
	@echo "Building transparent STARK enrollment tool..."
	@cd nonos-stark-enroll && RUSTFLAGS="" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --target $(HOST_TARGET)

nonos-mk-ensure-zk-keys: $(ZK_BOOT_ROOT) $(ZK_BOOT_COMMITMENTS)

nonos-mk-verify-capsule-attest: nonos-mk-all-capsules-attested
	@printf "\n  transparent capsule attestation is kernel-verified at spawn\n\n"

# Self-verification of the shipped image. Five independent checks, each with a
# different source of truth, so no single compromised input passes quietly:
#   A  the trust artifact ledger, against the baked SHA-256 list
#   B  every manifest signature, Ed25519 + ML-DSA-65 against the trust anchor
#   C  every binary's .nonos.caps section, against the manifest it shipped with
#   D  every STARK membership trailer, re-verified with the same nonos-stark
#      gate the kernel runs at spawn and the bootloader runs before the jump
#   E  the policy root proven in D, byte-embedded in the kernel that boots
# Fail-closed: the first failure stops the build claiming to be verified.
.PHONY: nonos-mk-verify-image
nonos-mk-verify-image: $(NONOS_STARK_ENROLL) $(CAPSULE_SIGN_BIN)
	@echo "Self-verifying the image..."
	@echo "  [A] trust artifact ledger"
	@$(MAKE) --no-print-directory nonos-mk-check-trust-manifest >/dev/null
	@echo "        ok    baked SHA-256 ledger matches the tree"
	@echo "  [B] manifest signatures (Ed25519 + ML-DSA-65, trust anchor policy)"
	@t0=$$(date +%s); $(foreach s,$(NONOS_ENROLLED_CAPSULES),\
		$(CAPSULE_SIGN_BIN) verify-manifest \
			--manifest $($(s)_MANIFEST) --cert $($(s)_CERT) \
			--policy $(NONOS_TRUST_ANCHOR_POLICY_BIN) >/dev/null || exit 1;) \
	echo "        ok    $(words $(NONOS_ENROLLED_CAPSULES)) manifests verify ($$(($$(date +%s)-t0))s)"
	@echo "  [C] declared capabilities (.nonos.caps vs the signed manifest)"
	@t0=$$(date +%s); $(foreach s,$(NONOS_ENROLLED_CAPSULES),\
		$(NONOS_CAPS_CHECK) $($(s)_BIN) \
			--manifest-caps $($(s)_REQUIRED_CAPS) --allow-missing >/dev/null || exit 1;) \
	echo "        ok    $(words $(NONOS_ENROLLED_CAPSULES)) binaries declare what their manifests grant ($$(($$(date +%s)-t0))s)"
	@echo "  [D] STARK membership proofs (the gate the kernel runs at spawn)"
	@# No output piping here: a pipe would let the pipeline's last command mask
	@# the verifier's exit code, and a verifier that cannot fail the build is
	@# decoration. The tool's own per-capsule lines are the evidence.
	@$(NONOS_STARK_ENROLL) verify $(ZK_CAPSULE_ROOT) \
		$(foreach s,$(NONOS_ENROLLED_CAPSULES),$($(s)_REQUIRED_CAPS):$($(s)_BIN):$($(s)_ATTESTATION))
	@if [ -f "$(KERNEL_ATTEST_ROOT_BIN)" ] && [ -f "$(KERNEL_ATTEST_TRAILER)" ] && [ -f "$(dir $(KERNEL_ATTEST_TRAILER))kernel.enrolled.elf" ]; then \
		$(NONOS_STARK_ENROLL) verify-kernel "$(KERNEL_ATTEST_ROOT_BIN)" \
			"$(dir $(KERNEL_ATTEST_TRAILER))kernel.enrolled.elf" "$(KERNEL_ATTEST_TRAILER)" || exit 1; \
	else \
		echo "        --    kernel self-attestation artifacts not present in this profile"; \
	fi
	@echo "  [E] root embedding + build receipt"
	@$(NONOS_PYTHON) scripts/build_receipt.py \
		--policy-root "$(ZK_CAPSULE_ROOT)" \
		--kernel-attest-root "$(KERNEL_ATTEST_ROOT_BIN)" \
		--bootloader "$(BOOTLOADER_DIR)/target/x86_64-unknown-uefi/release/nonos_boot.efi" \
		--enrolled-elf "$(dir $(KERNEL_ATTEST_TRAILER))kernel.enrolled.elf" \
		--kernel "$(TARGET_DIR)/kernel_attested.bin" \
		--artifact "$(TARGET_DIR)/nonos.iso" \
		--artifact "$(ESP_DIR)/EFI/nonos/kernel.bin" \
		--out "$(TARGET_DIR)/attestation/build-receipt.json"

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

EMBED_TOOL_SRCS := $(shell find $(BOOTLOADER_DIR)/tools/embed-zk-proof/src -type f -name '*.rs' 2>/dev/null) \
                   $(BOOTLOADER_DIR)/tools/embed-zk-proof/Cargo.toml
$(EMBED_TOOL): nonos-mk-check-deps $(EMBED_TOOL_SRCS)
	@echo "Building ZK embed tool..."
	@cd $(BOOTLOADER_DIR)/tools/embed-zk-proof && RUSTFLAGS="" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --target $(HOST_TARGET)

SIGN_TOOL_SRCS := $(shell find $(BOOTLOADER_DIR)/tools/sign-kernel/src -type f -name '*.rs' 2>/dev/null) \
                  $(BOOTLOADER_DIR)/tools/sign-kernel/Cargo.toml
$(SIGN_TOOL): nonos-mk-check-deps $(SIGN_TOOL_SRCS)
	@echo "Building kernel signing tool..."
	@cd $(BOOTLOADER_DIR)/tools/sign-kernel && RUSTFLAGS="" RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --target $(HOST_TARGET)

ifeq ($(NONOS_DEVICE_BINDING),unbound)
# The vendor image carries no device slot set. The baked root is the
# sha256 of a fixed public tag, so it is reproducible by anyone and
# opens nothing: every runtime binding proof is refused until the
# installer enrolls the owner's slots and writes an enrolled loader.
# The boot gate itself is the kernel STARK self-attestation and does
# not touch this root. Secrets have no rule here on purpose; a target
# that wants them in an unbound build must fail loudly.
$(ZK_BOOT_ROOT):
	@mkdir -p $(dir $@)
	@printf '%s' 'NONOS-DEVICE-SLOT-UNBOUND-v1' | $(SHA256) | cut -c1-64 | xxd -r -p > $@
	@test "$$(wc -c < $@ | tr -d ' ')" = 32 || { echo "sentinel root malformed"; exit 1; }
	@echo "Device slot: unbound (sentinel root baked)"

$(ZK_BOOT_COMMITMENTS): $(ZK_BOOT_ROOT)
	@: > $@
else
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
endif

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
		$(GOP_PREF_STAMP) \
		$(TARGET_DIR)/.nonos-toolchain.stamp
	@echo "Building UEFI bootloader (policy: $(BOOTLOADER_POLICY))..."
	$(eval SIGNING_KEY_ABS := $(if $(filter /%,$(SIGNING_KEY)),$(SIGNING_KEY),$(shell pwd)/$(SIGNING_KEY)))
	@cd $(BOOTLOADER_DIR) && \
		$(if $(NONOS_TRUST_ANCHOR_PUBKEY),NONOS_TRUST_ANCHOR_PUBKEY=$(abspath $(NONOS_TRUST_ANCHOR_PUBKEY)),NONOS_SIGNING_KEY=$(SIGNING_KEY_ABS)) \
		NONOS_MLDSA65_PUBKEY=$(abspath $(KERNEL_MLDSA65_PUB)) \
		NONOS_ZK_DEVICE_ROOT=$(abspath $(ZK_BOOT_ROOT)) \
		$(if $(NONOS_STARK_KERNEL_ATTEST_ON),NONOS_KERNEL_ATTEST_ROOT=$(abspath $(KERNEL_ATTEST_ROOT_BIN))) \
		$(if $(NONOS_GOP_PREF),NONOS_GOP_PREF=$(NONOS_GOP_PREF)) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		RUSTFLAGS='-C panic=abort -C target-feature=+crt-static --cfg curve25519_dalek_backend="serial" --remap-path-prefix=$(abspath .)=/nonos -C link-arg=/DEBUG:NONE' \
		$(CARGO) build --target x86_64-unknown-uefi --release \
			--features zk-transparent,$(BOOTLOADER_POLICY)$(BOOT_STARK_FEATURE)
# The RUSTFLAGS line above is authoritative and mirrors the target
# flags in nonos-bootloader/.cargo/config.toml, which cargo ignores
# whenever the env var is set. The two additions make the loader
# byte-reproducible: checkout paths would otherwise reach the binary
# through panic locations, and the linker's debug directory carries a
# PDB record no UEFI release has any use for.

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
# Which user target the capsules and their libc are built for, so the whole
# userland pipeline follows one setting and an artifact path cannot disagree
# with what was compiled. Deliberately not named CAPSULE_TARGET: capsule.mk
# clears that one after every capsule so a per-capsule override cannot leak
# into the next, which would wipe a global set here.
NONOS_USER_TARGET ?= x86_64-nonos-user
export NONOS_USER_TARGET
USERLAND_LIBC := $(USERLAND_DIR)/libc/target/$(NONOS_USER_TARGET)/release/libnonos_libc.a
# The std startup object lives in the build tree, not /tmp. It must be an
# absolute path because capsules build from their own subdirectories and pass
# it to the linker as -Clink-arg. cargo clean removes it; the rule below
# rebuilds it. Override NONOS_RT_OBJ only if you have a reason to.
NONOS_RT_OBJ  ?= $(abspath $(TARGET_DIR))/nonos_rt.o
USERLAND_LIBC_SRCS := $(shell find $(USERLAND_DIR)/libc/src -name '*.rs') \
                      $(USERLAND_DIR)/libc/Cargo.toml \
                      $(USERLAND_DIR)/$(NONOS_USER_TARGET).json
NONOS_RT_SRCS := $(shell find toolchain/nonos-rt/src -name '*.rs' 2>/dev/null) \
                 toolchain/nonos-rt/Cargo.toml \
                 toolchain/nonos-rt/Cargo.lock \
                 $(USERLAND_DIR)/$(NONOS_USER_TARGET).json

# Trust-anchor inputs, global to all signed capsules. The two
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
		$(CARGO) rustc --release --target ../$(NONOS_USER_TARGET).json \
		-Zbuild-std=core --crate-type staticlib
	@touch $@

nonos-mk-libc: $(USERLAND_LIBC)

$(NONOS_RT_OBJ): $(NONOS_RT_SRCS) | $(TARGET_DIR)/.nonos-toolchain.stamp
	@echo "Building NONOS std startup object..."
	@cd toolchain/nonos-rt && \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) rustc --release --target ../../userland/$(NONOS_USER_TARGET).json \
		-Zbuild-std=core -- --emit obj=$(NONOS_RT_OBJ)
	@# `--emit obj=` only writes when cargo actually runs rustc. This crate keeps
	@# its own target directory, which `distclean` does not remove, so cargo can
	@# call it fresh, skip rustc and leave nothing behind while still exiting 0.
	@# Every std capsule then fails to link against an object that is not there.
	@test -f $(NONOS_RT_OBJ) || { \
		cd toolchain/nonos-rt && RUSTUP_TOOLCHAIN=$(TOOLCHAIN) $(CARGO) clean && cd ../.. && \
		cd toolchain/nonos-rt && RUSTUP_TOOLCHAIN=$(TOOLCHAIN) $(CARGO) rustc --release \
			--target ../../userland/$(NONOS_USER_TARGET).json \
			-Zbuild-std=core -- --emit obj=$(NONOS_RT_OBJ); }
	@test -f $(NONOS_RT_OBJ) || { echo "nonos_rt.o was not produced"; exit 1; }

# std platform layer: patch the pinned rust-src so -Zbuild-std=std turns
# unmodified `use std::...` crates into NONOS binaries. Stamped and keyed on
# the PAL sources + apply.sh, so a clean checkout (or a rustup update that
# reset rust-src) re-applies it before any std capsule builds. Without this
# the patch was a manual prerequisite, hidden state that broke fresh trees.
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
		userland/$(NONOS_USER_TARGET).json | $(TARGET_DIR)/.nonos-toolchain.stamp
	@echo "Building upstream ripgrep $(UPSTREAM_RIPGREP_VERSION) for NONOS (unmodified crates.io source)..."
	@RUSTUP_TOOLCHAIN=$(TOOLCHAIN) RUSTFLAGS="-Clink-arg=$(abspath $(NONOS_RT_OBJ))" \
		$(CARGO) install ripgrep --version $(UPSTREAM_RIPGREP_VERSION) \
		--target $(abspath userland/$(NONOS_USER_TARGET).json) \
		-Zbuild-std=std,panic_abort -Zbuild-std-features=compiler-builtins-mem \
		--root $(abspath $(TARGET_DIR)/upstream-ripgrep) --no-track --force --bin rg
	@cp $(TARGET_DIR)/upstream-ripgrep/bin/rg $@

.PHONY: nonos-mk-upstream-ripgrep
nonos-mk-upstream-ripgrep: $(UPSTREAM_RIPGREP_BIN)

# sd needs the is-terminal cfg-gap override in .cargo/config.toml, and cargo
# only honors [patch] for path builds (published manifests strip it), so the
# pinned crates.io source is vendored byte-identical under upstream-src and
# built with --path. The program source itself is unmodified.
UPSTREAM_SD_VERSION := 1.0.0
UPSTREAM_SD_SRC     := userland/upstream-src/sd-$(UPSTREAM_SD_VERSION)
UPSTREAM_SD_BIN     := $(TARGET_DIR)/upstream-sd/sd
$(UPSTREAM_SD_BIN): $(NONOS_RT_OBJ) $(NONOS_STD_PAL_STAMP) \
		userland/$(NONOS_USER_TARGET).json | $(TARGET_DIR)/.nonos-toolchain.stamp
	@echo "Building upstream sd $(UPSTREAM_SD_VERSION) for NONOS (unmodified crates.io source)..."
	@cd $(UPSTREAM_SD_SRC) && RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		RUSTFLAGS="-Clink-arg=$(abspath $(NONOS_RT_OBJ))" \
		$(CARGO) install --path . \
		--target $(abspath userland/$(NONOS_USER_TARGET).json) \
		-Zbuild-std=std,panic_abort -Zbuild-std-features=compiler-builtins-mem \
		--root $(abspath $(TARGET_DIR)/upstream-sd) --no-track --force --bin sd
	@cp $(TARGET_DIR)/upstream-sd/bin/sd $@

.PHONY: nonos-mk-upstream-sd
nonos-mk-upstream-sd: $(UPSTREAM_SD_BIN)

# tokio-smoke: the mio-backend + socket2-shim runtime gate. Built like sd from
# vendored source with the per-program mio/socket2/tokio patches; runs an async
# runtime that opens a real TCP connection, proving the reactor and Waker.
UPSTREAM_TOKIO_SMOKE_SRC := userland/upstream-src/tokio-smoke
UPSTREAM_TOKIO_SMOKE_BIN := $(TARGET_DIR)/upstream-tokio-smoke/tokio-smoke
$(UPSTREAM_TOKIO_SMOKE_BIN): $(NONOS_RT_OBJ) $(NONOS_STD_PAL_STAMP) \
		userland/$(NONOS_USER_TARGET).json | $(TARGET_DIR)/.nonos-toolchain.stamp
	@echo "Building tokio-smoke runtime gate for NONOS (tokio via mio backend + socket2 shim)..."
	@cd $(UPSTREAM_TOKIO_SMOKE_SRC) && RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		RUSTFLAGS="-Clink-arg=$(abspath $(NONOS_RT_OBJ))" \
		$(CARGO) install --path . \
		--target $(abspath userland/$(NONOS_USER_TARGET).json) \
		-Zbuild-std=std,panic_abort -Zbuild-std-features=compiler-builtins-mem \
		--root $(abspath $(TARGET_DIR)/upstream-tokio-smoke) --no-track --force --bin tokio-smoke
	@cp $(TARGET_DIR)/upstream-tokio-smoke/bin/tokio-smoke $@

.PHONY: nonos-mk-upstream-tokio-smoke
nonos-mk-upstream-tokio-smoke: $(UPSTREAM_TOKIO_SMOKE_BIN)

# Installed crates.io command-line tools, cross-compiled unmodified for
# x86_64-nonos from the byte-identical source vendored under upstream-src (cargo
# only honors [patch] for path builds, so shimmed crates must build with --path).
# Each tool's Capsule.mk consumes target/upstream-<bin>/bin/<bin>; this template
# is the one rule that builds it, so adding a tool is a name in NONOS_TOOL_BINS
# plus its `nonos-app add`, not a new recipe. getrandom-backed tools take the
# RDRAND backend, which nonos x86_64 always has.
NONOS_TOOL_BINS := grex dotenv-linter pastel jsonxf tokei huniq csview

# Default is to strip default features (they pull unix-only or update-check
# extras nonos does not want). grex and tokei gate their binary behind a `cli`
# feature, so those two re-enable it. Per-tool overrides live in <bin>_CARGO_FEATURES.
NONOS_TOOL_FEATURES_DEFAULT := --no-default-features
grex_CARGO_FEATURES  := --no-default-features --features cli
tokei_CARGO_FEATURES := --no-default-features --features cli

define nonos_upstream_tool_rule
$(TARGET_DIR)/upstream-$(1)/bin/$(1): $(NONOS_RT_OBJ) $(NONOS_STD_PAL_STAMP) \
		userland/$(NONOS_USER_TARGET).json | $(TARGET_DIR)/.nonos-toolchain.stamp
	@echo "Building upstream $(1) for NONOS (unmodified crates.io source)..."
	@cd userland/upstream-src/$(1) && RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		RUSTFLAGS="-Clink-arg=$(abspath $(NONOS_RT_OBJ)) --cfg getrandom_backend=\"rdrand\"" \
		$(CARGO) install --path . $(or $($(1)_CARGO_FEATURES),$(NONOS_TOOL_FEATURES_DEFAULT)) \
		--target $(abspath userland/$(NONOS_USER_TARGET).json) \
		-Zbuild-std=std,panic_abort -Zbuild-std-features=compiler-builtins-mem \
		--root $(abspath $(TARGET_DIR)/upstream-$(1)) --no-track --force --bin $(1)
endef
$(foreach t,$(NONOS_TOOL_BINS),$(eval $(call nonos_upstream_tool_rule,$(t))))

.PHONY: nonos-mk-upstream-tools
nonos-mk-upstream-tools: $(foreach t,$(NONOS_TOOL_BINS),$(TARGET_DIR)/upstream-$(t)/bin/$(t))

# Real source prerequisites, same shape as BOOTLOADER_SRCS: a restored
# cache can hand back a stale binary, and a bare file target would then
# never consult cargo again, so a submodule bump never reached the tool.
CAPSULE_SIGN_SRCS := $(shell find nonos-sign/src -type f -name '*.rs' 2>/dev/null) \
                     nonos-sign/Cargo.toml nonos-sign/Cargo.lock
$(CAPSULE_SIGN_BIN): $(CAPSULE_SIGN_SRCS)
	@echo "Building capsule-sign host tool..."
	@cd nonos-sign && cargo build --release --bin capsule-sign

NONOS_PACK_BIN := tools/nonos-pack/target/release/nonos-pack
$(NONOS_PACK_BIN):
	@echo "Building nonos-pack host tool..."
	@cd tools/nonos-pack && cargo build --release --bin nonos-pack

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

# Re-stamp the ledger over the current trust tree. A local rebuild legitimately
# regenerates trailers and signatures (STARK grinding is randomized), so the
# build refreshes the ledger it will then verify against; publishing commits
# the refreshed ledger, and from that point the check pins the shipped set.
# Deterministic order, so two stamps of one tree are byte-identical.
.PHONY: nonos-mk-trust-ledger
nonos-mk-trust-ledger:
	@cd $(NONOS_TRUST_DIR) && \
		find capsules keys policy zk -type f ! -name MANIFEST.sha256 2>/dev/null \
		| LC_ALL=C sort | xargs $(SHA256) > MANIFEST.sha256
	@echo "Trust ledger re-stamped over $$(wc -l < $(NONOS_TRUST_DIR)/MANIFEST.sha256 | tr -d ' ') artifacts."

nonos-mk-verify-trust: nonos-mk-desktop-gui-prod
	@$(MAKE) nonos-mk-host-trust-verify
	@$(MAKE) nonos-mk-check-trust-manifest

# Nothing signs in a reuse build, so no seed is required to exist.
ifeq ($(NONOS_TRUST_REUSE),1)
# The about and settings capsules bake a commit id at compile time. The
# enrolled binaries carry the commit the enrollment builder ran on, so a
# verifying rebuild at any later commit must pin the same value or drift
# by exactly that embedded string. The keystore records it at enrollment.
NONOS_ENROLLED_COMMIT := $(strip $(shell cat nonos-data/trust/COMMIT 2>/dev/null))
ifneq ($(NONOS_ENROLLED_COMMIT),)
export NONOS_BUILD_SHA := $(NONOS_ENROLLED_COMMIT)
endif

nonos-mk-check-trust-keys:
	@true

$(NONOS_TRUST_ANCHOR_POLICY_BIN):
	@test -f $@ || { echo "::error::$@ is not committed; a reuse build cannot seal it"; exit 1; }
else
nonos-mk-check-trust-keys:
	@for f in $(NONOS_TA_ED25519_SEED) $(NONOS_TA_ED25519_PUB) \
	          $(NONOS_TA_MLDSA65_SEED) $(NONOS_TA_MLDSA65_PUB); do \
	    [ -f "$$f" ] || { \
	        echo "::error::missing $$f; generate via: $(CAPSULE_SIGN_BIN) keygen --alg <ed25519|mldsa65> --out <prefix>"; \
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
endif

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
include userland/capsule_sd/Capsule.mk
ifneq ($(wildcard userland/capsule_flacprobe/Capsule.mk),)
  include userland/capsule_flacprobe/Capsule.mk
endif
include userland/capsule_csview/Capsule.mk
include userland/capsule_huniq/Capsule.mk
include userland/capsule_tokei/Capsule.mk
include userland/capsule_jsonxf/Capsule.mk
include userland/capsule_pastel/Capsule.mk
include userland/capsule_dotenv-linter/Capsule.mk
include userland/capsule_grex/Capsule.mk
include userland/capsule_tokio_smoke/Capsule.mk
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
include userland/capsule_video_player/Capsule.mk
include userland/capsule_clipboard/Capsule.mk
include userland/capsule_login/Capsule.mk
include userland/toolkit/Capsule.mk
include userland/capsule_about/Capsule.mk
include userland/capsule_hello/Capsule.mk
include userland/capsule_gui_demo/Capsule.mk
include userland/capsule_game_2048/Capsule.mk
include userland/capsule_egui_proof/Capsule.mk
include userland/capsule_mdview/Capsule.mk
include userland/capsule_qrgen/Capsule.mk
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
include userland/capsule_audio/Capsule.mk
include userland/capsule_audio_player/Capsule.mk
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
include userland/capsule_socks5/Capsule.mk
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
# Which capsules go under the root. Every verified capsule by default, which
# is what an x86_64 image ships. A target that cannot build the whole set
# narrows this to what it does ship, since enrolling a capsule means measuring
# a binary that has to exist for that architecture.
NONOS_ENROLLED_CAPSULES ?= $(NONOS_VERIFIED_CAPSULES)

# A reuse build never re-enrolls: the committed root and trailers are
# the enrolled truth, and the per-capsule manifest checks already pin
# every fresh ELF to its enrolled measurement. Re-running the grinder
# here would replace owner-enrolled artifacts with pipeline ones.
ifeq ($(NONOS_TRUST_REUSE),1)
$(ZK_CAPSULE_ROOT):
	@test -f $@ || { echo "::error::$@ is not committed; a reuse build cannot enroll"; exit 1; }
else
$(ZK_CAPSULE_ROOT): $(NONOS_STARK_ENROLL) \
		$(foreach s,$(NONOS_ENROLLED_CAPSULES),$($(s)_BIN) $($(s)_MANIFEST))
	@echo "Enrolling $(words $(NONOS_ENROLLED_CAPSULES)) capsules under one transparent STARK policy root..."
	@mkdir -p $(dir $(ZK_CAPSULE_ROOT)) $(NONOS_BAKED_TRUST_DIR)/capsules
	@$(NONOS_STARK_ENROLL) capsules $(ZK_CAPSULE_ROOT) \
		$(foreach s,$(NONOS_ENROLLED_CAPSULES),$($(s)_REQUIRED_CAPS):$($(s)_BIN):$($(s)_ATTESTATION))
endif

.PHONY: nonos-mk-stark-enroll-capsules
nonos-mk-stark-enroll-capsules: $(ZK_CAPSULE_ROOT)
	@echo "Enrolled the capsule set under the STARK policy root $(ZK_CAPSULE_ROOT)"

# Every verified capsule binary, the exact set the trust chain covers.
# The enrollment builder uses this so no capsule can be silently absent
# from what gets enrolled; the hand-listed host-trust subset is not it.
.PHONY: nonos-mk-verified-elfs
nonos-mk-verified-elfs: $(foreach s,$(NONOS_VERIFIED_CAPSULES),$($(s)_BIN))
	@echo "Built $(words $(NONOS_VERIFIED_CAPSULES)) verified capsule binaries."

# One capsule binary path per line, for tooling that mirrors the set:
# the enrollment builder packages exactly this list, nothing inferred.
.PHONY: nonos-mk-print-verified-bins
nonos-mk-print-verified-bins:
	@$(foreach s,$(NONOS_VERIFIED_CAPSULES),echo $($(s)_BIN);)

nonos-mk-all-capsules-attested: $(NONOS_VERIFIED_ARTIFACTS)
	@echo "Signed and attested $(words $(NONOS_VERIFIED_CAPSULES)) included capsules."

# Enroll the kernel's own measurement and emit its self-attestation root and
# trailer. The bootloader embeds the root (NONOS_KERNEL_ATTEST_ROOT) and verifies
# the trailer, carried in the kernel image footer, before jumping. The tool
# re-checks the trailer against the same verifier the bootloader runs.
# One rule, one run. A multi-target rule executes its recipe once per demanded
# target, so under -j the root and the trailer were enrolled by two concurrent
# processes writing the same files, and the interleaved trailer verified
# against nothing. The stamp makes the enrollment a single grouped step the
# portable way (macOS ships make 3.81, which has no `&:` grouped targets).
KERNEL_ATTEST_STAMP := $(TARGET_DIR)/kernel-attest/.enrolled
$(KERNEL_ATTEST_STAMP): $(KERNEL_ATTEST_ELF) $(NONOS_STARK_ENROLL)
	@echo "Enrolling the kernel self-attestation..."
	@mkdir -p $(dir $(KERNEL_ATTEST_ROOT_BIN)) $(dir $(KERNEL_ATTEST_TRAILER))
	@# Snapshot first, enroll the snapshot: the loose ELF path is shared by
	@# several kernel profiles and can be relinked mid-build, so the proof is
	@# issued over frozen bytes, verify-kernel runs against the same frozen
	@# bytes, and the receipt ties them to kernel_attested.bin by byte prefix.
	@cp $(KERNEL_ATTEST_ELF) $(dir $(KERNEL_ATTEST_TRAILER))kernel.enrolled.elf
	@$(NONOS_STARK_ENROLL) kernel $(dir $(KERNEL_ATTEST_TRAILER))kernel.enrolled.elf \
		$(KERNEL_ATTEST_ROOT_BIN) $(KERNEL_ATTEST_TRAILER)
	@touch $@

# With the attest gate on, the loader bakes the root that enrollment
# produces, so a fresh tree must enroll before the loader compiles;
# under -j the two otherwise race and the loader's build script
# refuses, correctly. Gate off, a standalone loader build stays free
# of any enrollment. Declared here because the stamp path is assigned
# just above; earlier in the file it would expand empty and bind
# nothing.
ifeq ($(NONOS_STARK_KERNEL_ATTEST_ON),1)
$(BOOTLOADER_DIR)/target/x86_64-unknown-uefi/release/nonos_boot.efi: $(KERNEL_ATTEST_STAMP)
endif

# The stamp says enrollment ran; the byproducts live in two directories with
# more than one historical cleaner. If either file is gone the stamp is stale
# by definition, so invalidate it and re-drive rather than trust it.
.PHONY: nonos-mk-kernel-attest-ensure
nonos-mk-kernel-attest-ensure:
	@if [ ! -f "$(KERNEL_ATTEST_ROOT_BIN)" ] || [ ! -f "$(KERNEL_ATTEST_TRAILER)" ]; then \
		rm -f $(KERNEL_ATTEST_STAMP); \
	fi
	@$(MAKE) --no-print-directory $(KERNEL_ATTEST_STAMP)

nonos-mk-kernel-attest: $(KERNEL_ATTEST_STAMP)
	@echo "Kernel self-attestation enrolled: root $(KERNEL_ATTEST_ROOT_BIN)"

NONOS_DESKTOP_GUI_CAPSULE_CHECKS = \
	$(proof-io_VERIFY) $(std-proof_VERIFY) $(ripgrep_VERIFY) $(sd_VERIFY) $(tokio-smoke_VERIFY) \
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

MARKETPLACE_ABI_LIB := $(USERLAND_DIR)/marketplace_abi/target/$(NONOS_USER_TARGET)/release/libnonos_marketplace_abi.rlib

$(MARKETPLACE_ABI_LIB):
	@echo "Building marketplace ABI rlib..."
	@cd $(USERLAND_DIR)/marketplace_abi && \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) \
		$(CARGO) build --release --target ../$(NONOS_USER_TARGET).json \
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
		$(CARGO) build --release --target ../$(NONOS_USER_TARGET).json \
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

# Kernel. Every target spells the profile out explicitly.

KERNEL_BUILD_FLAGS := --release --target x86_64-nonos.json \
		-Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem

ARM_KERNEL_BUILD_FLAGS := --release --target aarch64-nonos.json \
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
#
# The signing key is an order-only prerequisite for the same reason. A rotated
# key is newer than the kernel ELF, and as a normal prerequisite that rebuilt
# this featureless kernel over the same output path a feature variant writes.
# The signer then dual-signed and attested a microkernel-core image while the
# log still said microkernel-full-gui. The key has to exist; its age says
# nothing about whether this ELF is current.
$(TARGET_DIR)/x86_64-nonos/release/nonos-kernel: | $(SIGNING_KEY)
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

# A note on aarch64-nonos.json, which cannot carry comments of its own: the cpu
# stays "generic" so codegen targets ARMv8.0, and the feature bits only let the
# assembler accept the mnemonics the kernel writes by hand: pointer authentication,
# the speculation barriers, memory tagging (gcr_el1) and the v8.5 RNG (rndr). Named
# one by one rather than as an architecture version, because +v8.5a would also let
# LLVM emit v8.5 instructions into ordinary code and those fault on a Cortex-A72.
# Every one of these is reached through a runtime ID register check, so a single
# binary runs on a Pi and still uses tagging and PAC on an Apple M-series.
#
# Build the aarch64 kernel. PATH puts the rustup shims first because a Homebrew
# rustc on /usr/local/bin shadows them and then rejects the -Z flags with a
# confusing "only accepted on the nightly compiler".
.PHONY: nonos-mk-arm nonos-mk-arm-bench nonos-mk-bench-micro nonos-mk-arm-run nonos-mk-arm-gui nonos-mk-arm-gui-capsules nonos-mk-arm-gui-run
nonos-mk-arm: nonos-mk-ensure-signing-key
	@echo "Building kernel (aarch64, microkernel-core + nonos-arch-preview)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) PATH="$(HOME)/.cargo/bin:$$PATH" \
		$(CARGO) build $(ARM_KERNEL_BUILD_FLAGS) \
		--no-default-features --features microkernel-core$(_boot_comma)nonos-arch-preview

# Boot the aarch64 kernel under QEMU. Every flag here is load bearing.
#
#   gic-version=3   virt defaults to a GICv2, and the kernel drives a v3. Without
#                   this the redistributor is not there and the first access to it
#                   aborts.
#   -cpu max        advertises the optional features, so the paths guarded by an
#                   ID register check are the ones that actually run. It is the
#                   closer proxy for a Neoverse or an M series part than a72 is,
#                   and it is what caught the MTE and SVE gaps.
#   virtio-rng-pci  the RNG refuses to seed from nothing, so without an entropy
#                   source the boot stops at init_rng and says so.
#
# QEMU passes no device tree when it boots an ELF, so x0 arrives as zero and the
# board facts come from the defaults in BootInfo. That path is what this target
# exercises; a device tree boot needs an Image-format target instead.
ARM_QEMU_FLAGS := -M virt,gic-version=3 -cpu max -m 512 -nographic \
		-serial mon:stdio -device virtio-rng-pci

# Build with the in-kernel microbenchmarks compiled in and boot far enough to
# print them. The numbers are cycle counts from the kernel's own paths, so they
# belong to this image and this machine, not to a marketing table.
nonos-mk-arm-bench: nonos-mk-ensure-signing-key
	@echo "Building kernel (aarch64, microkernel-core + nonos-bench-micro)..."
	@$(SDK_FLAGS) NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) PATH="$(HOME)/.cargo/bin:$$PATH" \
		$(CARGO) build $(ARM_KERNEL_BUILD_FLAGS) \
		--no-default-features \
		--features microkernel-core$(_boot_comma)nonos-arch-preview$(_boot_comma)nonos-bench-micro

# The same measurement image for x86_64. The kernel paths under test are the
# shared ones, so the two runs are comparable and the difference between them
# is the architecture rather than the build.
nonos-mk-bench-micro: nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-core + nonos-bench-micro,microkernel-core$(_boot_comma)nonos-bench-micro)

nonos-mk-arm-run: nonos-mk-arm
	@echo "Booting aarch64 kernel under QEMU (ctrl-a x to quit)..."
	@qemu-system-aarch64 $(ARM_QEMU_FLAGS) \
		-kernel $(TARGET_DIR)/aarch64-nonos/release/nonos-kernel

# The desktop needs a framebuffer, so this swaps -nographic for a virtio-gpu and
# keeps the serial log on the terminal. virt has no VGA to fall back on: without
# a display device the compositor has nothing to draw into.
ARM_GUI_QEMU_FLAGS := -M virt,gic-version=3 -cpu max -m 2048 \
		-device virtio-gpu-pci -device virtio-keyboard-pci \
		-device virtio-tablet-pci -device virtio-rng-pci \
		-serial mon:stdio

nonos-mk-arm-gui-run: nonos-mk-arm-gui
	@echo "Booting aarch64 desktop under QEMU..."
	@qemu-system-aarch64 $(ARM_GUI_QEMU_FLAGS) \
		-kernel $(TARGET_DIR)/aarch64-nonos/release/nonos-kernel

nonos-mk-core: nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-core,microkernel-core)

# Core kernel with the transparent STARK attestation backend selected. The
# spawn gate verifies a money-grade membership proof instead of the pairing
# check, and the kernel self-attestation module is compiled in. The ship
# profiles carry the same feature with the full capsule set enrolled; this
# lane compiles the gate without any capsules, for gate work and CI checks.
nonos-mk-core-attested: nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-core + nonos-stark-attest,microkernel-core$(_boot_comma)nonos-stark-attest)

# -----------------------------------------------------------------------------
# Configurable build. Choose components with the menuconfig-style tool and build
# exactly that set. This is the front door for assembling your own kernel; the
# curated images are `make` (the full production system) and `make qemu`.
# -----------------------------------------------------------------------------

# The feature set and prerequisites for the configured build. A component
# selection (NONOS_PROFILE := custom) carries its own explicit feature list and
# the slugs it needs built; a hand-written profile resolves the standard way.
# Either way the capsule set is enrolled under one policy root so the kernel it
# embeds can attest every capsule it spawns.
ifeq ($(NONOS_PROFILE),custom)
FROM_CONFIG_FEATURES := $(NONOS_CUSTOM_FEATURES)
FROM_CONFIG_DEPS := nonos-mk-all-capsules-attested \
	$(foreach s,$(NONOS_CUSTOM_SLUGS),$($(s)_ARTIFACTS))
else
FROM_CONFIG_FEATURES = $(NONOS_PROFILE)$(if $(filter 1,$(NONOS_ATTEST)),$(_boot_comma)nonos-stark-attest)
FROM_CONFIG_DEPS := nonos-mk-all-capsules-attested
endif

# Open the interactive configurator and write .nonos-config.
nonos-mk-menuconfig:
	@./tools/nonos-config

# Build the exact system described by .nonos-config and package it into a
# bootable image: enrol the capsules under the policy root, build the kernel
# with just the chosen features, then sign, attest, and package the ESP. Fails
# clearly if no config has been written yet.
nonos-mk-from-config: $(FROM_CONFIG_DEPS) nonos-mk-check-deps nonos-mk-ensure-signing-key
	@test -f .nonos-config || { echo "no .nonos-config; run 'make menuconfig' first"; exit 1; }
	@echo "Building from .nonos-config ($(NONOS_PROFILE), rollback index $(NONOS_ROLLBACK_INDEX))..."
	$(call nonos_kernel_build,$(NONOS_PROFILE) from .nonos-config,$(FROM_CONFIG_FEATURES))
	@$(MAKE) --no-print-directory nonos-mk-esp
	@echo
	@echo "Bootable image ready: $(ESP_DIR)"
	@echo "Boot it with:  make qemu-from-config"

# Boot the image from-config just packaged, without rebuilding a different one.
nonos-mk-run-from-config: $(QEMU_BLK_IMG) $(QEMU_OVMF_VARS_RW)
	@test -f $(ESP_DIR)/EFI/nonos/kernel.bin || { echo "no image; run 'make from-config' first"; exit 1; }
	@mkdir -p $(dir $(QEMU_SERIAL_LOG))
	@echo "Booting the from-config image in QEMU (serial log: $(QEMU_SERIAL_LOG))..."
	@$(QEMU) -m $(QEMU_MEM) -accel hvf -cpu host,+rdrand,+rdseed -smp 1 -machine q35 \
		-drive "format=raw,file=fat:rw:$(ESP_DIR)" \
		-drive if=pflash,format=raw,readonly=on,file="$(OVMF)" \
		$(QEMU_BLK) $(QEMU_GPU) $(QEMU_NET) $(QEMU_USB) $(QEMU_RNG) \
		-serial "file:$(QEMU_SERIAL_LOG)" -display none -no-reboot

# Short aliases, so a user types what the help prints.
.PHONY: menuconfig from-config qemu-from-config nonos-mk-run-from-config
menuconfig: nonos-mk-menuconfig
from-config: nonos-mk-from-config
qemu-from-config: nonos-mk-run-from-config

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

nonos-mk-video-player-test: $(proof-io_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building video_player capsule (nonos-video-player-smoketest)..."
	@$(MAKE) -B video-player_CARGO_FEATURES=nonos-video-player-smoketest nonos-mk-video-player-sign
	$(call nonos_kernel_build,microkernel-video-player-smoketest,microkernel-video-player-smoketest)

nonos-mk-driver-hda-smoketest-test: $(proof-io_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building kernel (microkernel-driver-hda-smoketest)..."
	@$(MAKE) -B driver-hda_CARGO_FEATURES=nonos-driver-hda-smoketest nonos-mk-driver-hda-sign
	@$(MAKE) nonos-mk-audio-sign
	@$(MAKE) nonos-mk-stark-enroll-capsules
	$(call nonos_kernel_build,microkernel-driver-hda-smoketest + nonos-stark-attest,microkernel-driver-hda-smoketest$(_boot_comma)nonos-stark-attest)

# Dev fast-path for driver-hda iterations. Depends on the capsules' _MANIFEST
# (which pulls _BIN + _CERT) but deliberately NOT their _ATTESTATION, so a
# changed capsule ELF does not pull $(ZK_CAPSULE_ROOT) and re-trigger the
# ~11min STARK enrollment. The kernel is built with nonos-stark-attest +
# nonos-zk-rollout: it still runs the real attest verify against whatever
# (now stale) trailer is embedded, but a mismatch is logged, not fatal.
# nonos-zk-rollout is compile_error-exclusive with nonos-production, so this
# can never leak into a ship build. Select it with
# HDA_BUILD_TARGET=nonos-mk-driver-hda-smoketest-dev-test.
nonos-mk-driver-hda-smoketest-dev-test: $(proof-io_MANIFEST) $(audio_MANIFEST) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building driver-hda capsule (nonos-driver-hda-smoketest feature)..."
	@$(MAKE) -B driver-hda_CARGO_FEATURES=nonos-driver-hda-smoketest nonos-mk-driver-hda-sign
	@echo "Building kernel (microkernel-driver-hda-smoketest, zk-rollout; no enroll)..."
	$(call nonos_kernel_build,microkernel-driver-hda-smoketest + nonos-zk-rollout,microkernel-driver-hda-smoketest$(_boot_comma)nonos-stark-attest$(_boot_comma)nonos-zk-rollout)

nonos-mk-audio-player-smoketest-test: $(proof-io_ARTIFACTS) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building capsules (audio-player smoketest)..."
	@$(MAKE) -B audio_player_CARGO_FEATURES=nonos-audio-player-smoketest vfs_CARGO_FEATURES=seed-audio-store \
		nonos-mk-audio_player-sign nonos-mk-vfs-sign
	@$(MAKE) nonos-mk-driver-hda-sign
	@$(MAKE) nonos-mk-audio-sign
	@$(MAKE) nonos-mk-stark-enroll-capsules
	$(call nonos_kernel_build,microkernel-audio-player-smoketest + nonos-stark-attest,microkernel-audio-player-smoketest$(_boot_comma)nonos-stark-attest)

# Dev fast-path for audio-player iterations. See the driver-hda dev-test note:
# depends on _MANIFEST (not _ATTESTATION) so a changed capsule ELF does not pull
# $(ZK_CAPSULE_ROOT) and re-trigger the ~11min STARK enrollment. The kernel runs
# the real attest verify but a mismatch is logged, not fatal (nonos-zk-rollout,
# compile_error-exclusive with nonos-production so it can never leak into a ship
# build). vfs is rebuilt with seed-audio-store so the boot WAV is present.
nonos-mk-audio-player-smoketest-dev-test: $(proof-io_MANIFEST) $(audio_MANIFEST) $(driver-hda_MANIFEST) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@echo "Building capsules (audio-player smoketest, dev; no enroll)..."
	@$(MAKE) -B audio_player_CARGO_FEATURES=nonos-audio-player-smoketest vfs_CARGO_FEATURES=seed-audio-store \
		nonos-mk-audio_player-sign nonos-mk-vfs-sign
	@echo "Building kernel (microkernel-audio-player-smoketest, zk-rollout; no enroll)..."
	$(call nonos_kernel_build,microkernel-audio-player-smoketest + nonos-zk-rollout,microkernel-audio-player-smoketest$(_boot_comma)nonos-stark-attest$(_boot_comma)nonos-zk-rollout)

# nonos-mk-desktop-gui-prod: the QEMU-bootable profile. This is NOT just a lighter
# cut of zerostate; it deliberately excludes the real-hardware driver capsules
# (iwlwifi, rtl8821ce, rtl8169, e1000, rtl8139, ahci, hda, nvme, usb-msc, i2c)
# that have no counterpart under QEMU and block on spawn when their hardware is
# absent. The runtime, debug, and benchmark boot targets use this so a QEMU boot
# reaches the ready marker. The full hardware image is nonos-mk-zerostate, which
# is what ships and what the ISO carries; do not fold this into it.
# The capsules that link against the std platform layer. That layer issues
# syscalls with raw x86_64 registers, so these do not cross-compile to aarch64
# yet and are kept separate from the base set.
DESKTOP_STD_TOOL_ARTIFACTS := $(std-proof_ARTIFACTS) $(ripgrep_ARTIFACTS) \
		$(sd_ARTIFACTS) \
		$(flacprobe_ARTIFACTS) \
		$(csview_ARTIFACTS) \
		$(huniq_ARTIFACTS) \
		$(tokei_ARTIFACTS) \
		$(jsonxf_ARTIFACTS) \
		$(pastel_ARTIFACTS) \
		$(dotenv-linter_ARTIFACTS) \
		$(grex_ARTIFACTS) \
		$(tokio-smoke_ARTIFACTS)

# Named by slug so the two flavours below stay in step. A capsule's
# `_ARTIFACTS` includes its STARK attestation trailer, and generating one
# enrols every capsule in the policy root, tool capsules included.
DESKTOP_BASE_SLUGS := proof-io ramfs keyring entropy crypto vfs \
		driver-virtio-rng driver-virtio-blk driver-virtio-gpu \
		driver-virtio-net driver-ps2-input driver-xhci driver-usb-hid \
		net-core net-sockets net-nym socks5 policy wallpaper_catalog \
		installer input-router compositor wm desktop-shell image-codec \
		clipboard login wallpaper toolkit about boot-splash calculator \
		browser wallet-nonos terminal file-manager text-editor \
		settings process-manager attest power \
		audio driver-hda audio_player video-player

DESKTOP_BASE_CAPSULE_ARTIFACTS := \
		$(foreach s,$(DESKTOP_BASE_SLUGS),$($(s)_ARTIFACTS))

# The same capsules signed, without the attestation trailer. A profile built
# without the STARK spawn gate never reads one, and depending on it would drag
# the whole enrolment in.
DESKTOP_BASE_SIGNED_ARTIFACTS := \
		$(foreach s,$(DESKTOP_BASE_SLUGS),$($(s)_BIN) $($(s)_CERT) $($(s)_MANIFEST))

# The policy root enrols every capsule the STARK spawn gate will admit, tool
# capsules included, so it belongs with the profile that compiles that gate in.
DESKTOP_GUI_CAPSULE_ARTIFACTS := $(DESKTOP_BASE_CAPSULE_ARTIFACTS) \
		$(DESKTOP_STD_TOOL_ARTIFACTS) \
		$(ZK_POLICY_ROOT)

nonos-mk-desktop-gui-prod: $(DESKTOP_GUI_CAPSULE_ARTIFACTS) \
		nonos-mk-verify-desktop-gui-capsules \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,microkernel-desktop-gui + nonos-stark-attest,microkernel-desktop-gui$(_boot_comma)nonos-stark-attest)

# The fast loop. Proving is a release cost, never an iteration cost: the policy
# tree commits to the whole capsule set, so one changed capsule re-proves all of
# them (~10 minutes of STARK grinding), which no edit-compile-boot loop should
# ever pay. This target builds and signs exactly what changed and the kernel in
# rollout mode (`nonos-zk-rollout`: a stale proof is logged at spawn, not
# fatal), reusing the trailers already on disk. Signing still runs, capability
# checks still run; only the membership proofs are allowed to be stale, and the
# boot log says so on every spawn. `make` remains the only path that proves.
.PHONY: nonos-mk-dev
nonos-mk-dev: $(filter-out %.zk_trailer.bin,$(DESKTOP_GUI_CAPSULE_ARTIFACTS)) \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	@ls $(NONOS_BAKED_TRUST_DIR)/capsules/*.zk_trailer.bin >/dev/null 2>&1 || { \
		echo "no trailers on disk yet; run 'make' once so dev builds have proofs to carry"; exit 1; }
	@echo "DEV BUILD: membership proofs may be stale by design; 'make' is the proving path."
	$(call nonos_kernel_build,microkernel-desktop-gui + rollout (DEV),microkernel-desktop-gui$(_boot_comma)nonos-stark-attest$(_boot_comma)nonos-zk-rollout)
	@$(MAKE) --no-print-directory nonos-mk-esp
	@echo "Dev image ready: make dev-qemu boots it."

# The desktop for aarch64. The capsule pass runs as a sub-make so
# NONOS_USER_TARGET reaches the artefact paths, which are expanded when the
# rules are read and so cannot be redirected by a target-specific variable.
# Signing runs over the aarch64 ELFs, and the certificates it writes land in
# the shared trust directory, so an x86_64 image built earlier needs its own
# capsule pass again before it will boot.
nonos-mk-arm-gui-capsules: $(DESKTOP_BASE_SIGNED_ARTIFACTS) $(ZK_POLICY_ROOT)

nonos-mk-arm-gui: nonos-mk-check-deps nonos-mk-ensure-signing-key
	@$(MAKE) NONOS_USER_TARGET=aarch64-nonos-user \
		NONOS_ENROLLED_CAPSULES="$(DESKTOP_BASE_SLUGS)" nonos-mk-arm-gui-capsules
	@echo "Building kernel (aarch64, microkernel-desktop-base + nonos-stark-attest)..."
	@$(SDK_FLAGS) NONOS_USER_TARGET=aarch64-nonos-user \
		NONOS_SIGNING_KEY=$(KERNEL_SIGNING_KEY) \
		RUSTUP_TOOLCHAIN=$(TOOLCHAIN) PATH="$(HOME)/.cargo/bin:$$PATH" \
		$(CARGO) build $(ARM_KERNEL_BUILD_FLAGS) \
		--no-default-features \
		--features microkernel-desktop-base$(_boot_comma)nonos-arch-preview$(_boot_comma)nonos-stark-attest

# nonos-mk-zerostate: the canonical NONOS image. The whole ZeroState system in
# one build: every capsule and driver, the transparent STARK spawn gate
# enforced, dual Ed25519 + ML-DSA-65 signing, the anti-rollback index bound into
# the signature, and the TPM measured-boot path. This is what `make` builds and
# what ships; nonos-mk-desktop-gui-prod is the same recipe minus the
# real-hardware drivers, for booting under QEMU.
nonos-mk-zerostate: nonos-mk-all-capsules-attested \
		$(driver-iwlwifi_ARTIFACTS) $(driver-rtl8821ce_ARTIFACTS) \
		nonos-mk-verify-desktop-gui-capsules \
		nonos-mk-check-deps nonos-mk-ensure-signing-key
	$(call nonos_kernel_build,zerostate: microkernel-full-gui + nonos-stark-attest,microkernel-full-gui$(_boot_comma)nonos-stark-attest)

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

ifeq ($(NONOS_STARK_KERNEL_ATTEST),1)
# Kernel self-attestation: embed the transparent STARK trailer the bootloader
# verifies against the enrolled kernel root before jump, in place of the curve
# boot proof. The trailer is bound to the kernel measurement by nonos-stark-enroll.
$(TARGET_DIR)/kernel_attested.bin: $(TARGET_DIR)/kernel_signed.bin $(EMBED_TOOL) $(KERNEL_ATTEST_STAMP)
	@echo "Embedding kernel STARK self-attestation trailer..."
	@$(EMBED_TOOL) --input $< --output $@ --proof-file $(KERNEL_ATTEST_TRAILER) --verbose
else
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
endif

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
	@# Re-drive the whole attested-kernel chain at pack time. Recursive
	@# sub-makes relink the kernel after this target's prerequisites were
	@# resolved. If the ELF moved after its self-attestation was enrolled, the
	@# embedded trailer measures a kernel that no longer exists and the kernel
	@# refuses to boot. Deleting the enrollment forces it to bind the final
	@# ELF, then the sign and embed follow; re-statting here cannot be raced by
	@# an earlier resolution.
	@# Sequential re-drive. Recursive sub-makes may have relinked the kernel
	@# after this rule's prerequisites were resolved, so re-stat and re-run the
	@# chain in order: enrollment binds the final ELF, the bootloader bakes the
	@# root that enrollment produced, and the embed carries the trailer that
	@# matches both. One recipe, three ordered steps, no window in which a
	@# parallel job can observe a half-rebuilt chain; the old approach deleted
	@# the enrollment here, which shot any bootloader compile already in flight.
	@# Enrol, then rebuild the bootloader against the root that enrolment
	@# produced, then embed. Sequenced here rather than as a prerequisite of
	@# the bootloader: a bootloader built on its own, with the attest gate off
	@# or with no kernel to enrol, must not be made to demand an enrolment it
	@# has no way to perform.
ifeq ($(NONOS_STARK_KERNEL_ATTEST_ON),1)
	@$(MAKE) --no-print-directory nonos-mk-kernel-attest-ensure
	@rm -f $(BOOTLOADER_DIR)/target/x86_64-unknown-uefi/release/nonos_boot.efi
	@$(MAKE) --no-print-directory $(BOOTLOADER_DIR)/target/x86_64-unknown-uefi/release/nonos_boot.efi
endif
	@$(MAKE) --no-print-directory $(TARGET_DIR)/kernel_attested.bin
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
