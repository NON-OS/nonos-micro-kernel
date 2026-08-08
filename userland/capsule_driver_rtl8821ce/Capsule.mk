# RTL8821CE — Realtek 8821CE PCIe Wi-Fi. Hardware authority is brokered
# DeviceClaim/MMIO/IRQ/DMA only. 802.11/WPA2 policy and the net stack stay
# in the shared wifi core and the upper network capsules. Signing,
# certificate, and manifest rules come from the shared capsule macro.

CAPSULE_SLUG             := driver-rtl8821ce
CAPSULE_HANDLE           := driver.rtl8821ce0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_rtl8821ce
CAPSULE_BIN_NAME         := driver_rtl8821ce
CAPSULE_FEATURE          := nonos-capsule-driver-rtl8821ce
CAPSULE_NAMESPACE        := systems.nonos.driver.rtl8821ce0
CAPSULE_SERVICE_ENDPOINT := service:4234:driver.rtl8821ce0
CAPSULE_REPLY_ENDPOINT   := reply:4235:endpoint.4294967320
# IPC|Memory|Crypto|Driver|DeviceEnum|Mmio|Irq|Dma|Debug = 0xF8139
# Debug (0x100) is the upper bound so a serial-debug kernel can grant it and the
# radio bring-up reports its progress; a hardened build grants a subset without
# it and the driver still spawns.
# Crypto (0x20) is what the CryptoRandom syscall is gated on. The station address
# is drawn rather than read out of the efuse, so without this the draw returns
# nothing and the PHY is left unconfigured with the radio down.
CAPSULE_REQUIRED_CAPS    := 0xF8139
CAPSULE_KERNEL_MIRROR    := src/hardware/rtl8821ce_capsule

include nonos-mk/capsule.mk
