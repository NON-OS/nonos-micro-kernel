# boot_proofs

Host-runnable proofs for the bootloader's security-critical logic. The real
`security::anti_rollback` decision code and the real `image_format` footer
parser are included through `#[path]` and run on the host. Only the TPM and
NVRAM write is shimmed, so the invariants are proved about the code that gates a
kernel boot.

## Anti-rollback

The stored version floor is the mechanism that stops an attacker downgrading the
system to a kernel with a known vulnerability. The proofs establish:

- Version zero is always rejected, and without a TPM-backed or initialized
  anchor nothing boots.
- A version below the floor is rejected; a version at or above it is accepted.
- Booting a version raises the floor, after which no older version boots.
- A too-old boot is rejected and leaves the stored state untouched, because the
  check runs before any commit.
- The floor never decreases across updates, and setting the minimum only ever
  raises it.

Kani harnesses extend the core claims over every `u64`: acceptance is exactly
non-zero and at or above the floor, an update never lowers the floor, and no
older version is accepted after a successful boot.

## Image footer parser

The footer names the byte ranges of the kernel, signature, and proof regions and
is attacker-controlled. Over roughly 125,000 crafted footers with adversarial
region offsets and sizes, and a range of degenerate inputs, the parser never
panics and never returns a region slice that escapes the input buffer. A Kani
harness proves the parse is total for every input of the checked size.

## Run

```sh
cd nonos-bootloader/boot_proofs
cargo test --release
cargo kani                # all-input totality (requires Kani)
```
