CAPSULE_SLUG             := settings
CAPSULE_HANDLE           := app.settings
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_settings
CAPSULE_BIN_NAME         := settings
CAPSULE_FEATURE          := nonos-capsule-settings
CAPSULE_NAMESPACE        := systems.nonos.app.settings
CAPSULE_SERVICE_ENDPOINT := service:4728:app.settings
CAPSULE_REPLY_ENDPOINT   := reply:4729:endpoint.app.settings.reply
# CoreExec|IPC|Memory|Debug|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1919
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_settings

include nonos-mk/capsule.mk
