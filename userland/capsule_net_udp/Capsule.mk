# net_udp — UDP transport capsule. Stateless, IP-class consumer.
# Talks to `capsule_net_ip` to send and receive datagrams; serves
# the `net.udp` endpoint upstream. IPC + Memory + Network caps.

CAPSULE_SLUG             := net-udp
CAPSULE_HANDLE           := net.udp
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_udp
CAPSULE_BIN_NAME         := net_udp
CAPSULE_FEATURE          := nonos-capsule-net-udp
CAPSULE_NAMESPACE        := systems.nonos.net.udp
CAPSULE_SERVICE_ENDPOINT := service:4420:net.udp
CAPSULE_REPLY_ENDPOINT   := reply:4421:endpoint.net.udp.reply
CAPSULE_REQUIRED_CAPS    := 0x0001d
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_udp

include nonos-mk/capsule.mk
