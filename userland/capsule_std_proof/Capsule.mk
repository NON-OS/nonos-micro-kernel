# std_proof — proof that an unmodified crates.io crate runs on NONOS
# through the std platform layer. `main` parses JSON with serde_json,
# pulled straight from crates.io with no source edits, and prints one
# serial line. Built with -Zbuild-std=std and linked against the
# nonos-rt _start shim. IPC and Memory only; no drivers.

CAPSULE_BUILD_STD          := std,panic_abort

CAPSULE_SLUG               := std-proof
CAPSULE_HANDLE             := std_proof
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_DIR                := userland/capsule_std_proof
CAPSULE_BIN_NAME           := std_proof
CAPSULE_FEATURE            := nonos-capsule-std-proof
CAPSULE_NAMESPACE          := systems.nonos.std_proof
CAPSULE_SERVICE_ENDPOINT   := service:4502:std_proof
CAPSULE_REPLY_ENDPOINT     := reply:4503:endpoint.std_proof.reply
# CoreExec | IPC | Memory = 0x01 | 0x08 | 0x10 = 0x19
CAPSULE_REQUIRED_CAPS      := 0x19
CAPSULE_KERNEL_MIRROR      := src/userspace/capsule_std_proof

include nonos-mk/capsule.mk
