# Paths, versions, keys, and the pinned toolchain: the global variables every
# other module builds on. No targets live here.

BOOTLOADER_DIR := nonos-bootloader
TARGET_DIR     := target
ESP_DIR        := $(TARGET_DIR)/esp
USB_IMG        := $(TARGET_DIR)/nonos.img
# Total image size in MiB. The attested kernel alone is ~70 MB, so keep generous headroom.
USB_IMG_MB     ?= 256
KEYS_DIR       := $(BOOTLOADER_DIR)/keys

export SOURCE_DATE_EPOCH ?= $(shell git log -1 --format=%ct 2>/dev/null || date +%s)
export CARGO_INCREMENTAL := 0

# Capsules are independent targets and their shared inputs are real
# prerequisites, so the set fans out safely. NONOS_JOBS=1 when bisecting a
# build failure and interleaved output gets in the way.
#
# How wide is a memory question rather than a core-count one. A rustc pass over
# the kernel crate holds a couple of gigabytes, and make's fan-out multiplies
# that by cargo's own, so sizing purely off logical cores oversubscribes badly:
# eight make jobs each starting an eight-way cargo is dozens of concurrent
# rustc on a machine that fits about six. That drives the host into swap, where
# the build gets slower than it was serial and anything else running, a QEMU
# guest in particular, fails to allocate at all.
#
# So the width is the smaller of the physical cores and what memory can hold at
# roughly four gigabytes a job, and cargo is capped per invocation so the two
# layers cannot multiply out of hand.
NONOS_CORES := $(shell sysctl -n hw.physicalcpu 2>/dev/null || nproc 2>/dev/null || echo 2)
NONOS_RAM_GB := $(shell expr $$(sysctl -n hw.memsize 2>/dev/null || echo 8589934592) / 1073741824)
NONOS_JOBS ?= $(shell \
	c=$(NONOS_CORES); m=$$(expr $(NONOS_RAM_GB) / 4); \
	[ "$$m" -lt "$$c" ] && c=$$m; [ "$$c" -lt 1 ] && c=1; echo $$c)
MAKEFLAGS += -j$(NONOS_JOBS)
export CARGO_BUILD_JOBS ?= 2

# Every capsule builds the standard library into its own target directory, so
# a cold tree compiles core and alloc once per capsule. One shared
# CARGO_TARGET_DIR would dedupe that, but cargo locks a target directory for
# the whole build and would serialise the fan-out above. A content-addressed
# cache dedupes without taking that lock.
#
# It caches inputs to a signing pipeline, so it stays local and stays
# switchable. NONOS_CACHE=0 for release and reproducibility builds.
NONOS_CACHE ?= auto
ifneq ($(NONOS_CACHE),0)
NONOS_SCCACHE := $(shell command -v sccache 2>/dev/null)
ifneq ($(NONOS_SCCACHE),)
export RUSTC_WRAPPER := $(NONOS_SCCACHE)
endif
endif

export PATH := $(HOME)/.cargo/bin:$(PATH)
TOOLCHAIN := nightly-2026-01-16
CARGO     := $(HOME)/.cargo/bin/cargo
RUSTUP    := $(HOME)/.cargo/bin/rustup
NONOS_PYTHON ?= /usr/bin/python3
NONOS_BENCH_OUT ?=
NONOS_BENCH_BUILD_CMD ?= make nonos-mk-verify-fast
NONOS_BENCH_SKIP_BUILD ?= 0
NONOS_BENCH_SKIP_BOOT ?= 0
NONOS_BENCH_STRICT ?= 0
NONOS_BENCH_BOOT_TIMEOUT ?= 360
NONOS_BENCH_BOOT_CMD ?=
NONOS_BENCH_HOST_OUT ?= target/bench/host
NONOS_BENCH_BOOT_JSON ?= target/bench/boot-log.json
NONOS_BENCH_SERIAL_LOG ?=
NONOS_BENCH_BASELINE ?=
NONOS_BENCH_CANDIDATE ?=
NONOS_BENCH_REGRESSION_LIMIT ?= 15

UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)

ifeq ($(UNAME_S),Darwin)
    ifeq ($(UNAME_M),arm64)
        HOST_TARGET := aarch64-apple-darwin
    else
        HOST_TARGET := x86_64-apple-darwin
    endif
    SDK_PATH   := /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk
    SDK_EXISTS := $(shell test -d $(SDK_PATH) && echo yes)
    ifeq ($(SDK_EXISTS),yes)
        SDK_FLAGS := SDKROOT=$(SDK_PATH) \
                     AR=/Library/Developer/CommandLineTools/usr/bin/ar \
                     CC=/Library/Developer/CommandLineTools/usr/bin/clang \
                     PATH="/Library/Developer/CommandLineTools/usr/bin:$$PATH"
    else
        SDK_FLAGS :=
    endif
    SHA256 := shasum -a 256
