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

# Modular build: each concern lives in mk/*.mk, included in numeric order
# so immediate (:=) variables evaluate exactly as in the former monolith.
include $(sort $(wildcard mk/*.mk))
