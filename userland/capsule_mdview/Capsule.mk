CAPSULE_BUILD_STD        := std,panic_abort
CAPSULE_SLUG             := mdview
CAPSULE_HANDLE           := mdview
CAPSULE_DOMAIN           := com.example
CAPSULE_DIR              := userland/capsule_mdview
CAPSULE_BIN_NAME         := mdview
CAPSULE_NAMESPACE        := com.example.mdview
CAPSULE_SERVICE_ENDPOINT := service:4922:app.mdview
CAPSULE_REPLY_ENDPOINT   := reply:4923:endpoint.app.mdview.reply
# CoreExec|IPC|Memory|FileSystem|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1859

include nonos-mk/capsule.mk
