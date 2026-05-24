# RTL8139 — Realtek 8139 Fast Ethernet NIC. PCI PIO + INTx + DMA.
# Frame-level transport only: no socket, routing, ARP, or IP policy.
# Signing, certificate, manifest, and trust-anchor flow are inherited
# from nonos-mk/capsule.mk.

CAPSULE_SLUG             := driver-rtl8139
CAPSULE_HANDLE           := driver.rtl8139_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_rtl8139
CAPSULE_BIN_NAME         := driver_rtl8139
CAPSULE_FEATURE          := nonos-capsule-driver-rtl8139
CAPSULE_NAMESPACE        := systems.nonos.driver.rtl8139_0
CAPSULE_SERVICE_ENDPOINT := service:4212:driver.rtl8139_0
CAPSULE_REPLY_ENDPOINT   := reply:4213:endpoint.4294967309
# IPC|Memory|Driver|DeviceEnum|Irq|Dma|Pio = 0x1D8019
CAPSULE_REQUIRED_CAPS    := 0x1D8019

include nonos-mk/capsule.mk
