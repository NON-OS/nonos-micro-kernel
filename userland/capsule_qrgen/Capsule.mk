CAPSULE_BUILD_STD        := std,panic_abort
CAPSULE_SLUG             := qrgen
CAPSULE_HANDLE           := qrgen
CAPSULE_DOMAIN           := com.example
CAPSULE_DIR              := userland/capsule_qrgen
CAPSULE_BIN_NAME         := qrgen
CAPSULE_NAMESPACE        := com.example.qrgen
CAPSULE_SERVICE_ENDPOINT := service:4924:app.qrgen
CAPSULE_REPLY_ENDPOINT   := reply:4925:endpoint.app.qrgen.reply
# CoreExec|IPC|Memory|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1819

include nonos-mk/capsule.mk
