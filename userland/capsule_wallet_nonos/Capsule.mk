CAPSULE_SLUG             := wallet-nonos
CAPSULE_HANDLE           := app.nonos_wallet
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_wallet_nonos
CAPSULE_BIN_NAME         := wallet_nonos
CAPSULE_FEATURE          := nonos-capsule-wallet-nonos
CAPSULE_NAMESPACE        := systems.nonos.app.nonos_wallet
CAPSULE_SERVICE_ENDPOINT := service:4734:app.nonos_wallet
CAPSULE_REPLY_ENDPOINT   := reply:4735:endpoint.app.nonos_wallet.reply
CAPSULE_REQUIRED_CAPS    := 0x183d
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_wallet_nonos

include nonos-mk/capsule.mk
