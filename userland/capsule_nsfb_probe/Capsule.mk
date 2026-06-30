CAPSULE_SLUG               := nsfb-probe
CAPSULE_HANDLE             := nsfb_probe
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_DIR                := userland/capsule_nsfb_probe
CAPSULE_BIN_NAME           := nsfb_probe
CAPSULE_FEATURE            := nonos-capsule-nsfb-probe
CAPSULE_NAMESPACE          := systems.nonos.nsfb_probe
CAPSULE_SERVICE_ENDPOINT   := service:4512:nsfb_probe
CAPSULE_REPLY_ENDPOINT     := reply:4513:endpoint.nsfb_probe.reply
# CoreExec | IPC | Memory | Debug | GraphicsSurfaceCreate
# = 0x01 | 0x08 | 0x10 | 0x100 | 0x1000 = 0x1119
# Debug(0x100) carries the write(1,...) -> MkDebug serial marker;
# GraphicsSurfaceCreate(0x1000) authorizes surface register/share/release.
CAPSULE_REQUIRED_CAPS      := 0x1119
CAPSULE_KERNEL_MIRROR      := src/userspace/capsule_nsfb_probe
CAPSULE_PREBUILT_BIN       := userland/capsule_nsfb_probe/build/nsfb_probe

include nonos-mk/capsule-c.mk
include nonos-mk/capsule.mk
