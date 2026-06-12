CAPSULE_SLUG             := snake
CAPSULE_HANDLE           := app.snake
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_snake
CAPSULE_BIN_NAME         := snake
CAPSULE_FEATURE          := nonos-capsule-snake
CAPSULE_NAMESPACE        := systems.nonos.app.snake
CAPSULE_SERVICE_ENDPOINT := service:4732:app.snake
CAPSULE_REPLY_ENDPOINT   := reply:4733:endpoint.app.snake.reply
# CoreExec|IPC|Memory|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1819
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_snake

include nonos-mk/capsule.mk
