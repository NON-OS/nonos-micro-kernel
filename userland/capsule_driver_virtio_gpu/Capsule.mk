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
# CoreExec | IPC | Memory | GraphicsSurfaceCreate | DeviceEnum | Driver | Mmio | Irq | Dma | Pio
# = 0x01 | 0x08 | 0x10 | 0x1000 | 0x8000 | 0x10000 | 0x20000 | 0x40000 | 0x80000 | 0x100000 = 0x1F9019
CAPSULE_REQUIRED_CAPS    := 0x1F9019
CAPSULE_KERNEL_MIRROR    := src/hardware/virtio_gpu_capsule

include nonos-mk/capsule.mk
