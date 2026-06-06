# Retired workflows

These six workflows were removed when the modular `ci-next` suite landed.
This file records, per file, the real work each one did so nothing is lost
when the new `_<module>.yml` reusable workflows are wired. Targets, scripts,
and serial markers below are the concrete things the replacements must keep
exercising.

## boot-smoke.yml

Runtime smoke under QEMU + OVMF. `on: workflow_dispatch` only (push/PR
triggers were parked while the QEMU sig-verify path was under triage).

Per-lane shape: checkout (submodules: recursive), install nightly via
rustup (`rust-src llvm-tools-preview rustfmt`, target `x86_64-unknown-uefi`),
install `python3 python3-pip xorriso gdisk mtools qemu-system-x86 ovmf
binutils` (some lanes add `netcat-openbsd`), `pip3 install --user pynacl`,
`bash nonos-ci/setup-signing-key.sh`, `make nonos-mk-ensure-zk-keys`, run the
boot make target, tail/upload the serial log on failure, `rm -rf .keys/`.

Lanes and their make targets + expected serial markers:

- ramfs-round-trip: `make nonos-mk-boot-ramfs`
  log `target/boot-test-ramfs.log`
  markers: `[INIT] Starting`, `[RAMFS] capsule spawned`,
  `[RAMFS-TEST] open ok|write ok|read ok|truncate ok|close ok|PASS`.
- keyring-round-trip: `make nonos-mk-boot-keyring`
  log `target/boot-test-keyring.log`
  markers: `[INIT] Starting`, `[KEYRING] capsule spawned`,
  `[KEYRING-TEST] capsule alive|store ok|retrieve ok|lock ok|
  retrieve-locked denied|unlock ok|retrieve-unlocked ok|metadata ok|
  count ok|delete ok|retrieve-after-delete denied|PASS`.
- ps2-input-round-trip: `make nonos-mk-boot-ps2-input`
  log `target/boot-test-driver-ps2-input.log`
  markers: `[INIT] Starting`, `[driver_ps2] endpoint driver.ps2_kbd0 ready`,
  `[DRIVER-PS2-TEST] capsule alive|healthcheck ok|get_state ok|
  poll_events ok|counters advanced|PASS`.
- xhci-round-trip (P0 controller bring-up, no USB enumeration):
  `make nonos-mk-boot-xhci`, log `target/boot-test-driver-xhci.log`
  markers: `[INIT] Starting`, the full `[driver_xhci] discover ok ...
  endpoint driver.xhci0 ready` bring-up sequence, then
  `[DRIVER-XHCI-TEST] capsule alive|healthcheck ok|controller_status ok|
  port_status ok|PASS`.
- input-e2e-ps2: `make nonos-mk-boot-input-e2e-ps2`
  log `target/boot-test-input-e2e-ps2.log`
  markers: `[INPUT-PROOF] surface composited|surface ready|key down code=|
  pointer motion x=|click dispatch local=|focus routed|PASS`.
- input-e2e-xhci (key flows via USB HID): `make nonos-mk-boot-input-e2e-xhci`
  log `target/boot-test-input-e2e-xhci.log`, same `[INPUT-PROOF]` markers.

A `boot-smoke-summary` job gated on all six lanes (`result == success`).

## microkernel-baseline.yml

Blocking microkernel-only baseline. `on: push [main, develop]`,
`pull_request [main]`.

- static-checks: `bash nonos-ci/run-static-checks.sh` then
  `make nonos-mk-static`. Enforces `default = ["microkernel-core"]`, active
  `nonos = [...]` profile, no `nonos-legacy-tree` / `nonos-syscall-keyring`
  reachable from production profiles, and baseline counters in
  `nonos-ci/baselines/*`.
- cargo-check-default: `cargo check --release --target x86_64-nonos.json
  -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem`
  (default features). Needs signing key at `.keys/dev-signing.seed`
  exported as `NONOS_SIGNING_KEY`.
- cargo-check-microkernel-core: same as above with
  `--no-default-features --features microkernel-core`.
- build-and-scan: scratch-trust-bootstrap for `proof-io ramfs keyring`,
  build `nonos-mk-libc nonos-mk-proof-io nonos-mk-ramfs nonos-mk-keyring`,
  `make nonos-mk-capsules`, `make nonos-mk-scan`,
  `bash nonos-ci/scan-microkernel-symbols.sh`; snapshot the
  microkernel-capsules image; then `make nonos-mk-keyring-test` and re-scan;
  snapshot the smoketest image; upload both images (retention 14).
- repro (advisory, `continue-on-error`): pin `SOURCE_DATE_EPOCH` to the
  commit time, build `nonos-mk-capsules` twice with `make nonos-mk-clean`
  between, compare `sha256sum` of `target/x86_64-nonos/release/nonos-kernel`,
  emit objdump section-header diff, upload `target/repro/`.
- baseline-summary gated on static-checks + both cargo-checks + build-and-scan
  (repro lane explicitly advisory).

## nonos-trust-chain.yml

Trust-chain CI for the verified-spawn capsule set. `on: push [main, develop]`,
`pull_request [main]`, weekly `cron 0 6 * * 1`, `workflow_dispatch`.

