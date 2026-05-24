# xhci — USB 3 host controller. PCI MMIO + INTx + DMA. xHCI is
# MMIO-only (no PIO). The capsule owns DCBAA, scratchpad,
# command ring, event ring + ERST. INTx interrupt model;
# MSI/MSI-X land later behind a separate broker work item.

CAPSULE_SLUG             := driver-xhci
CAPSULE_HANDLE           := driver.xhci0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_xhci
CAPSULE_BIN_NAME         := driver_xhci
CAPSULE_FEATURE          := nonos-capsule-driver-xhci
CAPSULE_NAMESPACE        := systems.nonos.driver.xhci0
CAPSULE_SERVICE_ENDPOINT := service:4206:driver.xhci0
CAPSULE_REPLY_ENDPOINT   := reply:4207:endpoint.4294967307
# IPC|Memory|Driver|DeviceEnum|Mmio|Irq|Dma = 0xF8019
CAPSULE_REQUIRED_CAPS    := 0xF8019
CAPSULE_KERNEL_MIRROR    := src/hardware/xhci_capsule

include nonos-mk/capsule.mk
