# net_l2 — ethernet + ARP capsule. Talks to a NIC driver capsule
# over IPC (no hardware authority of its own) and serves the
# `net.l2` endpoint to upstream IP-stack capsules. No socket
# policy, no IP, no transport.

CAPSULE_SLUG             := net-l2
CAPSULE_HANDLE           := net.l2
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_l2
CAPSULE_BIN_NAME         := net_l2
CAPSULE_FEATURE          := nonos-capsule-net-l2
CAPSULE_NAMESPACE        := systems.nonos.net.l2
CAPSULE_SERVICE_ENDPOINT := service:4400:net.l2
CAPSULE_REPLY_ENDPOINT   := reply:4401:endpoint.net.l2.reply
# IPC|Memory — no hardware caps; the NIC capsule owns those.
CAPSULE_REQUIRED_CAPS    := 0x00019
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_l2

include nonos-mk/capsule.mk
