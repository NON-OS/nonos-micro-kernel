CAPSULE_SLUG               := c-proof
CAPSULE_HANDLE             := c_proof
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_DIR                := userland/capsule_c_proof
CAPSULE_BIN_NAME           := c_proof
CAPSULE_FEATURE            := nonos-capsule-c-proof
CAPSULE_NAMESPACE          := systems.nonos.c_proof
CAPSULE_SERVICE_ENDPOINT   := service:4504:c_proof
CAPSULE_REPLY_ENDPOINT     := reply:4505:endpoint.c_proof.reply
# CoreExec | IPC | Memory = 0x01 | 0x08 | 0x10 = 0x19
CAPSULE_REQUIRED_CAPS      := 0x19
CAPSULE_KERNEL_MIRROR      := src/userspace/capsule_c_proof
CAPSULE_PREBUILT_BIN       := userland/capsule_c_proof/build/c_proof

include nonos-mk/capsule-c.mk
include nonos-mk/capsule.mk
