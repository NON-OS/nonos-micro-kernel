CAPSULE_SLUG               := relibc-test
CAPSULE_HANDLE             := relibc_test
CAPSULE_DOMAIN             := systems.nonos
CAPSULE_DIR                := userland/capsule_relibc_test
CAPSULE_BIN_NAME           := relibc_test
CAPSULE_FEATURE            := nonos-capsule-relibc-test
CAPSULE_NAMESPACE          := systems.nonos.relibc_test
CAPSULE_SERVICE_ENDPOINT   := service:4506:relibc_test
CAPSULE_REPLY_ENDPOINT     := reply:4507:endpoint.relibc_test.reply
# CoreExec | IPC | Memory | Crypto | FileSystem | Debug
# = 0x01 | 0x08 | 0x10 | 0x20 | 0x40 | 0x100 = 0x179
CAPSULE_REQUIRED_CAPS      := 0x179
CAPSULE_KERNEL_MIRROR      := src/userspace/capsule_relibc_test
CAPSULE_PREBUILT_BIN       := userland/capsule_relibc_test/build/relibc_test

include nonos-mk/capsule-c.mk
include nonos-mk/capsule.mk
