# std_proof — proof that unmodified std Rust runs on NONOS through the
# std platform layer: crates.io code, threads over MTSP, a process-local
# env, file I/O with seek over the vfs, and a TCP socket through the
# userland net stack, one PASS/FAIL serial line each. Built with
# -Zbuild-std=std and linked against the nonos-rt _start shim.

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
# CoreExec | IPC | Memory | Debug = 0x01 | 0x08 | 0x10 | 0x100 = 0x119.
# Debug is the proof's output channel (println -> MkDebug -> serial and
# the proc.<pid> inbox the terminal drains). The baked boot spawn folds
# it through serial_debug_cap(); a store install grants it from this
# manifest, which is the point of a proof capsule.
CAPSULE_REQUIRED_CAPS      := 0x119
CAPSULE_KERNEL_MIRROR      := src/userspace/capsule_std_proof

include nonos-mk/capsule.mk
