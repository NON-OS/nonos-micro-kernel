# wm capsule. Owns window state (geometry, kind, visibility),
# z-order, focus, and lifecycle subscriptions. The app owns its
# surface and shares it directly with the compositor; the wm only
# carries metadata and pushes FOCUS_SET to the compositor when
# focus changes.

CAPSULE_SLUG             := wm
CAPSULE_HANDLE           := wm
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_wm
CAPSULE_BIN_NAME         := wm
CAPSULE_FEATURE          := nonos-capsule-wm
CAPSULE_NAMESPACE        := systems.nonos.wm
CAPSULE_SERVICE_ENDPOINT := service:4330:wm
CAPSULE_REPLY_ENDPOINT   := reply:4331:endpoint.wm.reply
# CoreExec | IPC | Memory = 0x01 | 0x08 | 0x10 = 0x19
# Debug deliberately absent: wm emits no MkDebug markers in steady state.
# The NO LOGS / NO TRACES posture refuses any serial surface.
CAPSULE_REQUIRED_CAPS    := 0x19
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_wm

include nonos-mk/capsule.mk
