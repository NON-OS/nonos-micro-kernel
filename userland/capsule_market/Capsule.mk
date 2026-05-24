# market — userland service capsule for the marketplace index.
# Standard userland-service bundle: IPC for `mk_ipc_*`, Memory
# for the heap. Crypto math runs through the kernel-routed
# `crypto_capsule` syscall path, so no Crypto cap is needed; the
# crypto authority lives behind that boundary.

CAPSULE_SLUG             := market
CAPSULE_HANDLE           := market
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_market
CAPSULE_BIN_NAME         := market
CAPSULE_FEATURE          := nonos-capsule-market
CAPSULE_NAMESPACE        := systems.nonos.market
CAPSULE_SERVICE_ENDPOINT := service:4106:market.index
CAPSULE_REPLY_ENDPOINT   := reply:4107:endpoint.4294967303
# IPC | Memory = 0x08 | 0x10 = 0x19
CAPSULE_REQUIRED_CAPS    := 0x19
CAPSULE_KERNEL_MIRROR    := src/security/market_capsule

include nonos-mk/capsule.mk
