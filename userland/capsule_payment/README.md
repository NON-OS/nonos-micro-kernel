# capsule_payment

## Role

`capsule_payment` is the userland payment authority. It runs as a CPL=3
capsule and issues signed NOX install receipts: a caller asks it to settle
an install, the capsule records the charge against a monotonic nonce, and
it returns a receipt signed by the keyring so the installer can verify
payment without trusting the requester. It owns the payment nonce, the
pending outbox, and the per-publisher settlement state.

```text
installer / marketplace client
        |
        | OP_PAY (publisher addr, amount) / OP_DRAIN_RECEIPTS
        v
capsule_payment -- MkIpcCall --> keyring (secp256k1 receipt signature)
        |
        `-- nonce + outbox state
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x18
```

The capsule resolves the keyring with `MkServiceLookup`, requests receipt
signatures with `MkIpcCall`, serves callers with `MkIpcRecvFrom` plus
`MkIpcSendToPid`, and terminates only through `MkExit`. It requests no
hardware grants.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | liveness, nonce high-water mark |
| `OP_PAY` | publisher address, amount | charge accepted, receipt id |
| `OP_DRAIN_RECEIPTS` | cursor/count | pending keyring-signed receipt bytes |
| `OP_LIST_TOKENS` | none | supported wallet/payment assets |

Unknown operations reply `E_BAD_OP`. Malformed bodies reply `E_INVAL`.

## Authority

The capsule may talk to the keyring over IPC. It has no PCI, MMIO, IRQ,
DMA, PIO, network, display, or focus-routing authority. It never moves
funds itself; settlement authority is the publisher Ethereum address
carried in the signed receipt and reconciled on chain.

## Privacy and persistence

The nonce and outbox live in capsule memory for the life of the boot.
Receipts carry only the publisher address, amount, and nonce; no caller
identity is retained.

## Token registry

`OP_LIST_TOKENS` returns the payment capsule's asset registry as a compact
binary payload:

```text
u32 count
repeat count:
  u8  symbol_len
  u8  decimals
  u16 settlement_kind
  u32 flags
  u64 chain_id
  u8  contract_address[20]
  u8  symbol[symbol_len]
```

Settlement kinds:

| Value | Meaning |
|---:|---|
| 1 | Native ETH |
| 2 | NOX receipt settlement |
| 3 | Primer x402 settlement |

Flags:

| Bit | Meaning |
|---:|---|
| `1 << 0` | enabled |
| `1 << 1` | native token |
| `1 << 2` | ERC-20 token |
| `1 << 3` | settled by local receipt |
| `1 << 4` | settled by x402/Primer rail |
| `1 << 5` | contract/configuration required before live settlement |

Current built-ins:

| Symbol | Chain | Contract | Status |
|---|---:|---|---|
| ETH | 1 | native | enabled |
| NOX | 1 | `0x0a26c80Be4E060e688d7C23aDdB92cBb5D2C9eCA` | enabled |
| PR | 8453 | pending | reserved for Primer x402; requires verified contract/config |
