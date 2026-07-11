CAPSULE_SLUG             := process-manager
CAPSULE_HANDLE           := app.process_manager
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_process_manager
CAPSULE_BIN_NAME         := process_manager
CAPSULE_FEATURE          := nonos-capsule-process-manager
CAPSULE_NAMESPACE        := systems.nonos.app.process_manager
CAPSULE_SERVICE_ENDPOINT := service:4736:app.process_manager
CAPSULE_REPLY_ENDPOINT   := reply:4737:endpoint.app.process_manager.reply
CAPSULE_INSTANCE_ENDPOINTS := service:4862:app.process_manager.1 reply:4863:endpoint.app.process_manager.1.reply service:4864:app.process_manager.2 reply:4865:endpoint.app.process_manager.2.reply
# CoreExec|IPC|Memory|GraphicsDisplayQuery|GraphicsSurfaceCreate|ProcessControl
# = 0x1 | 0x8 | 0x10 | 0x800 | 0x1000 | 0x2000000 = 0x2001819
CAPSULE_REQUIRED_CAPS    := 0x2001819
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_process_manager

include nonos-mk/capsule.mk
