# net_core — smoltcp-backed network core. Owns the virtio_net NIC via a
# phy::Device and serves the net.* services from one smoltcp Interface.
# Identity service `net.core`; real service names registered at runtime.

CAPSULE_SLUG             := net-core
CAPSULE_HANDLE           := net.core
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_core
CAPSULE_BIN_NAME         := net_core
CAPSULE_FEATURE          := nonos-capsule-net-core
CAPSULE_NAMESPACE        := systems.nonos.net.core
CAPSULE_SERVICE_ENDPOINT := service:4480:net.core
CAPSULE_REPLY_ENDPOINT   := reply:4481:endpoint.net.core.reply
# IPC|Memory|Crypto|Network
CAPSULE_REQUIRED_CAPS    := 0x0003d
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_core

include nonos-mk/capsule.mk
