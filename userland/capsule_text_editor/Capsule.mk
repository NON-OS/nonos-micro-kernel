CAPSULE_SLUG             := text-editor
CAPSULE_HANDLE           := app.text_editor
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_text_editor
CAPSULE_BIN_NAME         := text_editor
CAPSULE_FEATURE          := nonos-capsule-text-editor
CAPSULE_NAMESPACE        := systems.nonos.app.text_editor
CAPSULE_SERVICE_ENDPOINT := service:4726:app.text_editor
CAPSULE_REPLY_ENDPOINT   := reply:4727:endpoint.app.text_editor.reply
CAPSULE_INSTANCE_ENDPOINTS := service:4830:app.text_editor.1 reply:4831:endpoint.app.text_editor.1.reply service:4832:app.text_editor.2 reply:4833:endpoint.app.text_editor.2.reply
# CoreExec|IPC|Memory|FileSystem|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1859
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_text_editor

include nonos-mk/capsule.mk
