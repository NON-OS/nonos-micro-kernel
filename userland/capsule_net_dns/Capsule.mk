# net_dns — DNS resolver. UDP client over `capsule_net_udp`. Caches
# answers, exposes the `net.dns` endpoint. IPC + Memory + Crypto + Network.

CAPSULE_SLUG             := net-dns
CAPSULE_HANDLE           := net.dns
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_dns
CAPSULE_BIN_NAME         := net_dns
CAPSULE_FEATURE          := nonos-capsule-net-dns
CAPSULE_NAMESPACE        := systems.nonos.net.dns
CAPSULE_SERVICE_ENDPOINT := service:4450:net.dns
CAPSULE_REPLY_ENDPOINT   := reply:4451:endpoint.net.dns.reply
CAPSULE_REQUIRED_CAPS    := 0x0003d
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_dns

include nonos-mk/capsule.mk
