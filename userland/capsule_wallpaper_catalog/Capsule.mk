# NONOS capsule_wallpaper_catalog
# Read-only asset vendor for the 63 Full-HD wallpaper JPEGs.

CAPSULE_SLUG             := wallpaper_catalog
CAPSULE_HANDLE           := wallpaper_catalog
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_wallpaper_catalog
CAPSULE_BIN_NAME         := wallpaper_catalog
CAPSULE_FEATURE          := nonos-capsule-wallpaper-catalog
CAPSULE_NAMESPACE        := systems.nonos.wallpaper_catalog
CAPSULE_SERVICE_ENDPOINT := service:4110:wallpaper_catalog
CAPSULE_REPLY_ENDPOINT   := reply:4111:endpoint.wallpaper_catalog.reply
CAPSULE_EXTRA_DEPS       := $(sort $(wildcard nonos-data/wallpapers/*.jpg))
# CoreExec | IPC | Memory = 0x01 | 0x08 | 0x10 = 0x19
CAPSULE_REQUIRED_CAPS    := 0x19
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_wallpaper_catalog

include nonos-mk/capsule.mk
