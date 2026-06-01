CAPSULE_SLUG             := about
CAPSULE_HANDLE           := app.about
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_about
CAPSULE_BIN_NAME         := about
CAPSULE_FEATURE          := nonos-capsule-about
CAPSULE_NAMESPACE        := systems.nonos.app.about
CAPSULE_SERVICE_ENDPOINT := service:4710:app.about
CAPSULE_REPLY_ENDPOINT   := reply:4711:endpoint.app.about.reply
# CoreExec|IPC|Memory|Debug|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1919
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_about

include nonos-mk/capsule.mk
