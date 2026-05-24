# entropy — userland service capsule. The capsule is the entropy
# authority, so it never holds `Capability::Entropy`; callers
# carry that bit and reach the pool through IPC. The capsule
# itself needs IPC for `mk_ipc_*`, Memory for the heap, and
# Crypto for the hash + RNG primitives consumed inside the pool.

CAPSULE_SLUG             := entropy
CAPSULE_HANDLE           := entropy
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_entropy
CAPSULE_BIN_NAME         := entropy
CAPSULE_FEATURE          := nonos-capsule-entropy
CAPSULE_NAMESPACE        := systems.nonos.entropy
CAPSULE_SERVICE_ENDPOINT := service:4100:entropy_pool
CAPSULE_REPLY_ENDPOINT   := reply:4101:endpoint.4294967299
# IPC | Memory | Crypto = 0x08 | 0x10 | 0x20 = 0x39
CAPSULE_REQUIRED_CAPS    := 0x39
CAPSULE_KERNEL_MIRROR    := src/security/entropy_capsule

include nonos-mk/capsule.mk
