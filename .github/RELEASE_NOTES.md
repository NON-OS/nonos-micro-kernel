NØNOS 0.9.2 (pre-release)

A ZeroState microkernel image, signed and STARK-attested. The kernel carries
its own attestation trailer, every capsule is enrolled under the policy root
published beside it, and the whole trust chain is verified on the artifact
before it is uploaded. A tag that fails any gate publishes nothing.

New in this release: you can verify the kernel proof yourself, in a browser,
without trusting us or running anything. The verification gate is the same
code the bootloader runs before it jumps to the kernel, compiled to
WebAssembly. It loads the trailer and the boot root shipped with the image
and checks the STARK in your tab. Flip one bit of the proof and it refuses.
Live at verify.nonos.software; source at github.com/NON-OS/stark-attest.

Attestation

- The kernel self-attests. The bootloader will not jump to a kernel whose
  trailer does not verify against the root baked into the boot chain, and the
  proof binds the kernel measurement and the boot epoch, so an old image
  cannot answer for a new root.
- Every userland capsule is enrolled under one policy root. The kernel refuses
  to spawn a capsule whose attestation does not verify, and the proof binds the
  capsule measurement, the exact capabilities it was granted, and the policy
  epoch. A swapped binary, an over-grant of capabilities, and a replayed epoch
  all fail the same check.
- The proof system is transparent and post-quantum. It rests on a hash and a
  low-degree test, with no trusted setup and no ceremony.

Networking, browser, desktop

- Nym routing is reliable again. The directory keeps re-syncing until it
  carries both an exit and a gateway, the gateway list is retried the way the
  exit list already was, and the relay poll window is sized so mixnet replies
  are caught without starving the browser.
- The SOCKS5 relay poll was shortened so the browser stays responsive while a
  request is in flight over the mixnet.
- IPC service replies now land in the caller reply inbox rather than the
  process inbox, and virtio-blk ignores its own reply loopback in the serve
  loop, which is what had been wedging the desktop on boot.
- The TLS client only treats a server flight as complete on a clean record
  boundary, and pins ISRG Root X2.

Hardening

- swapgs runs on every vector a capsule can enter the kernel through, and the
  syscall path rejects every non-canonical return address before sysretq
  rather than only half of them.
- Every fault stack has a guard page under it, an application processor loads
  the IDT the kernel actually runs on when it comes up, and the kernel refuses
  a capsule that asks for an interrupt line the kernel itself listens on.

Build

- One curated make surface. The default build produces the image, restamps the
  trust ledger, and re-verifies the built artifact against the roots embedded
  in it before calling itself done.

Verifying what you downloaded

Check the files against SHA256SUMS. Check the policy roots in this release
against the ones your running image reports. The kernel measurement in the
attestation is a BLAKE3 hash you can reproduce with b3sum against the image.

This is a pre-release. It boots and attests on the hardware and emulators we
test, but it is not a general-availability build, and the STARK instance has
not had an independent cryptanalysis. Treat it as what it is: a real system
you can check for yourself, still early.
