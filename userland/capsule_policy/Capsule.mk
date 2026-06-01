# NONOS capsule_policy — system policy store.
# RAM-ephemeral live settings, IPC surface for capsule readers /
# writers, MkAdminPolicyPush to push kernel-mirrored fields.

CAPSULE_SLUG             := policy
CAPSULE_HANDLE           := policy
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_policy
CAPSULE_BIN_NAME         := policy
CAPSULE_FEATURE          := nonos-capsule-policy
CAPSULE_NAMESPACE        := systems.nonos.policy
CAPSULE_SERVICE_ENDPOINT := service:4108:policy
CAPSULE_REPLY_ENDPOINT   := reply:4109:endpoint.policy.reply
# CoreExec | IPC | Memory | Admin = 0x01 | 0x08 | 0x10 | 0x200 = 0x219
CAPSULE_REQUIRED_CAPS    := 0x219
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_policy

include nonos-mk/capsule.mk
