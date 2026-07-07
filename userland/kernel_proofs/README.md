# kernel_proofs

Host-runnable proofs for the kernel's memory isolation and authorization
boundary. The real page-permission, user-copy, syscall-decode, capability, and
ELF-loader source is included through `#[path]` and run directly, so the
invariants are proved about the code that enforces them.

## W^X

A page must never be both writable and executable. The proof works on the real
`to_pte_flags` encoding: a permission set that is not a write-execute violation
never encodes a page-table entry that is simultaneously writable and executable,
so a mapper that rejects `is_wx_violation` cannot install a W+X page. The flag
encoding (present, writable, user, no-execute) is also proved faithful to the
permission bits. Runnable and by Kani over all permission patterns.

## User-copy bounds

`check_range` guards every copy between the kernel and userspace. It is proved
total, and an accepted range is page aligned and lies inside user space without
wrapping. Null pointers, oversized lengths, addresses past user space, and
overflowing ranges are all rejected. Runnable and by Kani over all addresses and
lengths.

## Syscall decode

An untrusted `u64` syscall id crosses the kernel boundary. Decoding it is proved
total (no value panics), the id table stays consistent with the name table, and
known ids round trip through their numeric value. Kani proves totality over all
`u64`.

## Authorization

The real `is_allowed` capability table is proved to deny an empty token every
syscall, to permit a crypto syscall only for a token that grants the crypto
capability, and to never remove access when a capability is added. Capability
ids are shown to occupy distinct single bits.

## ELF loader

The capsule loader parses attacker-controlled ELF. A truncated header is
rejected, and an accepted program-header table fits inside the file with no
integer overflow in `phoff + phnum * phentsize`, over adversarial headers with
large offsets and counts.

## Run

```sh
cd userland/kernel_proofs
cargo test --release
cargo kani                # all-input isolation and decode checks (requires Kani)
```