else ifeq ($(UNAME_S),Linux)
    HOST_TARGET := x86_64-unknown-linux-gnu
    SDK_FLAGS   :=
    SHA256      := sha256sum
else
    $(error Unsupported host platform: $(UNAME_S))
endif

# Signing key. Auto-generated on first use.
SIGNING_KEY ?= $(KEYS_DIR)/signing_key_v1.bin
KERNEL_MLDSA65_PREFIX ?= $(KEYS_DIR)/kernel_mldsa65
KERNEL_MLDSA65_KEY ?= $(KERNEL_MLDSA65_PREFIX).seed
KERNEL_MLDSA65_PUB ?= $(KERNEL_MLDSA65_PREFIX).pub
NONOS_TRUST_DIR := nonos-data/trust

# Transparent ZK attestation paths.
ZK_CIRCUIT_DIR   := $(BOOTLOADER_DIR)/tools/nonos-attestation-circuit
ZK_KEYS_DIR      := $(ZK_CIRCUIT_DIR)/generated_keys
ZK_ENROLL_TOOL   := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/transparent-enroll
ZK_TRANSPARENT_PROVE_TOOL := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/transparent-prove
ZK_TRANSPARENT_VERIFY_TOOL := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/transparent-verify
ZK_CAPSULE_PROOF_TOOL := $(ZK_CIRCUIT_DIR)/target/$(HOST_TARGET)/release/capsule-attest-proof
# Transparent post-quantum STARK enrollment. Produces the capsule policy root
# and every capsule's money-grade membership trailer, verified by the same
# nonos-stark the kernel and bootloader link. This is the production capsule
# attestation path; the curve-based capsule-attest-proof above is retired.
NONOS_STARK_ENROLL := nonos-stark-enroll/target/$(HOST_TARGET)/release/nonos-stark-enroll
CAPSULE_SIGN_BIN := nonos-sign/target/release/capsule-sign
ZK_BOOT_ROOT     ?= $(NONOS_TRUST_DIR)/zk/device_root.bin
ZK_BOOT_COMMITMENTS ?= $(NONOS_TRUST_DIR)/zk/device_commitments.bin
ZK_BOOT_LABELS   ?= $(NONOS_TRUST_DIR)/zk/device_labels.txt
ZK_BOOT_SECRETS  ?= $(NONOS_TRUST_DIR)/zk/device_secrets.txt
ZK_BOOT_ENROLL_SEED ?=
ZK_BOOT_INDEX    ?=
ZK_BOOT_SECRET_X ?=
ZK_BOOT_SECRET_R ?=
ZK_BOOT_NONCE    ?=
ZK_BOOT_MACHINE_ID ?=
ZK_BOOT_TIMESTAMP ?=
ZK_BOOT_NONCE_SEED ?=
ZK_BOOT_CHALLENGE ?=
ZK_BOOT_SIDECAR  ?= $(ESP_DIR)/EFI/nonos/boot.zkp
ZK_CAPSULE_ROOT  ?= $(NONOS_TRUST_DIR)/policy/zk_capsule_policy_root.bin
# The kernel embeds this root at compile time (security/capsule_attest), so an
# attested kernel build depends on the capsule enrollment having run first.
ZK_POLICY_ROOT   ?= $(ZK_CAPSULE_ROOT)
# Kernel self-attestation: the bootloader embeds this root and verifies the
# kernel's own STARK membership trailer before the jump. Provisioned by the
# kernel enrollment step below.
KERNEL_ATTEST_ROOT_BIN ?= $(NONOS_TRUST_DIR)/policy/kernel_attest_root.bin
KERNEL_ATTEST_TRAILER  ?= $(TARGET_DIR)/kernel-attest/kernel.zk_trailer.bin
KERNEL_ATTEST_ELF      ?= $(TARGET_DIR)/x86_64-nonos/release/nonos-kernel
_boot_comma := ,
# Kernel self-attestation is on by default: a clean checkout with no .nonos-config
# (CI, first build) ships the full STARK boot, kernel and capsules both. A config
# carrying NONOS_STARK_KERNEL_ATTEST := 0, or make NONOS_STARK_KERNEL_ATTEST=0 on
# the command line, turns it off for a build that only wants signature trust.
NONOS_STARK_KERNEL_ATTEST ?= 1
# Only the literal 1 turns it on, so an explicit := 0 reads as off; test the
# value, not just whether the variable is defined.
NONOS_STARK_KERNEL_ATTEST_ON := $(filter 1,$(NONOS_STARK_KERNEL_ATTEST))
BOOT_STARK_FEATURE := $(if $(NONOS_STARK_KERNEL_ATTEST_ON),$(_boot_comma)stark-kernel-attest)
ZK_CAPSULE_LABELS ?= $(TARGET_DIR)/capsule-attest/capsule_labels.txt
ZK_CAPSULE_SECRETS ?= $(TARGET_DIR)/capsule-attest/capsule_secrets.txt
ZK_CAPSULE_COMMITMENTS ?= $(TARGET_DIR)/capsule-attest/capsule_commitments.bin
ZK_CAPSULE_ENROLL_SEED ?=
ZK_CAPSULE_NONCE_SEED ?=
ZK_CAPSULE_EPOCH ?= 1
EMBED_TOOL       := $(BOOTLOADER_DIR)/tools/embed-zk-proof/target/$(HOST_TARGET)/release/embed-zk-proof
SIGN_TOOL        := $(BOOTLOADER_DIR)/tools/sign-kernel/target/$(HOST_TARGET)/release/sign-kernel

