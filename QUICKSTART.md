# Build your first NONOS capsule

This is the path from a clean clone to a signed, ZK-attested capsule
running on the NONOS desktop. Every step below was executed exactly as
written to build `userland/capsule_hello`, which is kept in the tree as
the living reference for this guide. Budget an afternoon the first
time; the build tooling does all the security work for you.

Prerequisites: rustup (the repo pins its own toolchain), QEMU, and
make. macOS and Linux both work. The build installs the toolchain and
its `rust-src` component for you on first run; if you ever see a
`-Zbuild-std` / missing `rust-src` error, the build's toolchain step
will install it, or you can add it directly with
`rustup component add rust-src --toolchain nightly-2026-01-16`.

Clone with submodules; the trust keystore and the build includes come
along with the tree:

```sh
git clone --recursive https://github.com/NON-OS/nonos-micro-kernel.git
```

## 1. Write the app

A capsule is a normal Cargo crate. Copy the shape of
`userland/capsule_hello`: five small files.

`src/main.rs` wires the capsule entry point to your app:

```rust
#![no_std]
#![no_main]

extern crate alloc;

mod hello;

use nonos_app_skeleton::run;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    run(hello::Hello::new)
}
```

Your app implements one trait with three methods:

```rust
impl App for Hello {
    fn manifest(&self) -> AppManifest { ... }   // window title, size, input mask
    fn on_event(&mut self, e: InputEvent) -> EventOutcome { ... }
    fn paint(&mut self, fb: &mut PaintBuffer) { ... }
}
```

`PaintBuffer` gives you `clear`, `fill_rect`, `text` and `text_scaled`.
`EventOutcome::Close` from `on_event` closes the window; return
`Repaint` when state changed and `Idle` otherwise. Look at
`userland/capsule_hello/src/hello/` for the complete 5-file version
and `userland/capsule_about` for a bigger app with tabs and scrolling.

Your `Cargo.toml` takes two path dependencies:

```toml
[dependencies]
nonos_libc = { package = "nonos_userland_libc", path = "../libc" }
nonos_app_skeleton = { path = "../app_skeleton" }
```

## 2. Declare the capsule

Create `userland/capsule_<name>/Capsule.mk` (13 lines, copy hello's
and change the names and ports):

```make
CAPSULE_SLUG             := hello
CAPSULE_HANDLE           := app.hello
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_hello
CAPSULE_BIN_NAME         := hello
CAPSULE_FEATURE          := nonos-capsule-hello
CAPSULE_NAMESPACE        := systems.nonos.app.hello
CAPSULE_SERVICE_ENDPOINT := service:4810:app.hello
CAPSULE_REPLY_ENDPOINT   := reply:4811:endpoint.app.hello.reply
CAPSULE_REQUIRED_CAPS    := 0x1819
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_hello

include nonos-mk/capsule.mk
```

Pick a free port pair (grep the other Capsule.mk files). The caps mask
0x1819 is the standard GUI set: CoreExec, IPC, Memory, display query,
surface create. Then add one line to the include list in the root
`Makefile`, next to the other capsules:

```make
include userland/capsule_hello/Capsule.mk
```

## 3. Generate your publisher identity

Two keypairs, one command each. Seeds stay in `.keys/` (never
committed), public keys go into the trust keystore:

```sh
cd nonos-sign && cargo build --release --bin capsule-sign && cd ..
nonos-sign/target/release/capsule-sign keygen --alg ed25519 --out .keys/hello_publisher_ed25519
nonos-sign/target/release/capsule-sign keygen --alg mldsa65 --out .keys/hello_publisher_mldsa65
mv .keys/hello_publisher_*.pub nonos-data/trust/keys/
```

## 4. Build, sign, attest

```sh
make nonos-mk-hello          # cross-compile the capsule ELF
make nonos-mk-hello-sign     # manifest + NONOS-ID cert + ZK proof
```

The sign target emits the signed manifest, mints your capsule's
NONOS-ID certificate, and generates the transparent enrolled-secret
attestation proof that binds the capsule hash, capability mask, epoch
and policy root. What lands
where:

```
  your crate                          nonos-data/trust/capsules/
  -----------                         --------------------------
  hello (ELF) ---------+
                       |              hello.manifest.bin
  .keys/hello_*.seed --+--- sign ---> hello.nonos_id_cert.bin
   (yours, private)    |              hello.zk_trailer.bin
                       |
  enrollment seed -----+              target/capsule-attest/
   (private)                          generated proof inputs
```

Verify it yourself the way the kernel will:

```sh
make nonos-mk-capsules
```

The capsule kernel profile embeds the proof trailer and compiles only
when every referenced trust artifact is present. If the proof does not
verify at spawn time, the runtime gate refuses the capsule; there is no
bypass to forget.

## 5. Let the kernel spawn it

The kernel embeds capsules it boots. Mirror the four small files in
`src/userspace/capsule_hello/` (mod, embed, spawn, state; copy hello's
and rename), then register it in three places:

- `src/userspace/mod.rs`: `pub mod capsule_hello;`
- root `Cargo.toml`: add `nonos-capsule-hello = []` to the features
  and the same name to the `microkernel-desktop-gui` capsule list
- `src/userspace/init/spawn_plan/apps.rs`: a `spawn_hello()` entry
  matching the others

The spawn gate re-verifies your attestation proof at every boot before
the process gets memory. A capsule with a missing or invalid proof is
refused; you will see it in the serial log as `[ZK-ATTEST]`.

## 6. Run it

From a clean clone you have no enrolled device identity yet (the secret is
gitignored, because attestation proves knowledge of it). One command mints a
throwaway developer identity from a fixed public seed and boots:

```sh
make nonos-mk-dev-run
```

That identity is for evaluation only: public seed, no custody, never ship it.
For a real build you enroll your own identity and pass its secret, then:

```sh
ZK_BOOT_INDEX=0 ZK_BOOT_SECRET_X=... ZK_BOOT_SECRET_R=... \
ZK_BOOT_NONCE_SEED=... ZK_CAPSULE_ENROLL_SEED=... ZK_CAPSULE_NONCE_SEED=... \
make nonos-mk-run
```

QEMU boots the verified chain and your window comes up on the desktop.
The default boot attaches no NIC. Use `make nonos-mk-run-nat` for outbound
network tests such as wallet RPC, or `make nonos-mk-run-net` when you also
need QEMU host forwarding.

## What you never had to do

Touch kernel internals, hand-roll signing, manage certificates,
understand trusted setup, or ask anyone for permission. The pipeline refuses
to produce an unsigned or unattested capsule, so the secure path and
the easy path are the same path.

If a step here does not match reality, that is a bug in this document;
please file it.
