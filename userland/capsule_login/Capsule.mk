CAPSULE_SLUG             := login
CAPSULE_HANDLE           := login
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_login
CAPSULE_BIN_NAME         := login
CAPSULE_FEATURE          := nonos-capsule-login
CAPSULE_NAMESPACE        := systems.nonos.login
CAPSULE_SERVICE_ENDPOINT := service:4416:login
CAPSULE_REPLY_ENDPOINT   := reply:4417:endpoint.login.reply
# IPC | Memory = 0x08 | 0x10 = 0x19
CAPSULE_REQUIRED_CAPS    := 0x19
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_login

include nonos-mk/capsule.mk
