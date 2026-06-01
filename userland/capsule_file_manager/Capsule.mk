CAPSULE_SLUG             := file-manager
CAPSULE_HANDLE           := app.file_manager
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_file_manager
CAPSULE_BIN_NAME         := file_manager
CAPSULE_FEATURE          := nonos-capsule-file-manager
CAPSULE_NAMESPACE        := systems.nonos.app.file_manager
CAPSULE_SERVICE_ENDPOINT := service:4724:app.file_manager
CAPSULE_REPLY_ENDPOINT   := reply:4725:endpoint.app.file_manager.reply
# CoreExec|IPC|Memory|Debug|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1919
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_file_manager

include nonos-mk/capsule.mk
