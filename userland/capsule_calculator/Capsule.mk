CAPSULE_SLUG             := calculator
CAPSULE_HANDLE           := app.calculator
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_calculator
CAPSULE_BIN_NAME         := calculator
CAPSULE_FEATURE          := nonos-capsule-calculator
CAPSULE_NAMESPACE        := systems.nonos.app.calculator
CAPSULE_SERVICE_ENDPOINT := service:4720:app.calculator
CAPSULE_REPLY_ENDPOINT   := reply:4721:endpoint.app.calculator.reply
# CoreExec|IPC|Memory|Debug|GraphicsDisplayQuery|GraphicsSurfaceCreate
CAPSULE_REQUIRED_CAPS    := 0x1919
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_calculator

include nonos-mk/capsule.mk
