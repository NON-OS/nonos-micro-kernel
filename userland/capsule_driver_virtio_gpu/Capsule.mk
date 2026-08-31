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
# Debug, for the probe_3d and EDID serial lines only. Optional so the grant
# decides: install_caps ORs required in unconditionally, but masks optional
# against what the kernel grants, and a build without capsule-serial-debug
# grants nothing. Required and optional must stay disjoint (OverlappingCaps).
CAPSULE_OPTIONAL_CAPS    := 0x100
# The ceiling defaults to the required set alone, which would put the optional
# bit outside the id-cert and fail the manifest signer. It has to span both.
CAPSULE_CAPS_CEILING     := 0x1F9119
CAPSULE_KERNEL_MIRROR    := src/hardware/virtio_gpu_capsule

include nonos-mk/capsule.mk
