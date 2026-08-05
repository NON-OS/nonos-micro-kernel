CAPSULE_SLUG             := audio_player
CAPSULE_HANDLE           := app.audio_player
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_audio_player
CAPSULE_BIN_NAME         := audio_player
CAPSULE_FEATURE          := nonos-capsule-audio-player
CAPSULE_NAMESPACE        := systems.nonos.app.audio_player
CAPSULE_SERVICE_ENDPOINT := service:4870:app.audio_player
CAPSULE_REPLY_ENDPOINT   := reply:4871:endpoint.app.audio_player.reply
CAPSULE_INSTANCE_ENDPOINTS := service:4874:app.audio_player.1 reply:4875:endpoint.app.audio_player.1.reply service:4876:app.audio_player.2 reply:4877:endpoint.app.audio_player.2.reply
# CoreExec|IPC|Memory|FileSystem|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1959
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_audio_player

include nonos-mk/capsule.mk
