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
# 0x1_0000_0018. The old id 4294967313 collided with the NVMe driver's
# reply inbox, so on profiles that carry both (full-gui) whichever spawned
# second lost its endpoint registration and never came up.
CAPSULE_REPLY_ENDPOINT   := reply:4113:endpoint.4294967320
# CoreExec | IPC | Memory | SpawnBroker = 0x01 | 0x08 | 0x10 | 0x400000 = 0x400019
# SpawnBroker lets this installer attribute a capsule it loads on a
# requester's behalf to that requester's pid instead of its own, so the
# requester can mk_wait/mk_kill/mk_proc_input/mk_proc_output the child.
CAPSULE_REQUIRED_CAPS    := 0x400019
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_installer

include nonos-mk/capsule.mk
