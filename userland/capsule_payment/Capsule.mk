# payment — userland service capsule. Issues signed NOX install
# receipts. Holds no key material: it assembles receipt fields and
# calls capsule_keyring over IPC to sign. Needs IPC for mk_ipc_* and
# Memory for the heap; it never drives a kernel crypto syscall
# directly (signing happens inside the keyring).

CAPSULE_SLUG             := payment
CAPSULE_HANDLE           := payment
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_payment
CAPSULE_BIN_NAME         := payment
CAPSULE_FEATURE          := nonos-capsule-payment
CAPSULE_NAMESPACE        := systems.nonos.payment
CAPSULE_SERVICE_ENDPOINT := service:4110:payment
CAPSULE_REPLY_ENDPOINT   := reply:4111:endpoint.4294967312
# CoreExec | IPC | Memory = 0x01 | 0x08 | 0x10 = 0x19
CAPSULE_REQUIRED_CAPS    := 0x19
CAPSULE_KERNEL_MIRROR    := src/security/payment_capsule

include nonos-mk/capsule.mk
