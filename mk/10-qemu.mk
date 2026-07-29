# QEMU and OVMF discovery, and the prerequisites every build shares: the pinned
# toolchain, the signing keys, and the transparent-STARK enrollment tools.

.PHONY: nonos-mk nonos-mk-check-deps nonos-mk-ensure-signing-key nonos-mk-toolchain

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
QEMU_GPU := -device virtio-vga,disable-modern=on,vectors=0,edid=on,xres=$(QEMU_XRES),yres=$(QEMU_YRES)
QEMU_DISPLAY := cocoa,zoom-to-fit=on
endif
# Keyboard/mouse via the q35 i8042 (PS/2). USB HID interrupt-IN transfers
# are not serviced under macOS hvf, so usb-kbd/usb-mouse never deliver input
# there; the xHCI controller stays for the USB stack/storage paths.
QEMU_USB := -device qemu-xhci,id=xhci
QEMU_RNG := -device virtio-rng-pci
QEMU_AUDIODEV ?= coreaudio
QEMU_AUDIO := -audiodev $(QEMU_AUDIODEV),id=snd0 -device intel-hda -device hda-duplex,audiodev=snd0

# Software TPM 2.0 for measured boot. The guest reaches it by direct MMIO at
# 0xFED40000 (tpm-crb), so no TCG2 firmware is needed; swtpm backs it over a
# unix socket.
SWTPM ?= swtpm
SWTPM_STATE ?= /tmp/nonos-swtpm
SWTPM_SOCK ?= $(SWTPM_STATE)/swtpm-sock
# Provisioning tool. A TPM with no endorsement key answers TPM_RC_HANDLE to
# every EK read, so without this the boot chain exercises measurement but never
# the hardware identity it is supposed to bind to. Real parts ship with an EK
# already in them; an emulated one has to be given one.
SWTPM_SETUP ?= swtpm_setup
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

# nonos-mk: build the microkernel-capsules runtime baseline.

nonos-mk: nonos-mk-capsules
	@echo
	@echo "Built microkernel-capsules ($(VERSION))."
	@echo "  make nonos-mk-esp           package the ESP for QEMU"
	@echo "  make nonos-mk-run           full OS under QEMU + OVMF + TPM + NAT"
	@echo "  make nonos-mk-dev-run       clean-clone: mint a dev identity then boot"
	@echo "  make nonos-mk-verify        static gates + symbol scan"
	@echo "  make nonos-mk-test          verify + the boot harnesses"

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
