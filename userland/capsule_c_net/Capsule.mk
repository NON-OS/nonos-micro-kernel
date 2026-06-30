CAPSULE_SLUG               := c-net
CAPSULE_HANDLE             := c_net
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_DIR                := userland/capsule_c_net
CAPSULE_BIN_NAME           := c_net
CAPSULE_FEATURE            := nonos-capsule-c-net
CAPSULE_NAMESPACE          := systems.nonos.c_net
CAPSULE_SERVICE_ENDPOINT   := service:4508:c_net
CAPSULE_REPLY_ENDPOINT     := reply:4509:endpoint.c_net.reply
# CoreExec | IPC | Memory | Debug = 0x01 | 0x08 | 0x10 | 0x100 = 0x119
# Debug is required for the write(1,...) -> MkDebug serial marker.
CAPSULE_REQUIRED_CAPS      := 0x119
CAPSULE_KERNEL_MIRROR      := src/userspace/capsule_c_net
CAPSULE_PREBUILT_BIN       := userland/capsule_c_net/build/c_net

include nonos-mk/capsule-c.mk
include nonos-mk/capsule.mk
