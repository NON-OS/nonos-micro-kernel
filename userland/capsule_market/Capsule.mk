# market — userland service capsule for the marketplace index.
# Standard userland-service bundle: IPC for `mk_ipc_*`, Memory
# for the heap, and Crypto because the release-signature check calls
# crypto_ed25519_verify, whose syscall is gated on Capability::Crypto
# at the contract layer. Without it that verification is denied.

CAPSULE_SLUG             := market
CAPSULE_HANDLE           := market
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_market
CAPSULE_BIN_NAME         := market
CAPSULE_FEATURE          := nonos-capsule-market
CAPSULE_NAMESPACE        := systems.nonos.market
CAPSULE_SERVICE_ENDPOINT := service:4106:market.index
CAPSULE_REPLY_ENDPOINT   := reply:4107:endpoint.4294967303
# CoreExec | IPC | Memory | Crypto = 0x01 | 0x08 | 0x10 | 0x20 = 0x39
CAPSULE_REQUIRED_CAPS    := 0x39
CAPSULE_KERNEL_MIRROR    := src/security/market_capsule

include nonos-mk/capsule.mk
