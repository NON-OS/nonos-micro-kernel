# ps2_input — i8042 PS/2 keyboard. Pure PIO + IRQ; no MMIO, no
# DMA. Only driver in the verified set that uses `Pio`. Reply
# inbox label mirrors the kernel-side transport in
# `src/hardware/ps2_kbd_capsule/client/transport.rs`.

CAPSULE_SLUG             := driver-ps2-input
CAPSULE_HANDLE           := driver.ps2_kbd0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_ps2_input
CAPSULE_BIN_NAME         := driver_ps2_input
CAPSULE_FEATURE          := nonos-capsule-driver-ps2-input
CAPSULE_NAMESPACE        := systems.nonos.driver.ps2_kbd0
CAPSULE_SERVICE_ENDPOINT := service:4208:driver.ps2_kbd0
CAPSULE_REPLY_ENDPOINT   := reply:4209:endpoint.4294967306
# IPC|Memory|Driver|DeviceEnum|Irq|Pio
# = 0x08|0x10|0x10000|0x8000|0x40000|0x100000 = 0x158019
CAPSULE_REQUIRED_CAPS    := 0x158019
CAPSULE_KERNEL_MIRROR    := src/hardware/ps2_kbd_capsule

include nonos-mk/capsule.mk
