# capsule_installer

## Role

`capsule_installer` is the userland install authority. It runs as a CPL=3
capsule and gates every capsule install on a verified payment receipt:
a caller asks it to install a marketplace entry, the installer settles the
charge through the payment capsule, checks the returned receipt, and only
then admits the install. It owns no storage device; it is the policy point
that binds payment to admission.

```text
marketplace client
        |
        | OP_INSTALL (entry id)
        v
capsule_installer -- OP_PAY --> capsule_payment -- sign --> keyring
        |
        `-- admit install only on a verified receipt
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x18
```

The capsule resolves the payment capsule with `MkServiceLookup`, drives
settlement with `MkIpcCall`, serves callers with `MkIpcRecvFrom` plus
`MkIpcSendToPid`, and terminates only through `MkExit`. It requests no
hardware grants.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | liveness |
| `OP_PAY` | entry id, publisher address | settlement result, receipt id |
| `OP_INSTALL` | entry id, receipt id | install admitted or rejected |

Unknown operations reply `E_BAD_OP`. Malformed bodies reply `E_INVAL`.
An install request without a verified receipt is refused.

## Authority

The capsule may talk to the payment capsule over IPC. It has no PCI, MMIO,
IRQ, DMA, PIO, network, display, or focus-routing authority.

## Privacy and persistence

The installer keeps only the in-flight install/receipt mapping in capsule
memory for the life of the boot. It retains no caller identity.
