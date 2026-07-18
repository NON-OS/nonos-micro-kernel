# Bootable images built from the signed, attested kernel: the EFI System
# Partition, a raw USB image, and a UEFI ISO.

.PHONY: nonos-mk-usb-run

# EFI boot image, so it boots from a burned disc or QEMU with -cdrom. This is
# the distributable production image. For a USB stick use nonos-mk-usb-img
# instead: it writes a real GPT partition table, which is what firmware
# expects from a disk, where a plain El Torito ISO is not dependable.
NONOS_ISO ?= $(TARGET_DIR)/nonos.iso
nonos-mk-iso: nonos-mk-esp
	@echo "Building bootable UEFI ISO $(NONOS_ISO)..."
	@rm -rf $(TARGET_DIR)/isoroot $(TARGET_DIR)/efiboot.img
	@mkdir -p $(TARGET_DIR)/isoroot
	@sz=$$(( $$(du -sm $(ESP_DIR)/EFI | cut -f1) + 16 )); \
		dd if=/dev/zero of=$(TARGET_DIR)/efiboot.img bs=1048576 count=$$sz status=none 2>/dev/null \
		|| dd if=/dev/zero of=$(TARGET_DIR)/efiboot.img bs=1048576 count=$$sz 2>/dev/null
	@mformat -i $(TARGET_DIR)/efiboot.img -F ::
	@mcopy -i $(TARGET_DIR)/efiboot.img -s $(ESP_DIR)/EFI ::/EFI
	@cp -r $(ESP_DIR)/EFI $(TARGET_DIR)/isoroot/
	@cp $(TARGET_DIR)/efiboot.img $(TARGET_DIR)/isoroot/
	@xorriso -as mkisofs -R -J -V NONOS \
		-e efiboot.img -no-emul-boot \
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
