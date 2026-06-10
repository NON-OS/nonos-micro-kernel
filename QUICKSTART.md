# Build your first NONOS capsule

This is the path from a clean clone to a signed, ZK-attested capsule
running on the NONOS desktop. Every step below was executed exactly as
written to build `userland/capsule_hello`, which is kept in the tree as
the living reference for this guide. Budget an afternoon the first
time; the build tooling does all the security work for you.

Prerequisites: rustup (the repo pins its own toolchain), QEMU, and
make. macOS and Linux both work.

Clone with submodules; the trust keystore and the build includes come
along, including the canonical ceremony keys your proofs verify
against (verifying key fingerprint 6cd2015037ea6181):

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
NONOS-ID certificate, and generates the Groth16 attestation proof that
binds the capsule hash, capability mask and policy root. Verify it
yourself the way the kernel will:

```sh
make nonos-mk-attestation-receipt
```

Your capsule appears in the receipt with a live pairing-check verdict.
If the proof does not verify, nothing downstream will accept the
capsule; there is no bypass to forget.

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

```sh
make nonos-mk-run
```

QEMU boots the verified chain and your window comes up on the desktop.

## What you never had to do

Touch kernel internals, hand-roll signing, manage certificates,
understand Groth16, or ask anyone for permission. The pipeline refuses
to produce an unsigned or unattested capsule, so the secure path and
the easy path are the same path.

If a step here does not match reality, that is a bug in this document;
please file it.
