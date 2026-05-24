# vfs — userland service capsule. CAP_VFS is the caller-facing
# gate, not the capsule's bit. The capsule itself only needs IPC
# for `mk_ipc_*` and Memory for the heap.

CAPSULE_SLUG             := vfs
CAPSULE_HANDLE           := vfs
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_vfs
CAPSULE_BIN_NAME         := vfs
CAPSULE_FEATURE          := nonos-capsule-vfs
CAPSULE_NAMESPACE        := systems.nonos.vfs
CAPSULE_SERVICE_ENDPOINT := service:4104:vfs_pool
CAPSULE_REPLY_ENDPOINT   := reply:4105:endpoint.4294967301
# IPC | Memory = 0x08 | 0x10 = 0x19
CAPSULE_REQUIRED_CAPS    := 0x19
CAPSULE_KERNEL_MIRROR    := src/fs/vfs_capsule

include nonos-mk/capsule.mk
