CAPSULE_SLUG             := gui_demo
CAPSULE_HANDLE           := gui_demo
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_gui_demo
CAPSULE_BIN_NAME         := gui_demo
CAPSULE_NAMESPACE        := systems.nonos.gui_demo
CAPSULE_SERVICE_ENDPOINT := service:4916:app.gui_demo
CAPSULE_REPLY_ENDPOINT   := reply:4917:endpoint.app.gui_demo.reply
CAPSULE_REQUIRED_CAPS    := 0x1819

include nonos-mk/capsule.mk
