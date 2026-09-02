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
# IPC | Memory | Debug | StoreWrite = 0x08 | 0x10 | 0x100 | 0x4000000 = 0x4000119
# Keyring: blockfs_volume mounts and formats with a key from the keyring.
# Debug: one line after seeding says how the store came up; a silent failure
# there reads as a shell timeout two layers up.
CAPSULE_REQUIRED_CAPS    := 0x14000119
CAPSULE_KERNEL_MIRROR    := src/fs/vfs_capsule

# Resonare (app.audio_player) auto-boots on the base desktop and loads its
# default track from /audio/boot_tone.wav, so the audio store must be seeded in
# every image that ships vfs, not only the audio smoketest. Enabling the feature
# here makes seed_audio_store() populate /audio; smoketests still force it via a
# command-line vfs_CARGO_FEATURES override, which composes with this default.
CAPSULE_CARGO_FEATURES   := seed-audio-store

# vfs no longer embeds any signed artifact: packages are read from the block
# device at seed time, and mk/40-run.mk keys the packer on $(std-proof_ARTIFACTS)
# so freshness is enforced there instead. The std_proof entries that used to live
# here are gone with the include_bytes! they existed for; the std_proof trailer
# in particular formed the root -> vfs -> trailer -> root cycle that make was
# dropping with a warning on every build.
#
# The rg entries below predate this change and were already inert: nothing under
# src/ has ever referenced ripgrep. They are left in place pending a decision.
#
# Their paths name the x86_64 user target explicitly, so the block stays guarded
# and any other target depends on nothing here.
ifeq ($(NONOS_USER_TARGET),x86_64-nonos-user)
CAPSULE_EXTRA_DEPS := \
	userland/capsule_ripgrep/target/x86_64-nonos-user/release/rg \
	nonos-data/trust/capsules/rg.nonos_id_cert.bin \
	nonos-data/trust/capsules/rg.manifest.bin

CAPSULE_EXTRA_ORDER_DEPS := \
	nonos-data/trust/capsules/rg.zk_trailer.bin
endif

include nonos-mk/capsule.mk
