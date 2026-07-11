CAPSULE_SLUG             := browser
CAPSULE_HANDLE           := app.browser
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_browser
CAPSULE_BIN_NAME         := browser
CAPSULE_FEATURE          := nonos-capsule-browser
CAPSULE_NAMESPACE        := systems.nonos.app.browser
CAPSULE_SERVICE_ENDPOINT := service:4760:app.browser
CAPSULE_REPLY_ENDPOINT   := reply:4761:endpoint.app.browser.reply
# Endpoints for up to three extra on-demand windows. Each pair is declared in
# the signed manifest so a runtime-spawned instance can register its own
# service and receive its own compositor window.
CAPSULE_INSTANCE_ENDPOINTS := \
	service:4762:app.browser.1 reply:4763:endpoint.app.browser.1.reply \
	service:4764:app.browser.2 reply:4765:endpoint.app.browser.2.reply \
	service:4766:app.browser.3 reply:4767:endpoint.app.browser.3.reply
# CoreExec|IPC|Memory|Crypto|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x183d
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_browser

include nonos-mk/capsule.mk
