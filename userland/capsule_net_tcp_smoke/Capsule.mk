# net_tcp_smoke — TCP lifecycle smoketest driver. Emits deterministic
# serial markers consumed by tests/boot/tcp_lifecycle.sh. Scaffold
# only; markers go green once Tasks 2-10 wire the full TCP state
# machine. IPC + Memory caps only — no network hardware access.

CAPSULE_SLUG             := net-tcp-smoke
CAPSULE_HANDLE           := net.tcp.smoke
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_net_tcp_smoke
CAPSULE_BIN_NAME         := net_tcp_smoke
CAPSULE_FEATURE          := nonos-capsule-net-tcp-smoke
CAPSULE_NAMESPACE        := systems.nonos.net.tcp.smoke
CAPSULE_SERVICE_ENDPOINT := service:4432:net.tcp.smoke
CAPSULE_REPLY_ENDPOINT   := reply:4433:endpoint.net.tcp.smoke.reply
# IPC | Memory | Debug = 0x119
CAPSULE_REQUIRED_CAPS    := 0x00119
CAPSULE_CARGO_FEATURES   := tcp-selftest
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_net_tcp_smoke

include nonos-mk/capsule.mk
