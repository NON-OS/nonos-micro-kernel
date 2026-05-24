# crypto — userland service capsule. CAP_CRYPTO is the
# caller-facing gate; the capsule itself does not hold it.
# Needs IPC for `mk_ipc_*`, Memory for the heap, and Crypto to
# drive the primitives it serves.

CAPSULE_SLUG             := crypto
CAPSULE_HANDLE           := crypto
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_crypto
CAPSULE_BIN_NAME         := crypto
CAPSULE_FEATURE          := nonos-capsule-crypto
CAPSULE_NAMESPACE        := systems.nonos.crypto
CAPSULE_SERVICE_ENDPOINT := service:4102:crypto_pool
CAPSULE_REPLY_ENDPOINT   := reply:4103:endpoint.4294967300
# IPC | Memory | Crypto = 0x08 | 0x10 | 0x20 = 0x39
CAPSULE_REQUIRED_CAPS    := 0x39
CAPSULE_KERNEL_MIRROR    := src/security/crypto_capsule

include nonos-mk/capsule.mk
