# capsule_installer

## Role

`capsule_installer` is the userland install authority. It runs as a CPL=3
capsule and gates paid install admission on a keyring-signed payment receipt.
It can also ask the kernel to load a signed, attested capsule artifact set from
the VFS store through the same verified spawn path used by baked capsules. It
owns no storage device; it is the policy point that binds payment to admission
and load.

```text
marketplace client
        |
        | OP_INSTALL / OP_LOAD_FROM_STORE
        v
capsule_installer -- OP_PAY --> capsule_payment -- sign --> keyring
        |
        `-- MkCapsuleLoad --> verified spawn from VFS artifacts
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x10019
```

The capsule resolves the payment capsule with `MkServiceLookup`, drives
settlement with `MkIpcCall`, serves callers with `MkIpcRecvFrom` plus
`MkIpcSendToPid`, invokes `MkCapsuleLoad` for VFS-backed capsule artifacts, and
terminates only through `MkExit`. It carries the `Driver` bit only because the
current syscall contract gates `MkCapsuleLoad` as a driver-class authority.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | liveness |
| `OP_INSTALL` | owner pid, wallet id, price kind, capsule id, publisher, amount, receipt type | free admission hash or signed payment receipt hash |
| `OP_LOAD_FROM_STORE` | requested caps, artifact lengths, ELF/cert/manifest/ZK trailer bytes | spawned pid |

Unknown operations reply `E_BAD_OP`. Malformed bodies reply `E_INVAL`.
An install request without a verified receipt is refused.

## Authority

The capsule may talk to the payment capsule over IPC and may invoke
`MkCapsuleLoad`. It has no PCI, MMIO, IRQ, DMA, PIO, network, display, or
focus-routing authority.

## Privacy and persistence

The installer keeps only the in-flight install/receipt mapping in capsule
memory for the life of the boot. It retains no caller identity.
