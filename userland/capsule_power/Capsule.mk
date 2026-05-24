CAPSULE_SLUG             := power
CAPSULE_HANDLE           := power
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_power
CAPSULE_BIN_NAME         := power
CAPSULE_FEATURE          := nonos-capsule-power
CAPSULE_NAMESPACE        := systems.nonos.power
CAPSULE_SERVICE_ENDPOINT := service:4448:power
CAPSULE_REPLY_ENDPOINT   := reply:4449:endpoint.power.reply
# CoreExec | IPC | Memory | Admin = 0x01 | 0x08 | 0x10 | 0x200 = 0x219
# Admin gates AdminReboot + AdminShutdown.
# Debug deliberately absent: power_off must never leak to serial.
CAPSULE_REQUIRED_CAPS    := 0x219
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_power

include nonos-mk/capsule.mk
