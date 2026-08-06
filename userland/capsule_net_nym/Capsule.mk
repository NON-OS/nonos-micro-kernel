# net_nym: mixnet privacy capsule above net.tcp. It owns gateway
# streams, cover traffic and userland crypto-backed packet sealing.

CAPSULE_SLUG             := net-nym
CAPSULE_HANDLE           := net.nym
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_nym
CAPSULE_BIN_NAME         := net_nym
CAPSULE_FEATURE          := nonos-capsule-net-nym
CAPSULE_NAMESPACE        := systems.nonos.net.nym
CAPSULE_SERVICE_ENDPOINT := service:4470:net.nym
CAPSULE_REPLY_ENDPOINT   := reply:4471:endpoint.net.nym.reply
CAPSULE_REQUIRED_CAPS    := 0x0013d
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_nym

include nonos-mk/capsule.mk