# Anti-rollback index baked into and signed over the kernel image. Starts at 1,
# the minimum the bootloader accepts (index 0 means unset and is rejected); bump
# only for security-critical releases. The TPM monotonic counter enforces it as
# a floor so an older signed kernel cannot be rolled back onto a device.
NONOS_ROLLBACK_INDEX ?= 1

# Changing the index must force a re-sign: the signed kernel does not otherwise
# depend on the value, so it keys on this per-index stamp.
NONOS_ROLLBACK_STAMP := $(TARGET_DIR)/.rollback-index.$(NONOS_ROLLBACK_INDEX)

# Developer evaluation bootstrap (NOT production custody).
#
# A clean checkout has no enrolled device identity: device_secrets.txt and
# device_root.bin are gitignored on purpose, because attestation proves
# knowledge of that secret and shipping it would let anyone forge proofs.
# Set NONOS_DEV=1 (or use the nonos-mk-dev-* targets) to mint a clearly
# marked throwaway identity from a fixed public seed so the image boots out
# of the box. Production deployers leave NONOS_DEV unset and pass their own
# enrolled secrets; with it unset none of the assignments below apply and the
# build stays fail-closed.
ifeq ($(NONOS_DEV),1)
ifeq ($(strip $(ZK_BOOT_ENROLL_SEED)),)
ZK_BOOT_ENROLL_SEED := nonos-public-dev-enroll-seed-not-for-production
endif
ifeq ($(strip $(ZK_CAPSULE_ENROLL_SEED)),)
ZK_CAPSULE_ENROLL_SEED := nonos-public-dev-capsule-enroll-not-for-production
endif
ifeq ($(strip $(ZK_CAPSULE_NONCE_SEED)),)
ZK_CAPSULE_NONCE_SEED := nonos-public-dev-capsule-nonce-not-for-production
endif
ifeq ($(strip $(ZK_BOOT_NONCE_SEED)),)
ZK_BOOT_NONCE_SEED := deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
endif
ifeq ($(strip $(ZK_BOOT_INDEX)),)
ZK_BOOT_INDEX := 0
endif
# Read the boot secret back from the enrolled identity file so it always
# matches the generated root, whatever seed produced it. Lazy (=) so it
# resolves after the enroll rule has written the file.
ifeq ($(strip $(ZK_BOOT_SECRET_X)),)
ZK_BOOT_SECRET_X = $(shell awk 'NR==1{print $$2}' $(ZK_BOOT_SECRETS) 2>/dev/null)
endif
ifeq ($(strip $(ZK_BOOT_SECRET_R)),)
ZK_BOOT_SECRET_R = $(shell awk 'NR==1{print $$3}' $(ZK_BOOT_SECRETS) 2>/dev/null)
endif
endif

# Header and status line, shown once per invocation. Fields are read from
# the working tree at parse time.
NONOS_BRANCH    := $(shell git rev-parse --abbrev-ref HEAD 2>/dev/null || echo detached)
NONOS_COMMIT    := $(shell git rev-parse --short HEAD 2>/dev/null || echo none)
NONOS_DIRTY     := $(shell test -n "$$(git status --porcelain 2>/dev/null)" && echo '*' || echo '')
NONOS_ATTESTED  := $(shell ls nonos-data/trust/capsules/*.zk_trailer.bin 2>/dev/null | wc -l | tr -d ' ')
NONOS_SIGNED    := $(shell ls nonos-data/trust/capsules/*.manifest.bin 2>/dev/null | wc -l | tr -d ' ')
NONOS_ZK_ROOT_FPR := $(shell test -f $(ZK_BOOT_ROOT) && shasum -a 256 $(ZK_BOOT_ROOT) 2>/dev/null | cut -c1-16 || echo unenrolled)

define NONOS_BANNER

  ███╗   ██╗ ██████╗ ███╗   ██╗ ██████╗ ███████╗
  ████╗  ██║██╔═══██╗████╗  ██║██╔═══██╗██╔════╝
  ██╔██╗ ██║██║   ██║██╔██╗ ██║██║   ██║███████╗
  ██║╚██╗██║██║   ██║██║╚██╗██║██║   ██║╚════██║
  ██║ ╚████║╚██████╔╝██║ ╚████║╚██████╔╝███████║
  ╚═╝  ╚═══╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ ╚══════╝
endef
export NONOS_BANNER

$(info $(NONOS_BANNER))

