# Install to disk: composition under a machine root

The live USB already runs the full attested system. Installing is therefore
not compilation and not copying: it is choosing a composition, proving every
member of it, and enrolling that exact set under a root minted on this
machine. The result is an installation that is cryptographically unique to
its owner: the same binaries the vendor shipped, under a policy root that
exists nowhere else, with the vendor root retained for updates.

Nothing below requires new kernel machinery. Every step names the mechanism
that already exists in the tree.

## The flow

1. **Discover.** The wizard lists the available components from the live
   image itself: every `/capsules/*.manifest.bin` names a unit, its required
   capabilities and its attestation. The manifest catalog is the menu; the
   `.nonos-config` group structure (core, drivers, net, desktop, apps) is the
   shape the menu presents, so the on-device composer and the host
   `make menuconfig` speak one schema.

2. **Compose.** Selection screens in the wizard's existing frame. Hard
   dependencies stay selected (core, vfs, the display stack the wizard is
   running on); everything else is a choice. Each row shows the unit, its
   capability grant decoded, and its measurement.

3. **Prove.** Before anything is written, every selected capsule's shipped
   trailer is verified against the live image's policy root: the same
   verdict the kernel would reach at spawn, surfaced per unit in the UI. A
   unit that fails is not installable and says so. This step is display of
   fact, not ceremony: the spawn gate would refuse it later anyway; the
   installer refuses it earlier, with a face.

4. **Mint the machine root.** The wizard requests the kernel's local-build
   authority through `request_local_build_root`, and the user confirms
   through the `confirm_dev_root` consent step. From here the machine has
   its own root (`local_build::identity::root()`), held by the kernel, never
   exported.

5. **Enroll the composition.** For each selected capsule the kernel's
   `local_build::sign(elf, granted_caps)` mints a trailer binding this exact
   binary and this exact grant under the machine root. The vendor trailers
   are kept beside the local ones: the spawn gate already accepts either
   authority, so updates signed by the vendor keep working while the local
   root pins what this machine chose.

6. **Write.** Partition and format the target (the storage drivers and vfs
   the wizard is already running on), lay out the ESP exactly as the live
   image's own ESP is laid out, copy bootloader, kernel image, the selected
   capsules with both trailer sets, the roots. The bootloader's kernel
   self-attestation is untouched: the kernel is the vendor's kernel with the
   vendor's embedded trailer.

7. **The receipt.** The final screen shows the machine root in full, renders
   it as a QR through the qrgen capsule, and states what it means: this
   installation runs exactly the composition proven above, under a root that
   was created on this machine during this install. The QR carries the root
   so it can be checked against the attestation surface later.

## What is deliberately not in scope

- No re-measurement of binaries on device beyond what signing requires: the
  shipped trailers already bind measurements, and step 3 verifies them.
- No export of the machine secret. The root is public; the secret stays in
  the kernel's identity, as `local_build` already enforces.
- No new wire formats. Local trailers are the existing local-build trailer;
  the ESP layout is the shipping layout.

## Failure discipline

Every step is checkpointed and re-runnable. A failed write leaves the target
marked incomplete and the live system untouched. A refused proof in step 3
never reaches step 4. The consent step in 4 is the single point where the
user authorises a persistent change of trust state, and it says so in those
words.

## Sequencing

- Screen: discovery + composition list (manifest catalog, group shaping).
- Screen: per-unit proof verdicts.
- Kernel round trip: local root request + consent + per-unit signing.
- Disk: partition, ESP layout, copy, fsync, verify-read.
- Screen: root + QR receipt.

Each lands as its own reviewed change; the disk writer ships last and dark
until the whole path proves out in QEMU against a scratch disk.
