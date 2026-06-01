CAPSULE_SLUG             := process-manager
CAPSULE_HANDLE           := app.process_manager
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_process_manager
CAPSULE_BIN_NAME         := process_manager
CAPSULE_FEATURE          := nonos-capsule-process-manager
CAPSULE_NAMESPACE        := systems.nonos.app.process_manager
CAPSULE_SERVICE_ENDPOINT := service:4730:app.process_manager
CAPSULE_REPLY_ENDPOINT   := reply:4731:endpoint.app.process_manager.reply
# CoreExec|IPC|Memory|Debug|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1919
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_process_manager

include nonos-mk/capsule.mk
