# audio.server — system audio mixing service. IPC-only: no hardware,
# no broker claim, no MMIO/IRQ/DMA. It receives client play/PCM
# requests, mixes them additively into an S16 buffer, and forwards
# the result to the driver.hda0 PCM sink over IPC.

CAPSULE_SLUG             := audio
CAPSULE_HANDLE           := audio.server
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_audio
CAPSULE_BIN_NAME         := audio_server
CAPSULE_FEATURE          := nonos-capsule-audio
CAPSULE_NAMESPACE        := systems.nonos.audio.server
CAPSULE_SERVICE_ENDPOINT := service:4226:audio.server
CAPSULE_REPLY_ENDPOINT   := reply:4227:endpoint.4294967314
# IPC|Memory|Debug + CoreExec base = 0x119
CAPSULE_REQUIRED_CAPS    := 0x119

include nonos-mk/capsule.mk
