# desktop_shell capsule. Owns desktop policy, shell surfaces, tray
# dispatch, and app-launch IPC. It consumes compositor/wm services
# and never talks to graphics hardware directly.

CAPSULE_SLUG             := desktop-shell
CAPSULE_HANDLE           := desktop_shell
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_desktop_shell
CAPSULE_BIN_NAME         := desktop_shell
CAPSULE_FEATURE          := nonos-capsule-desktop-shell
CAPSULE_NAMESPACE        := systems.nonos.desktop_shell
CAPSULE_SERVICE_ENDPOINT := service:4410:desktop_shell
CAPSULE_REPLY_ENDPOINT   := reply:4411:endpoint.desktop_shell.reply
# CoreExec | Network | IPC | Memory | Debug | GraphicsDisplayQuery
# | GraphicsSurfaceCreate | SpawnWindow
# = 0x01 | 0x04 | 0x08 | 0x10 | 0x100 | 0x800 | 0x1000 | 0x1000000 = 0x100191d
# Debug carries mk_debug for the shell-frametime counter. Must stay in sync
# with requested_caps in src/userspace/capsule_desktop_shell/spawn.rs.
CAPSULE_REQUIRED_CAPS    := 0x100191d

# Uncomment for a frame-time measurement build; leave off for release.
# CAPSULE_CARGO_FEATURES := shell-frametime
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_desktop_shell

include nonos-mk/capsule.mk
