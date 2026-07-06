# net_ntp — SNTP time-sync client. UDP client over `capsule_net_udp`.
# Validates replies and adjusts the kernel clock via MkTimeAdjust.

CAPSULE_SLUG             := net-ntp
CAPSULE_HANDLE           := net.ntp.client
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_ntp
CAPSULE_BIN_NAME         := net_ntp
CAPSULE_FEATURE          := nonos-capsule-net-ntp
CAPSULE_NAMESPACE        := systems.nonos.net.ntp.client
CAPSULE_SERVICE_ENDPOINT := service:4482:net.ntp.client
CAPSULE_REPLY_ENDPOINT   := reply:4483:endpoint.net.ntp.client.reply
CAPSULE_REQUIRED_CAPS    := 0x40011D
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_ntp

include nonos-mk/capsule.mk
