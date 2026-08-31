CAPSULE_SLUG             := video-player
CAPSULE_HANDLE           := app.video_player
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_video_player
CAPSULE_BIN_NAME         := video_player
CAPSULE_FEATURE          := nonos-capsule-video-player
CAPSULE_NAMESPACE        := systems.nonos.app.video_player
CAPSULE_SERVICE_ENDPOINT := service:4926:app.video_player
CAPSULE_REPLY_ENDPOINT   := reply:4927:endpoint.app.video_player.reply
CAPSULE_INSTANCE_ENDPOINTS := service:4928:app.video_player.1 reply:4929:endpoint.app.video_player.1.reply service:4930:app.video_player.2 reply:4931:endpoint.app.video_player.2.reply
# CoreExec|IPC|Memory|FileSystem|GraphicsDisplayQuery|GraphicsSurfaceCreate|GraphicsSurfaceMap
CAPSULE_REQUIRED_CAPS    := 0x3859
# Debug, granted only by a build that compiles `capsule-serial-debug`.
CAPSULE_OPTIONAL_CAPS    := 0x100
CAPSULE_CAPS_CEILING     := 0x3959
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_video_player

include nonos-mk/capsule.mk
