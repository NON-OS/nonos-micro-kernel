CAPSULE_SLUG             := toolkit
CAPSULE_HANDLE           := toolkit
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/toolkit
CAPSULE_BIN_NAME         := toolkit
CAPSULE_FEATURE          := nonos-capsule-toolkit
CAPSULE_NAMESPACE        := systems.nonos.toolkit
CAPSULE_SERVICE_ENDPOINT := service:4610:toolkit
CAPSULE_REPLY_ENDPOINT   := reply:4611:endpoint.toolkit.reply
# CoreExec|IPC|Memory|Debug
CAPSULE_REQUIRED_CAPS    := 0x119
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_toolkit

include nonos-mk/capsule.mk