Two distinct, deliberately-not-conflated proof lanes:

- host-trust (baked artifact proof): assert baked policy
  `nonos-data/trust/policy/nonos_trust_anchor.policy.bin` and per-capsule
  `nonos-data/trust/capsules/<bin>.nonos_id_cert.bin` + `.manifest.bin`
  for `proof_io ramfs driver_virtio_rng driver_ps2_input driver_virtio_blk
  driver_virtio_net driver_xhci` are present (fail loudly unless
  `NONOS_CI_ALLOW_NO_BAKED_ARTIFACTS=1`), build `nonos-sign` `capsule-sign`,
  run `make nonos-mk-host-trust-test`, capture sha256 of baked artifacts.
- scratch-sign-matrix (scratch ceremony): build `capsule-sign`, keygen
  ed25519 + mldsa65 trust-anchor and per-capsule publisher keys (13 slugs),
  build libc + marketplace-abi + all capsule ELFs, seal policy
  (`make nonos-mk-trust-policy`), `make nonos-mk-<slug>-sign` for each,
  verify via `cd nonos-sign && cargo test --release --test artifacts`,
  collect cert/manifest/pubkey sha256, refuse to upload `.keys/*.seed`
  (assert mode 600), upload public keys + certs + manifests + digests.
- kernel-cargo-matrix: per verified-capsule feature, build the userland ELF
  then `cargo check --target x86_64-nonos.json
  -Zbuild-std=core,alloc,compiler_builtins
  -Zbuild-std-features=compiler-builtins-mem --no-default-features` with
  `microkernel-core,<feature>` and again with
  `microkernel-core,nonos-production,<feature>`.
- kernel-cargo-production-all: build every verified ELF, single
  `cargo check` with `microkernel-core,nonos-production` + all 13
  `nonos-capsule-*` features.
- userland-cargo-check: per-capsule `cargo check --release --target
  ../x86_64-nonos-user.json -Zbuild-std=<std>
  [-Zbuild-std-features=compiler-builtins-mem]` after `make nonos-mk-libc`.

## regenerate-trust.yml

`on: push [main]`, `permissions: contents: write`. Skips when the head
commit message starts with `trust: auto-regenerate` (loop guard). Requires
`TRUST_REGEN_PAT`. Installs nightly (`rust-src`), runs
`bash nonos-ci/scratch-trust-bootstrap.sh` over the full 14-slug
`CAPSULE_SLUGS` list, refreshes `nonos-data/trust/MANIFEST.sha256`
(`shasum -a 256 policy/*.bin keys/*.pub capsules/*.bin | sort -k 2`),
commits + pushes regenerated trust to the `nonos-trust-keystore` submodule,
and bumps the kernel pointer. This is a write/automation workflow, not a
gate; it is NOT replaced by ci-next and should be re-created separately if
auto-regeneration is still desired (out of scope for the read-only gate suite).

## security-audit.yml

`on: push [main, develop]`, `pull_request [main]`, weekly
`cron 23 5 * * 1`, `workflow_dispatch`.

- dependency-advisories (blocking): stable toolchain, `cargo install
  --locked cargo-audit` (cached), `cargo audit | tee target/security/
  audit.txt`, upload report. Vulnerabilities fail; unmaintained/unsound
  print only.
- dependency-policy (blocking): `cargo install --locked cargo-deny`,
  `cargo deny --all-features check licenses bans sources advisories`
  (config in `deny.toml`).
- clippy-security (blocking): nightly clippy on `--no-default-features
  --features microkernel-core` with `-Zbuild-std=core,alloc
  -Zbuild-std-features=compiler-builtins-mem`, denying
  `clippy::correctness suspicious integer_division cast_ptr_alignment
  transmute_ptr_to_ref transmute_ptr_to_ptr wildcard_dependencies`,
  warning `clippy::perf`.
- image-hygiene (blocking): scratch-trust-bootstrap (`proof-io ramfs
  keyring`), build `nonos-mk-libc ... nonos-mk-capsules`,
  `make nonos-mk-scan`, `bash nonos-ci/scan-microkernel-symbols.sh`,
  `bash nonos-ci/scan-binary-hygiene.sh`,
  `bash nonos-ci/check-pqclean-pin.sh`.
- manifest-presence (blocking, needs image-hygiene): build capsules,
  `objdump -h` the kernel ELF, require sections `.nonos.manifest` and
  `.nonos.sig`.
- audit-summary gated on all five blocking jobs.

## zk-ceremony.yml

`on: workflow_dispatch` with `action` (init/contribute/finalize/verify),
`circuit` (boot-authority/update-authority/recovery-key), `contributor_name`,
`ceremony_id`. Stable toolchain. Builds
`nonos-bootloader/tools/nonos-attestation-circuit` `run_ceremony` and drives
the trusted-setup MPC: init params, add a contribution (`--entropy system`),
finalize (requires >= 5 rounds, extracts `vk_boot_authority.bin`), verify a
transcript. Uploads round artifacts (retention 90, final VK 365). This is an
operator-driven ceremony tool, not a per-commit gate; like regenerate-trust
it is out of scope for the read-only ci-next gate suite and must be
re-created separately if still needed.
