# net_sockets — POSIX-shape socket multiplexer. Routes per-handle
# operations to the right transport capsule (TCP / UDP) and the
# DNS resolver. IPC + Memory only; no kernel surface.

CAPSULE_SLUG             := net-sockets
CAPSULE_HANDLE           := net.sockets
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_sockets
CAPSULE_BIN_NAME         := net_sockets
CAPSULE_FEATURE          := nonos-capsule-net-sockets
CAPSULE_NAMESPACE        := systems.nonos.net.sockets
CAPSULE_SERVICE_ENDPOINT := service:4460:net.sockets
CAPSULE_REPLY_ENDPOINT   := reply:4461:endpoint.net.sockets.reply
CAPSULE_REQUIRED_CAPS    := 0x00019
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_sockets

include nonos-mk/capsule.mk
