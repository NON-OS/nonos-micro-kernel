CAPSULE_SLUG             := clock
CAPSULE_HANDLE           := app.clock
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_clock
CAPSULE_BIN_NAME         := clock
CAPSULE_FEATURE          := nonos-capsule-clock
CAPSULE_NAMESPACE        := systems.nonos.app.clock
CAPSULE_SERVICE_ENDPOINT := service:4730:app.clock
CAPSULE_REPLY_ENDPOINT   := reply:4731:endpoint.app.clock.reply
CAPSULE_INSTANCE_ENDPOINTS := service:4842:app.clock.1 reply:4843:endpoint.app.clock.1.reply service:4844:app.clock.2 reply:4845:endpoint.app.clock.2.reply
# CoreExec|IPC|Memory|GraphicsDisplayQuery|GraphicsSurfaceCreate|TimeSet
CAPSULE_REQUIRED_CAPS    := 0x401819
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_clock

include nonos-mk/capsule.mk
