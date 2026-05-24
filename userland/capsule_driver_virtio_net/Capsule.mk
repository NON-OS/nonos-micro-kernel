# virtio_net — virtio network device. PCI MMIO + INTx + DMA with
# separate RX and TX rings (four DMA grants total). Frame-level
# transport over IPC; no socket or routing policy. `Network` cap
# is intentionally absent — that authority belongs to a future
# net-stack capsule built on top of this driver.

CAPSULE_SLUG             := driver-virtio-net
CAPSULE_HANDLE           := driver.virtio_net0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_virtio_net
CAPSULE_BIN_NAME         := driver_virtio_net
CAPSULE_FEATURE          := nonos-capsule-driver-virtio-net
CAPSULE_NAMESPACE        := systems.nonos.driver.virtio_net0
CAPSULE_SERVICE_ENDPOINT := service:4204:driver.virtio_net0
CAPSULE_REPLY_ENDPOINT   := reply:4205:endpoint.4294967305
# IPC|Memory|Driver|DeviceEnum|Mmio|Irq|Dma = 0xF8019
CAPSULE_REQUIRED_CAPS    := 0xF8019
CAPSULE_KERNEL_MIRROR    := src/hardware/virtio_net_capsule

include nonos-mk/capsule.mk
