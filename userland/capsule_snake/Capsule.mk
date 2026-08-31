CAPSULE_SLUG             := snake
CAPSULE_HANDLE           := app.snake
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_snake
CAPSULE_BIN_NAME         := snake
CAPSULE_FEATURE          := nonos-capsule-snake
CAPSULE_NAMESPACE        := systems.nonos.app.snake
CAPSULE_SERVICE_ENDPOINT := service:4732:app.snake
CAPSULE_REPLY_ENDPOINT   := reply:4733:endpoint.app.snake.reply
CAPSULE_INSTANCE_ENDPOINTS := service:4850:app.snake.1 reply:4851:endpoint.app.snake.1.reply service:4852:app.snake.2 reply:4853:endpoint.app.snake.2.reply
# CoreExec|IPC|Memory|FileSystem|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1859
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_snake

include nonos-mk/capsule.mk
