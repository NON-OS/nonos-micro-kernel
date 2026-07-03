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
# CoreExec | IPC | Memory | GraphicsDisplayQuery | GraphicsSurfaceCreate
# = 0x01 | 0x08 | 0x10 | 0x800 | 0x1000 = 0x1819
CAPSULE_REQUIRED_CAPS    := 0x181d
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_desktop_shell

include nonos-mk/capsule.mk
