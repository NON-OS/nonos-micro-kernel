CAPSULE_SLUG             := browser
CAPSULE_HANDLE           := app.browser
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_browser
CAPSULE_BIN_NAME         := browser
CAPSULE_FEATURE          := nonos-capsule-browser
CAPSULE_NAMESPACE        := systems.nonos.app.browser
CAPSULE_SERVICE_ENDPOINT := service:4760:app.browser
CAPSULE_REPLY_ENDPOINT   := reply:4761:endpoint.app.browser.reply
# CoreExec|IPC|Memory|Crypto|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x183d
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_browser

include nonos-mk/capsule.mk
