# boot_splash capsule. Bridges the post-handoff gap: as the first
# compositor client (before the desktop fleet) it attaches a fullscreen
# surface, paints a "NONOS initializing" splash with the boot-chain
# attestation status, optionally shows detail on a keypress, then exits
# so the desktop takes over. Same leaf-renderer capset as setup_wizard
# (no SurfaceMap/Present).

CAPSULE_SLUG             := boot-splash
CAPSULE_HANDLE           := app.boot_splash
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_boot_splash
CAPSULE_BIN_NAME         := boot_splash
CAPSULE_FEATURE          := nonos-capsule-boot-splash
CAPSULE_NAMESPACE        := systems.nonos.app.boot_splash
CAPSULE_SERVICE_ENDPOINT := service:4796:app.boot_splash
CAPSULE_REPLY_ENDPOINT   := reply:4797:endpoint.app.boot_splash.reply
CAPSULE_REQUIRED_CAPS    := 0x1819
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_boot_splash

include nonos-mk/capsule.mk
