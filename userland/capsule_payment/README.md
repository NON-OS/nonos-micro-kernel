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
        | OP_PAY (publisher addr, amount) / OP_SIGN_RECEIPT
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
| `OP_SIGN_RECEIPT` | receipt id | keyring-signed receipt bytes |

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
