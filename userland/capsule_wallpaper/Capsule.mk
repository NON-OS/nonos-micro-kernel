CAPSULE_SLUG             := wallpaper
CAPSULE_HANDLE           := wallpaper
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_wallpaper
CAPSULE_BIN_NAME         := wallpaper
CAPSULE_FEATURE          := nonos-capsule-wallpaper
CAPSULE_NAMESPACE        := systems.nonos.wallpaper
CAPSULE_SERVICE_ENDPOINT := service:4340:wallpaper
CAPSULE_REPLY_ENDPOINT   := reply:4341:endpoint.wallpaper.reply
# CoreExec | IPC | Memory | GraphicsDisplayQuery | GraphicsSurfaceCreate
# = 0x01 | 0x08 | 0x10 | 0x800 | 0x1000 = 0x1819
CAPSULE_REQUIRED_CAPS    := 0x1819
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_wallpaper

include nonos-mk/capsule.mk
