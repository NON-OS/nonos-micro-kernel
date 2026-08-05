CAPSULE_SLUG             := game_2048
CAPSULE_HANDLE           := game_2048
CAPSULE_DOMAIN           := com.example
CAPSULE_DIR              := userland/capsule_game_2048
CAPSULE_BIN_NAME         := game_2048
CAPSULE_NAMESPACE        := com.example.game_2048
CAPSULE_SERVICE_ENDPOINT := service:4920:app.game_2048
CAPSULE_REPLY_ENDPOINT   := reply:4921:endpoint.app.game_2048.reply
CAPSULE_REQUIRED_CAPS    := 0x1819

include nonos-mk/capsule.mk
