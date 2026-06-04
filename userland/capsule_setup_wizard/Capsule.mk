# setup_wizard capsule. First-boot setup wizard: attaches a fullscreen
# compositor surface, grabs the keyboard, walks the user through setup
# (keys/passphrase/wallpaper), then exits so the kernel brings up the
# desktop. Same leaf-renderer capset as input_probe (no SurfaceMap/Present).

CAPSULE_SLUG             := setup-wizard
CAPSULE_HANDLE           := app.setup_wizard
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_setup_wizard
CAPSULE_BIN_NAME         := setup_wizard
CAPSULE_FEATURE          := nonos-capsule-setup-wizard
CAPSULE_NAMESPACE        := systems.nonos.app.setup_wizard
CAPSULE_SERVICE_ENDPOINT := service:4794:app.setup_wizard
CAPSULE_REPLY_ENDPOINT   := reply:4795:endpoint.app.setup_wizard.reply
CAPSULE_REQUIRED_CAPS    := 0x1819
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_setup_wizard

include nonos-mk/capsule.mk
