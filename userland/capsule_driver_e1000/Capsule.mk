# e1000 — Intel 8254x gigabit NIC. PCI MMIO + INTx + DMA with
# separate RX and TX rings (four DMA grants total). Frame-level
# transport over IPC; no socket or routing policy. `Network` cap
# is intentionally absent — that authority belongs to a future
# net-stack capsule built on top of this driver. Wire shape
# matches `driver-virtio-net` so a single net-stack client can
# drive either backend.

CAPSULE_SLUG             := driver-e1000
CAPSULE_HANDLE           := driver.e1000_0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_e1000
CAPSULE_BIN_NAME         := driver_e1000
CAPSULE_FEATURE          := nonos-capsule-driver-e1000
CAPSULE_NAMESPACE        := systems.nonos.driver.e1000_0
CAPSULE_SERVICE_ENDPOINT := service:4210:driver.e1000_0
CAPSULE_REPLY_ENDPOINT   := reply:4211:endpoint.4294967308
# IPC|Memory|Crypto|Driver|DeviceEnum|Mmio|Irq|Dma = 0xF8039
# Crypto (0x20) is what the CryptoRandom syscall is gated on. The station address
# is drawn rather than read out of the EEPROM, and that draw fails closed, so
# without this the card has no address to transmit under.
CAPSULE_REQUIRED_CAPS    := 0xF8039
CAPSULE_KERNEL_MIRROR    := src/hardware/e1000_capsule

include nonos-mk/capsule.mk
