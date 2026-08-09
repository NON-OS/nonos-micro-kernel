CAPSULE_BUILD_STD        := std,panic_abort
CAPSULE_SLUG             := egui_proof
CAPSULE_HANDLE           := egui_proof
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_egui_proof
CAPSULE_BIN_NAME         := egui_proof
CAPSULE_NAMESPACE        := systems.nonos.egui_proof
CAPSULE_SERVICE_ENDPOINT := service:4918:app.egui_proof
CAPSULE_REPLY_ENDPOINT   := reply:4919:endpoint.app.egui_proof.reply
CAPSULE_REQUIRED_CAPS    := 0x1819

include nonos-mk/capsule.mk
