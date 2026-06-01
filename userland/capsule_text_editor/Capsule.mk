CAPSULE_SLUG             := text-editor
CAPSULE_HANDLE           := app.text_editor
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_text_editor
CAPSULE_BIN_NAME         := text_editor
CAPSULE_FEATURE          := nonos-capsule-text-editor
CAPSULE_NAMESPACE        := systems.nonos.app.text_editor
CAPSULE_SERVICE_ENDPOINT := service:4726:app.text_editor
CAPSULE_REPLY_ENDPOINT   := reply:4727:endpoint.app.text_editor.reply
# CoreExec|IPC|Memory|Debug|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1919
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_text_editor

include nonos-mk/capsule.mk
