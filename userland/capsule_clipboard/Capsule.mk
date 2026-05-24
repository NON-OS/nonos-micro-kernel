CAPSULE_SLUG             := clipboard
CAPSULE_HANDLE           := clipboard
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_clipboard
CAPSULE_BIN_NAME         := clipboard
CAPSULE_FEATURE          := nonos-capsule-clipboard
CAPSULE_NAMESPACE        := systems.nonos.clipboard
CAPSULE_SERVICE_ENDPOINT := service:4414:clipboard
CAPSULE_REPLY_ENDPOINT   := reply:4415:endpoint.clipboard.reply
# CoreExec | IPC | Memory = 0x01 | 0x08 | 0x10 = 0x19
# Debug deliberately absent: capsule_clipboard emits no MkDebug markers.
# The NO LOGS / NO TRACES posture refuses the serial surface.
CAPSULE_REQUIRED_CAPS    := 0x19
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_clipboard

include nonos-mk/capsule.mk
