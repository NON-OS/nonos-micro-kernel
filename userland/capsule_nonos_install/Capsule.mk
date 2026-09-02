# nonos_install capsule. Proof-carrying installer run from the live USB:
# composes the system on the target disk and hands back a root receipt.
# Console-only for now; the capset grows with each step that lands.

CAPSULE_SLUG             := nonos-install
CAPSULE_HANDLE           := app.nonos_install
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_nonos_install
CAPSULE_BIN_NAME         := nonos_install
CAPSULE_FEATURE          := nonos-capsule-nonos-install
CAPSULE_NAMESPACE        := systems.nonos.app.nonos_install
CAPSULE_SERVICE_ENDPOINT := service:4860:app.nonos_install
CAPSULE_REPLY_ENDPOINT   := reply:4861:endpoint.app.nonos_install.reply
# CoreExec | IO | IPC | Memory | DeviceEnum: exactly what the landed
# steps call. The capset grows only when a step that needs more lands.
CAPSULE_REQUIRED_CAPS    := 0x801B
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_nonos_install

include nonos-mk/capsule.mk
