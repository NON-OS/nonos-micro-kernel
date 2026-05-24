CAPSULE_SLUG             := attest
CAPSULE_HANDLE           := attest
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_attest
CAPSULE_BIN_NAME         := attest
CAPSULE_FEATURE          := nonos-capsule-attest
CAPSULE_NAMESPACE        := systems.nonos.attest
CAPSULE_SERVICE_ENDPOINT := service:4444:attest
CAPSULE_REPLY_ENDPOINT   := reply:4445:endpoint.attest.reply
# CoreExec | IPC | Memory = 0x01 | 0x08 | 0x10 = 0x19
# Debug deliberately absent: capsule_attest would lose all credibility
# if it emitted MkDebug markers. The NO LOGS posture is the point.
CAPSULE_REQUIRED_CAPS    := 0x19
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_attest

include nonos-mk/capsule.mk
