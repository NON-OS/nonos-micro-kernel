# input_probe capsule. Test-only input-echo surface for the
# input-stack end-to-end harness: renders to a compositor surface
# exactly like desktop_shell, subscribes to input deliveries and
# echoes them back on-screen. Mirrors desktop_shell's leaf-renderer
# capset (no SurfaceMap/Present — those are compositor/driver caps).

CAPSULE_SLUG             := input-probe
CAPSULE_HANDLE           := app.input_probe
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_input_probe
CAPSULE_BIN_NAME         := input_probe
CAPSULE_FEATURE          := nonos-capsule-input-probe
CAPSULE_NAMESPACE        := systems.nonos.app.input_probe
CAPSULE_SERVICE_ENDPOINT := service:4792:app.input_probe
CAPSULE_REPLY_ENDPOINT   := reply:4793:endpoint.app.input_probe.reply
CAPSULE_REQUIRED_CAPS    := 0x1819
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_input_probe

include nonos-mk/capsule.mk
