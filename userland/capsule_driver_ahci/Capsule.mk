# AHCI — SATA host controller capsule. PCI MMIO + INTx. This
# first production slice owns controller discovery, broker claim,
# ABAR mapping, IRQ binding, AHCI-mode enable, and port signature
# enumeration. It does not expose block I/O until command-list,
# FIS, PRDT, and DMA completion paths land.

CAPSULE_SLUG             := driver-ahci
CAPSULE_HANDLE           := driver.ahci0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_ahci
CAPSULE_BIN_NAME         := driver_ahci
CAPSULE_FEATURE          := nonos-capsule-driver-ahci
CAPSULE_NAMESPACE        := systems.nonos.driver.ahci0
CAPSULE_SERVICE_ENDPOINT := service:4216:driver.ahci0
CAPSULE_REPLY_ENDPOINT   := reply:4217:endpoint.4294967311
# IPC|Memory|Driver|DeviceEnum|Mmio|Irq = 0x78019
CAPSULE_REQUIRED_CAPS    := 0x78019

include nonos-mk/capsule.mk
