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

# Resonare (app.audio_player) auto-boots on the base desktop and loads its
# default track from /audio/boot_tone.wav, so the audio store must be seeded in
# every image that ships vfs, not only the audio smoketest. Enabling the feature
# here makes seed_audio_store() populate /audio; smoketests still force it via a
# command-line vfs_CARGO_FEATURES override, which composes with this default.
CAPSULE_CARGO_FEATURES   := seed-audio-store

# The vfs capsule stages packages into its store by include_bytes!ing their
# signed artifacts (see src/store/fdtable/packages.rs). Capsule signing is not
# byte-deterministic across builds, so whenever an artifact is re-signed the vfs
# binary must rebuild to embed the fresh copy; otherwise it ships artifacts
# signed by a stale trust anchor and the runtime load is rejected. Declaring the
# artifacts as inputs keeps the embedded copies in lockstep with the kernel's
# baked trust roots.
#
# The staged packages are std capsules, and the std platform layer issues its
# syscalls with raw x86_64 registers, so on any other target the store stages
# nothing and there is nothing here to depend on.
ifeq ($(NONOS_USER_TARGET),x86_64-nonos-user)
CAPSULE_EXTRA_DEPS := \
	userland/capsule_std_proof/target/x86_64-nonos-user/release/std_proof \
	userland/capsule_ripgrep/target/x86_64-nonos-user/release/rg \
	nonos-data/trust/capsules/std_proof.nonos_id_cert.bin \
	nonos-data/trust/capsules/std_proof.manifest.bin \
	nonos-data/trust/capsules/rg.nonos_id_cert.bin \
	nonos-data/trust/capsules/rg.manifest.bin

# The embedded STARK trailers are produced by the whole-set enrollment, whose
# policy root commits to every capsule binary, including this one. Depending on
# them normally would form a cycle (root -> vfs -> trailer -> root) that makes
# re-runs the enrollment on every pass. Order-only keeps the guarantee that the
# trailers exist and are current before vfs builds, without a rewritten trailer
# mtime retriggering vfs and, through it, the enrollment.
CAPSULE_EXTRA_ORDER_DEPS := \
	nonos-data/trust/capsules/std_proof.zk_trailer.bin \
	nonos-data/trust/capsules/rg.zk_trailer.bin
endif

include nonos-mk/capsule.mk
