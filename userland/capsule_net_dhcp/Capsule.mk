# net_dhcp — DHCPv4 client. Talks to `capsule_net_udp` for the
# bootp transport, drives the DISCOVER/OFFER/REQUEST/ACK ladder,
# installs the lease into `capsule_net_ip` once acknowledged.

CAPSULE_SLUG             := net-dhcp
CAPSULE_HANDLE           := net.dhcp.client
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_dhcp
CAPSULE_BIN_NAME         := net_dhcp
CAPSULE_FEATURE          := nonos-capsule-net-dhcp
CAPSULE_NAMESPACE        := systems.nonos.net.dhcp.client
CAPSULE_SERVICE_ENDPOINT := service:4440:net.dhcp.client
CAPSULE_REPLY_ENDPOINT   := reply:4441:endpoint.net.dhcp.client.reply
CAPSULE_REQUIRED_CAPS    := 0x0003d
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_dhcp

include nonos-mk/capsule.mk
