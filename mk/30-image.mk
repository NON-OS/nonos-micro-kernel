# Bootable images built from the signed, attested kernel: the EFI System
# Partition, a raw USB image, and a UEFI ISO.

.PHONY: nonos-mk-usb-run

# EFI boot image, so it boots from a burned disc or QEMU with -cdrom. This is
# the distributable production image. For a USB stick use nonos-mk-usb-img
# instead: it writes a real GPT partition table, which is what firmware
# expects from a disk, where a plain El Torito ISO is not dependable.
# Every timestamp that reaches an image is pinned. Left alone, the FAT volume
# serial, the file modification times and the ISO volume descriptor all carry
# the wall clock, so two builds of one commit differ in hash while being
# byte-identical in content. The download page states these builds are
# reproducible, and that has to be true of the artefact and not just the source.
# Override NONOS_IMAGE_DATE only to reproduce an older image.
NONOS_IMAGE_DATE      ?= 2026010100000000
NONOS_IMAGE_TOUCH     ?= 202601010000.00
NONOS_IMAGE_FAT_SERIAL ?= 4e4f4e4f

NONOS_ISO ?= $(TARGET_DIR)/nonos.iso
nonos-mk-iso: nonos-mk-esp
	@echo "Building bootable UEFI ISO $(NONOS_ISO)..."
	@rm -rf $(TARGET_DIR)/isoroot $(TARGET_DIR)/efiboot.img
	@mkdir -p $(TARGET_DIR)/isoroot
	@sz=$$(( $$(du -sm $(ESP_DIR)/EFI | cut -f1) + 16 )); \
		dd if=/dev/zero of=$(TARGET_DIR)/efiboot.img bs=1048576 count=$$sz status=none 2>/dev/null \
		|| dd if=/dev/zero of=$(TARGET_DIR)/efiboot.img bs=1048576 count=$$sz 2>/dev/null
	@cp -r $(ESP_DIR)/EFI $(TARGET_DIR)/isoroot/
	@find $(TARGET_DIR)/isoroot -exec touch -t $(NONOS_IMAGE_TOUCH) {} +
	@mformat -i $(TARGET_DIR)/efiboot.img -N $(NONOS_IMAGE_FAT_SERIAL) -F ::
	@mcopy -i $(TARGET_DIR)/efiboot.img -s $(TARGET_DIR)/isoroot/EFI ::/EFI
	@cp $(TARGET_DIR)/efiboot.img $(TARGET_DIR)/isoroot/
	@find $(TARGET_DIR)/isoroot -exec touch -t $(NONOS_IMAGE_TOUCH) {} +
	@xorriso -as mkisofs -R -J -V NONOS \
		-e efiboot.img -no-emul-boot \
		--modification-date=$(NONOS_IMAGE_DATE) \
		-o $(NONOS_ISO) $(TARGET_DIR)/isoroot >/dev/null 2>&1
	@echo "ISO ready at $(NONOS_ISO)"
	@echo "  Boot in QEMU:  qemu-system-x86_64 -bios OVMF.fd -cdrom $(NONOS_ISO)"
	@echo "  USB stick:     make nonos-mk-usb-img   (real GPT image for dd)"

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
