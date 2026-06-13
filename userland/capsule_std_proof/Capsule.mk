# std_proof — userland capsule built against nonos_std, the sovereign
# standard library. Its `_start` exercises a seeded HashMap, Arc<Mutex>,
# time, and process on the verified spawn path, then prints one serial
# line. Booting it proves a nonos_std capsule runs end to end. IPC and
# Memory only; no drivers, no broker resources.

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
