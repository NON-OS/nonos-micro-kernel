# capsule_policy

## Role

`capsule_policy` is the system policy store. It runs as a CPL=3 capsule and
holds the small set of system-wide policy values (keyed records) that other
capsules read at startup and update at runtime: a typed key-value service
with a bootstrap set of defaults. It owns the policy table and is the single
source of truth for the values it serves; no policy is duplicated in the
kernel.

```text
desktop services / shell
        |
        | OP_GET (key) / OP_SET (key, value)
        v
capsule_policy -- in-memory policy table (bootstrap defaults)
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x219
```

The capsule serves callers with `MkIpcRecvFrom` plus `MkIpcSendToPid` and
terminates only through `MkExit`. It requests no hardware grants.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_GET` | key | current value, or not-found |
| `OP_SET` | key, value | accepted or rejected |

Unknown operations reply `E_BAD_OP`. Malformed bodies reply `E_INVAL`.
Keys outside the known policy set are rejected rather than stored.

## Authority

The capsule serves policy over IPC only. It has no PCI, MMIO, IRQ, DMA,
PIO, filesystem, network, display, or focus-routing authority.

## Privacy and persistence

The policy table lives in capsule memory and is seeded from the bootstrap
defaults on every boot. Values set at runtime do not persist across a
reboot.
