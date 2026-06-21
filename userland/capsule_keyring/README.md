# capsule_keyring

## Role

`capsule_keyring` is the in-memory key management capsule. It stores key
records for requesting capsules and provides lock, unlock, retrieve, metadata,
delete, and count operations over IPC.

```text
authorized client
    |
    | keyring IPC
    v
keyring -- locked in-memory store --> key records
    |
    `-- reply / error
```

## Microkernel contract

The capsule uses the IPC mechanism only:

- `MkIpcRecv` receives requests on `service:4098:keyring`.
- `MkIpcSend` returns replies on `reply:4099:endpoint.4294967298`.
- `MkExit` terminates on fatal setup failure.
- The kernel mirror is `src/security/keyring_capsule`.

The kernel does not store user keys, expose a kernel key table, or perform
keyring policy. It routes IPC and enforces the capsule's signed manifest.

## Interface contract

| Surface | Purpose |
|---|---|
| store | insert caller-owned key material into volatile memory |
| retrieve | return key material only through the keyring protocol |
| lock/unlock | transition the key store between usable and sealed runtime states |
| metadata/delete/count | inspect or remove records without kernel key storage |

## Authority

The manifest grants `IPC`, `Memory`, and service authority through
`CAPSULE_REQUIRED_CAPS = 0x38`. It has no device, MMIO, IRQ, DMA, PIO,
filesystem, network, admin, or debug authority.

## Privacy and persistence

Key material remains in capsule memory. The current capsule is volatile: it
does not write keys to disk or keep state across reboot. The lock state is
runtime state and disappears with the process.

## Runtime lifecycle

The capsule starts with an empty volatile store, accepts key operations over
IPC, enforces lock state, and drops all key records on exit.

## Failure model

Invalid handles, locked state, capacity exhaustion, and malformed requests
return protocol errors. The kernel never mirrors key bytes for recovery.

## Current implemented surface

- Owns key records in a capsule-local store.
- Supports store, retrieve, metadata, delete, count, lock, and unlock flows.
- Supports Ethereum wallet import, generation, address derivation, NOX receipt
  signing, NOX ERC-20 approval signing, and native ETH EIP-1559 transfer signing.
- Lists wallet rails so UI and policy can distinguish enabled NONOS Wallet rails
  from reserved external wallet tracks.
- Is embedded, spawned, and validation-covered through the kernel mirror.
- Keeps key storage out of kernel memory.

## Wire format

The keyring protocol carries operation id, caller-owned key handle or label,
metadata fields, and optional key bytes. Replies carry status, metadata, or key
bytes depending on the operation. Layout ownership lives in `src/protocol`.

Wallet operations:

| Operation | Input | Output |
|---|---|---|
| `OP_WALLET_IMPORT` | caller pid, 32-byte secp256k1 secret | wallet id |
| `OP_WALLET_GENERATE` | caller pid | wallet id |
| `OP_WALLET_ADDRESS` | caller pid, wallet id | Ethereum address |
| `OP_SIGN_NOX_RECEIPT` | caller pid, wallet id, receipt fields | recoverable EIP-712 signature |
| `OP_SIGN_NOX_APPROVE` | caller pid, wallet id, nonce/fee/gas/amount | raw signed EIP-1559 NOX approval transaction |
| `OP_SIGN_ETH_TRANSFER` | caller pid, wallet id, recipient, nonce/fee/gas/value | raw signed EIP-1559 ETH transfer transaction |
| `OP_LIST_WALLET_RAILS` | none | supported and reserved wallet rails |

`OP_LIST_WALLET_RAILS` returns:

```text
u32 count
repeat count:
  u8  symbol_len
  u8  family
  u16 status
  u32 flags
  u64 chain_id
  u8  contract_address[20]
  u8  symbol[symbol_len]
```

Enabled rails are ETH and NOX. PR is config-required. SAL is a separate
Salvium wallet track and is reported reserved until the native Salvium wallet
core capsule is ported and tested.

## State ownership

The capsule owns key records, metadata, lock state, and deletion state. The
kernel owns neither key bytes nor key handles.

## Operating rules

- Zeroize records on delete and teardown where storage representation permits.
- Enforce locked state before returning secret material.
- Keep persistence out until a sealed storage policy exists.
- Never mirror keys into kernel service state.

## Release target

The finished keyring capsule has caller-scoped key ownership, locked-state
enforcement, bounded storage, zeroization on delete and teardown, validation checks
for every operation, and no kernel-resident secret table. Persistence, if added
later, must be explicit and sealed by a separate storage policy.

## Release evidence

Release evidence is validation coverage for store/retrieve/delete/count/lock/unlock,
plus teardown proof that key records do not persist in kernel state.

## Release checklist

- Store/retrieve/delete/count/lock/unlock validation passes.
- Locked-state denial is tested.
- Teardown clears volatile records.
- Static gate confirms no kernel key table exists.

## Explicit non-goals today

No hardware secure element, TPM sealing, persistent vault, remote sync,
password UI, filesystem persistence, or crypto primitive implementation lives
inside this capsule.

## Verification

- Build: `make -B nonos-mk-keyring`
- Validation: `nonos-mk-keyring-test`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Privacy check: key data must stay in capsule-owned memory and never move
  into kernel-resident service state.
