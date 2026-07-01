# AHCI — SATA host controller capsule. PCI MMIO + INTx. This
# production slice owns controller discovery, broker claim, ABAR
# mapping, IRQ binding, AHCI-mode enable, port identity, command
# DMA, and read/write/flush block operations.

CAPSULE_SLUG             := driver-ahci
CAPSULE_HANDLE           := driver.ahci0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_ahci
CAPSULE_BIN_NAME         := driver_ahci
CAPSULE_FEATURE          := nonos-capsule-driver-ahci
CAPSULE_NAMESPACE        := systems.nonos.driver.ahci0
CAPSULE_SERVICE_ENDPOINT := service:4216:driver.ahci0
CAPSULE_REPLY_ENDPOINT   := reply:4217:endpoint.4294967311
# IPC|Memory|Driver|DeviceEnum|Mmio|Irq|Dma = 0xf8019
CAPSULE_REQUIRED_CAPS    := 0xf8019

include nonos-mk/capsule.mk
