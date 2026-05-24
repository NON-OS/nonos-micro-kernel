# RTL8169 — Realtek 8168/8169 gigabit NIC. PCI MMIO + INTx + DMA.
# Raw Ethernet frames only; network policy belongs to the net-stack
# capsule. Signing, certificate, and manifest rules come from the
# shared hybrid-signature capsule macro.

CAPSULE_SLUG             := driver-rtl8169
CAPSULE_HANDLE           := driver.rtl8169_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_rtl8169
CAPSULE_BIN_NAME         := driver_rtl8169
CAPSULE_FEATURE          := nonos-capsule-driver-rtl8169
CAPSULE_NAMESPACE        := systems.nonos.driver.rtl8169_0
CAPSULE_SERVICE_ENDPOINT := service:4214:driver.rtl8169_0
CAPSULE_REPLY_ENDPOINT   := reply:4215:endpoint.4294967310
# IPC|Memory|Driver|DeviceEnum|Mmio|Irq|Dma = 0xF8019
CAPSULE_REQUIRED_CAPS    := 0xF8019

include nonos-mk/capsule.mk
