CAPSULE_SLUG             := image-viewer
CAPSULE_HANDLE           := image_viewer
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_image_viewer
CAPSULE_BIN_NAME         := image_viewer
CAPSULE_FEATURE          := nonos-capsule-image-viewer
CAPSULE_NAMESPACE        := systems.nonos.image_viewer
CAPSULE_SERVICE_ENDPOINT := service:4736:app.image_viewer
CAPSULE_REPLY_ENDPOINT   := reply:4737:endpoint.app.image_viewer.reply
# CoreExec|IPC|Memory|Debug|GraphicsDisplayQuery|GraphicsSurfaceCreate|GraphicsSurfaceMap
CAPSULE_REQUIRED_CAPS    := 0x3919
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_image_viewer

include nonos-mk/capsule.mk
