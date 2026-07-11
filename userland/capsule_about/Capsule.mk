CAPSULE_SLUG             := about
CAPSULE_HANDLE           := app.about
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_about
CAPSULE_BIN_NAME         := about
CAPSULE_FEATURE          := nonos-capsule-about
CAPSULE_NAMESPACE        := systems.nonos.app.about
CAPSULE_SERVICE_ENDPOINT := service:4710:app.about
CAPSULE_REPLY_ENDPOINT   := reply:4711:endpoint.app.about.reply
CAPSULE_INSTANCE_ENDPOINTS := service:4846:app.about.1 reply:4847:endpoint.app.about.1.reply service:4848:app.about.2 reply:4849:endpoint.app.about.2.reply
# CoreExec|IPC|Memory|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1819
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_about

include nonos-mk/capsule.mk
