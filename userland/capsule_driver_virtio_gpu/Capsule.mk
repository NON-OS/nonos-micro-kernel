# virtio_gpu — virtio display controller. PCI MMIO/PIO + INTx + DMA.
# The capsule owns device initialization and the control queue; UI,
# compositor policy, surfaces, and focus stay outside the driver.

CAPSULE_SLUG             := driver-virtio-gpu
CAPSULE_HANDLE           := driver.virtio_gpu0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_virtio_gpu
CAPSULE_BIN_NAME         := driver_virtio_gpu
CAPSULE_FEATURE          := nonos-capsule-driver-virtio-gpu
CAPSULE_NAMESPACE        := systems.nonos.driver.virtio_gpu0
CAPSULE_SERVICE_ENDPOINT := service:4226:driver.virtio_gpu0
CAPSULE_REPLY_ENDPOINT   := reply:4227:endpoint.4294967316
# IPC|Memory|Debug|Driver|DeviceEnum|Mmio|Irq|Dma|Pio = 0x1F8118
CAPSULE_REQUIRED_CAPS    := 0x1FB118
CAPSULE_KERNEL_MIRROR    := src/hardware/virtio_gpu_capsule

include nonos-mk/capsule.mk
