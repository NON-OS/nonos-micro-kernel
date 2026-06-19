# capsule_driver_ahci

## Role

`capsule_driver_ahci` is the SATA AHCI controller capsule. Its job is to own
the AHCI PCI function in userland, expose the controller's identity and port
state over IPC, and keep SATA command policy out of the kernel.

This slice is a controller-probe milestone. It proves discovery, claim,
MMIO mapping, IRQ ownership, AHCI-mode enable, port signature reporting, and
live per-port status telemetry. It does not advertise a block device until
command-list, FIS, PRDT, DMA, and completion handling are implemented.

```text
signed capsule
    |
    | MkDeviceList / MkDeviceClaim
    v
AHCI PCI function -- MkMmioMap(BAR5 / ABAR) --> user VA
    |
    `-- MkIrqBind / MkIrqPoll / MkIrqAck --> controller events
```

## Microkernel contract

The capsule talks to hardware only through the broker:

- `MkDeviceList` locates SATA AHCI controller records.
- `MkDeviceClaim` binds the controller to this capsule's process.
- `MkMmioMap` maps BAR5, the AHCI ABAR register window.
- `MkIrqBind`, `MkIrqPoll`, and `MkIrqAck` own the controller interrupt.
- `MkIpcRecv` and `MkIpcSend` serve `driver.ahci0` on
  `service:4216:driver.ahci0`.

The kernel validates the capability token, owns address spaces, owns broker
revocation, and tears grants down on exit. It does not contain SATA command
logic, ATA identify logic, block scheduling, or filesystem policy.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_CONTROLLER_INFO` | AHCI global register summary | 24-byte controller record |
| `OP_PORT_LIST` | implemented ports, signatures, and live status | count plus 36-byte entries |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, and
`Irq` (`CAPSULE_REQUIRED_CAPS = 0x78018`). `Dma` is intentionally absent in
this slice because no command table, received-FIS area, or PRDT is submitted.

```text
allowed:   device enumeration, one device claim, ABAR MMIO, one IRQ, IPC
forbidden: DMA, PIO, filesystem, admin, debug, raw kernel memory
```

## Privacy and persistence

The capsule reads controller metadata: global capability registers, implemented
port bitmap, per-port signatures, and per-port status registers. It does not
read sectors, store disk payloads, cache partition data, or persist controller
state. All broker grants are process-lifetime resources and are revoked by
kernel teardown.

## Runtime lifecycle

The capsule discovers one AHCI controller, claims it, maps ABAR, binds the
controller interrupt, enables AHCI mode, snapshots port state, and then serves
IPC. Shutdown releases IRQ, unmaps ABAR, and releases the device claim.

## Failure model

Discovery, claim, MMIO map, IRQ bind, and AHCI-mode enable are hard setup
barriers. Any failure aborts startup and rolls back prior broker grants.
Runtime requests return protocol errors rather than touching ports that were
not discovered.

## Current implemented surface

- Claims the AHCI controller through the broker.
- Maps ABAR through `MkMmioMap`.
- Binds the controller interrupt.
- Enables AHCI mode and reads controller-global registers.
- Reports implemented ports, signatures, PxIS, PxCMD, PxTFD, PxSERR, PxSACT,
  and PxCI through the service endpoint.
- Fails closed when discovery, claim, MMIO, or IRQ binding fails.

## Wire format

Requests use the capsule's 20-byte protocol header with magic `NAHC`, version
`1`, operation id, request id, and payload length. Replies use the same header
shape and begin with a 4-byte status word. `OP_CONTROLLER_INFO` returns a
24-byte fixed register summary. `OP_PORT_LIST` returns a 4-byte count followed
by fixed 36-byte port records:

```text
u8 index, u8 implemented, u8 present, u8 kind,
u32 PxSSTS, u32 PxSIG, u32 PxIS, u32 PxCMD,
u32 PxTFD, u32 PxSERR, u32 PxSACT, u32 PxCI
```

## State ownership

The capsule owns the AHCI claim epoch, ABAR mapping, IRQ grant id, controller
snapshot, and port snapshot. The kernel owns only the broker records and
address-space mappings. No SATA state is mirrored into kernel process structs.

## Operating rules

- Do not expose a block endpoint until command DMA and completion handling are
  implemented.
- Do not add `Dma` to the manifest before command-list/FIS/PRDT setup exists.
- Keep partition, filesystem, encryption, and cache policy above this driver.
- Any setup failure must unwind IRQ, MMIO, and device claim in reverse order.

## Release target

The finished AHCI capsule is a signed, spawned storage-controller service that
identifies attached SATA devices, allocates command-list/FIS/PRDT DMA through
the broker, executes read/write/flush requests, handles error recovery and
device reset, and exposes block endpoints for each usable port. It remains a
driver only: partitions, filesystems, encryption, and cache policy stay in
separate storage capsules.

## Release evidence

Release requires an `ich9-ahci` boot validation, port signature proof, teardown
grant-revocation proof, and a read/write validation once command DMA lands.

## Release checklist

- Signed manifest and publisher keys present.
- Kernel mirror embeds and feature-gates `driver.ahci0`.
- QEMU controller probe passes on `ich9-ahci`.
- Teardown proof shows no leaked MMIO/IRQ/device claim.
- Block endpoint appears only after DMA command path is proven.

## Explicit non-goals today

No command-list setup, received-FIS area, PRDT, NCQ, ATA identify, read/write,
flush, partition parsing, filesystem, encryption policy, or disk cache lives in
this capsule yet.

## Verification

- Build: `make -B nonos-mk-driver-ahci`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: the capsule must not import `crate::drivers`,
  `crate::hardware`, `crate::memory`, `crate::paging`, or use inline PIO/DMA.
- Documentation check: this README is required by the static gate and must
  describe authority, privacy, current surface, release evidence, and non-goals.
