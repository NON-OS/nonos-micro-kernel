# installer — userland install authority. Receives a verified
# install request from the trusted market driver and, for paid
# listings, gates the install on a settled NOX receipt produced by
# capsule_payment. Holds no key material. Needs IPC for mk_ipc_* and
# Memory for the heap.

CAPSULE_SLUG             := installer
CAPSULE_HANDLE           := installer
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_installer
CAPSULE_BIN_NAME         := installer
CAPSULE_FEATURE          := nonos-capsule-installer
CAPSULE_NAMESPACE        := systems.nonos.installer
CAPSULE_SERVICE_ENDPOINT := service:4112:installer
CAPSULE_REPLY_ENDPOINT   := reply:4113:endpoint.4294967313
# IPC | Memory = 0x08 | 0x10 = 0x18
CAPSULE_REQUIRED_CAPS    := 0x18
CAPSULE_KERNEL_MIRROR    := src/security/installer_capsule

include nonos-mk/capsule.mk
