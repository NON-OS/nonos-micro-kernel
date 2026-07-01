# NVMe — PCIe storage-controller capsule. This slice owns broker
# claim, BAR0 MMIO, MSI-X binding, admin and IO queue DMA,
# controller enable, Identify Controller, Identify Namespace,
# SMART / health snapshot, and read/write/flush block operations.

CAPSULE_SLUG             := driver-nvme
CAPSULE_HANDLE           := driver.nvme0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_nvme
CAPSULE_BIN_NAME         := driver_nvme
CAPSULE_FEATURE          := nonos-capsule-driver-nvme
CAPSULE_NAMESPACE        := systems.nonos.driver.nvme0
CAPSULE_SERVICE_ENDPOINT := service:4220:driver.nvme0
CAPSULE_REPLY_ENDPOINT   := reply:4221:endpoint.4294967313
# IPC|Memory|Driver|DeviceEnum|Mmio|Irq|Dma = 0xF8019
CAPSULE_REQUIRED_CAPS    := 0xF8019

include nonos-mk/capsule.mk
