# NØNOS userland: capsule_linux
# eK@nonos.systems
#
# The Linux personality. ForeignExec is the whole point of this capsule
# and no other capsule holds it: it is the right to create a process the
# kernel has not verified, build its address space, and answer the calls
# it makes. Crypto is there for getrandom, Debug for the guest's console
# until the file layer carries it, and the rest is the ordinary capsule
# floor.
#
# LocalSign is what lets this capsule vouch for a package it installed.
# Without it MkLocalSign is refused, every package goes into the store
# with no trailer, and the exec gate then refuses all of them: the
# marketplace installs software that can never run. It is a narrow
# right. The trailer is made against the machine's own root, and that
# root verifies nothing until a human confirms a code on the console,
# which lapses at the next boot.
#
# = CoreExec 0x1 | IPC 0x8 | Memory 0x10 | Crypto 0x20 | Debug 0x100
#   | ForeignExec 0x100000000 | LocalSign 0x200000000 = 0x300000139

CAPSULE_SLUG             := linux
CAPSULE_HANDLE           := app.linux
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_linux
CAPSULE_BIN_NAME         := linux
CAPSULE_FEATURE          := nonos-capsule-linux
CAPSULE_NAMESPACE        := systems.nonos.app.linux
CAPSULE_SERVICE_ENDPOINT := service:4936:app.linux
CAPSULE_REPLY_ENDPOINT   := reply:4937:endpoint.app.linux.reply
CAPSULE_REQUIRED_CAPS    := 0x300000139
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_linux

include nonos-mk/capsule.mk
