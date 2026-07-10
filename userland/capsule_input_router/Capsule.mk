# input_router capsule. Drains the kernel input ring, normalizes
# events, and dispatches to the focused subscriber. No driver
# claims; only IPC + the MkInputEvent* surface.

CAPSULE_SLUG             := input-router
CAPSULE_HANDLE           := input_router
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_input_router
CAPSULE_BIN_NAME         := input_router
CAPSULE_FEATURE          := nonos-capsule-input-router
CAPSULE_NAMESPACE        := systems.nonos.input_router
CAPSULE_SERVICE_ENDPOINT := service:4320:input_router
CAPSULE_REPLY_ENDPOINT   := reply:4321:endpoint.input_router.reply
# CoreExec | IPC | Memory | InputSource = 0x01 | 0x08 | 0x10 | 0x200000 = 0x200019
# InputSource is the consumer authority for the raw-input ring (drain/wait).
CAPSULE_REQUIRED_CAPS    := 0x200019
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_input_router

include nonos-mk/capsule.mk
