# proof_io — first signed userland capsule on the verified spawn
# path. Tiny `_start` does MkDebug + MkExit so the boot serial
# proves SYSCALL/SYSRET round-trip end-to-end before any heavier
# capsule comes up. No drivers, no broker resources — IPC and
# Memory only.

CAPSULE_SLUG               := proof-io
CAPSULE_HANDLE             := proof_io
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_DIR                := userland/capsule_proof_io
CAPSULE_BIN_NAME           := proof_io
CAPSULE_FEATURE            := nonos-capsule-proof-io
CAPSULE_NAMESPACE          := systems.nonos.proof_io
CAPSULE_SERVICE_ENDPOINT   := service:4500:proof_io
CAPSULE_REPLY_ENDPOINT     := reply:4501:endpoint.proof_io.reply
# IPC | Memory = 0x08 | 0x10 = 0x19
CAPSULE_REQUIRED_CAPS      := 0x19
CAPSULE_KERNEL_MIRROR      := src/userspace/capsule_proof_io
# proof_io's `_start` predates the global allocator and never calls
# alloc; build it without alloc to keep the ELF minimal.
CAPSULE_BUILD_STD          := core
CAPSULE_BUILD_STD_FEATURES :=

include nonos-mk/capsule.mk
