# net_ip — IPv4 network layer. Talks to `capsule_net_l2` to send
# and receive ethernet payloads; exposes the `net.ip` endpoint to
# upstream transport capsules (UDP/TCP) and the ICMP responder.
# IPC + Memory + Network caps; the NIC-side authority lives one layer
# down at L2 and is invisible here.

CAPSULE_SLUG             := net-ip
CAPSULE_HANDLE           := net.ip
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_ip
CAPSULE_BIN_NAME         := net_ip
CAPSULE_FEATURE          := nonos-capsule-net-ip
CAPSULE_NAMESPACE        := systems.nonos.net.ip
CAPSULE_SERVICE_ENDPOINT := service:4402:net.ip
CAPSULE_REPLY_ENDPOINT   := reply:4403:endpoint.net.ip.reply
# IPC|Memory|Network
CAPSULE_REQUIRED_CAPS    := 0x0001d
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_ip

include nonos-mk/capsule.mk
