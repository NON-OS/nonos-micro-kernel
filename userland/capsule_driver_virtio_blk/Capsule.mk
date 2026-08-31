# virtio_blk — virtio block device. PCI MMIO + INTx + DMA. The
# capsule maps three DMA regions (queue ring, request header,
# data buffer) and unwinds them on shutdown. Block IO transits
# IPC; no FS policy lives in the driver.

CAPSULE_SLUG             := driver-virtio-blk
CAPSULE_HANDLE           := driver.virtio_blk0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_virtio_blk
CAPSULE_BIN_NAME         := driver_virtio_blk
CAPSULE_FEATURE          := nonos-capsule-driver-virtio-blk
CAPSULE_NAMESPACE        := systems.nonos.driver.virtio_blk0
CAPSULE_SERVICE_ENDPOINT := service:4202:driver.virtio_blk0
CAPSULE_REPLY_ENDPOINT   := reply:4203:endpoint.4294967304
# IPC|Memory|Debug|Driver|DeviceEnum|Mmio|Irq|Dma|Pio = 0x1F8119
CAPSULE_REQUIRED_CAPS    := 0x1F8119
CAPSULE_KERNEL_MIRROR    := src/hardware/virtio_blk_capsule

include nonos-mk/capsule.mk
