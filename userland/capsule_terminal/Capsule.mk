CAPSULE_SLUG             := terminal
CAPSULE_HANDLE           := app.terminal
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_terminal
CAPSULE_BIN_NAME         := terminal
CAPSULE_FEATURE          := nonos-capsule-terminal
CAPSULE_NAMESPACE        := systems.nonos.app.terminal
CAPSULE_SERVICE_ENDPOINT := service:4722:app.terminal
CAPSULE_REPLY_ENDPOINT   := reply:4723:endpoint.app.terminal.reply
# CoreExec|IPC|Memory|Debug|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1919
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_terminal

include nonos-mk/capsule.mk
