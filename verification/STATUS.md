# NØNOS Verification Status

Last local run: 2026-07-07, from a clean checkout of `main`.

## Runnable Proofs

Command:

```sh
for c in crypto_proofs net_proofs kernel_proofs driver_proofs usb_proofs \
         fs_proofs stark_proofs nvme_proofs usb_msc_proofs \
         virtio_net_proofs xhci_proofs; do
  ( cd userland/$c && PATH="$HOME/.cargo/bin:$PATH" cargo test --release )
done
( cd nonos-bootloader/boot_proofs && PATH="$HOME/.cargo/bin:$PATH" cargo test --release )
```

Result:

```text
crypto_proofs      56 passed; 0 failed
net_proofs         10 passed; 0 failed
kernel_proofs      23 passed; 0 failed
driver_proofs       2 passed; 0 failed
usb_proofs          2 passed; 0 failed
fs_proofs          65 passed; 0 failed
stark_proofs       12 passed; 0 failed
nvme_proofs        10 passed; 0 failed
usb_msc_proofs     14 passed; 0 failed
virtio_net_proofs   8 passed; 0 failed
xhci_proofs         8 passed; 0 failed
boot_proofs         9 passed; 0 failed
```

Two of these needed repair before they reproduced: `net_proofs` stopped
compiling when the TCP reassembly module became a directory, and
`virtio_net_proofs` stopped compiling when the driver gained a negotiated
buffer count and moved the RX refill into an explicit `refill_consumed`
call. Both are `#[path]` divergences doing exactly what the design intends,
a build error instead of a silently stale proof, but neither crate runs in
CI today, so the breakage went unnoticed until this run. The repairs updated
the harnesses to the current production contract; no production source
changed.

## Kani

Command:

```sh
( cd userland/fs_proofs && PATH="$HOME/.cargo/bin:$PATH" cargo kani --output-format terse )
( cd userland/kernel_proofs && PATH="$HOME/.cargo/bin:$PATH" cargo kani --output-format terse )
```

Result:

```text
fs_proofs:     Complete - 7 successfully verified harnesses, 0 failures, 7 total.
kernel_proofs: Complete - 7 successfully verified harnesses, 0 failures, 7 total.
```

fs_proofs harnesses:

- `proof_decode_request_total`: all bounded request byte strings either decode safely or return a decode error.
- `proof_split_caller_no_impersonation`: a non-kernel sender cannot claim a different caller pid through the VFS payload.
- `proof_normalize_short_invariants`: bounded hostile path bytes normalize to an absolute canonical path without slash, dot, parent, or trailing slash defects.
- `proof_normalize_branch_cases`: parent-dir, duplicate slash, dot, trailing slash, relative path, and empty path branches preserve canonical invariants.
- `proof_eth_parse_total`: bounded Ethernet frames never panic and accepted frames expose only in-bounds payloads.
- `proof_ipv4_parse_total`: bounded IPv4 packets never panic and accepted packets expose only in-bounds payloads.
- `proof_udp_parse_total`: bounded UDP segments never panic and accepted segments expose only in-bounds payloads.

kernel_proofs harnesses:

- `wx_isolation_holds_for_all_permissions`: no permission set that rejects W^X encodes a writable-executable page.
- `syscall_decode_is_total`: decoding an untrusted syscall id never panics, over all u64.
- `syscall_id_decode_and_registry_agree_for_all_ids`: the decoder and the registry agree for every id.
- `user_range_check_is_total_and_bounded`: the kernel-to-user copy guard is total and accepted ranges stay in user space.
- `capability_bits_equal_the_spec_for_all_tokens`: the kernel capability bit operations equal the specification for every token.
- `check_range_equals_the_spec_for_all_inputs`: the user-copy guard equals its specification for all inputs.
- `pte_encoding_equals_the_spec_for_all_permissions`: the page-table encoding equals its specification for all permissions.

## Verus

Command:

```sh
cd verification/verus
verus --crate-type=lib src/lib.rs    # release/0.2026.06.28
```

Result:

```text
verification results:: 25 verified, 0 errors
```

Theorems:

- `revoke_is_monotonic`: revocation cannot create authority.
- `revoke_drops_the_right`: revoking a bit removes that bit.
- `attenuation_confines`: attenuation cannot grant rights absent from the parent token.
- `grant_preserves_and_adds`: granting preserves existing rights and grants the requested nonzero right.
- `empty_token_grants_nothing`: an empty token grants no right.
- `zero_length_is_rejected`: zero-byte IPC messages fail the shared IPC length gate.
- `oversized_message_is_rejected`: messages over `MAX_MESSAGE_SIZE` fail the shared IPC length gate.
- `accepted_message_is_bounded`: accepted IPC lengths are nonzero and at most `MAX_MESSAGE_SIZE`.
- `send_reply_share_the_same_length_gate`: send and reply use the same `1..=1048576` message-size rule.
- `encoded_pte_is_present`: encoded page-table entries are present.
- `writable_bit_matches_permission`: encoded writable permission matches the source permission.
- `user_bit_matches_permission`: encoded user permission matches the source permission.
- `executable_bit_matches_permission`: encoded NX state is the inverse of source executable permission.
- `no_wx_when_permission_rejects_wx`: non-WX permissions do not encode writable-executable pages.
- `permission_subset_is_monotonic`: a permission subset cannot have write, user, or execute authority absent from the parent.

## Lean

Command:

```sh
cd verification/lean
PATH="$HOME/.elan/bin:$PATH" lake build
```

Result:

```text
Build completed successfully.    # 0 errors, 111 theorems across 18 modules
grep -rE 'sorry|admit|^axiom' Nonos.lean Nonos/    # no proof-bearing matches
```

Modules: AntiRollback, Authorization, BlockIO, BootImage, Capability,
CapabilityBits, Crypto, Ipc, Isolation, Loader, NetParse, Path, Paging,
Secure, Stark.Field, Stark.Merkle, Syscall, UsbHid. Every theorem names the
code proof that discharges it; the mapping table is in `lean/README.md`.

## CI

The `lean` job (`.github/workflows/lean.yml`) runs `lake build` on every
push. The runnable, Kani, and Verus layers have no CI gate on `main` today;
restoring them is tracked in PR #311. Everything above was reproduced
locally with the commands shown.

## CI Pinning

- Kani GitHub Action: `model-checking/kani-github-action@v1`, pinned input `kani-version: "0.67.0"`.
- Verus release: `release/0.2026.06.28.1847ab3`.
- Verus Linux asset: `verus-0.2026.06.28.1847ab3-x86-linux.zip`.
