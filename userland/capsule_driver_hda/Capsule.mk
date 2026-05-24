# Intel HDA — HD-Audio controller capsule. PCI MMIO + INTx. This
# first production slice owns controller discovery, broker claim,
# BAR0 mapping, IRQ binding, controller reset release, and codec
# presence reporting. Playback/recording wait for real CORB/RIRB,
# BDL, and stream-DMA paths.

CAPSULE_SLUG             := driver-hda
CAPSULE_HANDLE           := driver.hda0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_hda
CAPSULE_BIN_NAME         := driver_hda
CAPSULE_FEATURE          := nonos-capsule-driver-hda
CAPSULE_NAMESPACE        := systems.nonos.driver.hda0
CAPSULE_SERVICE_ENDPOINT := service:4218:driver.hda0
CAPSULE_REPLY_ENDPOINT   := reply:4219:endpoint.4294967312
# IPC|Memory|Driver|DeviceEnum|Mmio|Irq = 0x78019
CAPSULE_REQUIRED_CAPS    := 0x78019

include nonos-mk/capsule.mk
