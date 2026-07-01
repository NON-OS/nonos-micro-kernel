# net_tcp — TCP transport. Stateful, IP-class consumer. Talks to
# `capsule_net_ip` to send and receive segments; serves the
# `net.tcp` endpoint upstream. IPC + Memory + Crypto + Network caps.

CAPSULE_SLUG             := net-tcp
CAPSULE_HANDLE           := net.tcp
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_tcp
CAPSULE_BIN_NAME         := net_tcp
CAPSULE_FEATURE          := nonos-capsule-net-tcp
CAPSULE_NAMESPACE        := systems.nonos.net.tcp
CAPSULE_SERVICE_ENDPOINT := service:4430:net.tcp
CAPSULE_REPLY_ENDPOINT   := reply:4431:endpoint.net.tcp.reply
CAPSULE_REQUIRED_CAPS    := 0x0003d
CAPSULE_CARGO_FEATURES   :=
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_tcp

include nonos-mk/capsule.mk
