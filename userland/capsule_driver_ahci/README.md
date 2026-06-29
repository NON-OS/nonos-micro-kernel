# capsule_driver_ahci

## Role

`capsule_driver_ahci` is the SATA AHCI controller capsule. Its job is to own
the AHCI PCI function in userland, expose the controller's identity and port
state over IPC, and keep SATA command policy out of the kernel.

This slice is a block-controller milestone. It proves discovery, claim, MMIO
mapping, IRQ ownership, AHCI-mode enable, ATA identify, command-list/FIS/PRDT
DMA setup, read/write transfer, flush, and live per-port status telemetry.

```text
signed capsule
    |
    | MkDeviceList / MkDeviceClaim
    v
AHCI PCI function -- MkMmioMap(BAR5 / ABAR) --> user VA
    |
    +-- MkIrqBind / MkIrqPoll / MkIrqAck --> controller events
    `-- MkDmaMap -------------------------> command/FIS/PRDT/data buffers
```

## Microkernel contract

The capsule talks to hardware only through the broker:

- `MkDeviceList` locates SATA AHCI controller records.
- `MkDeviceClaim` binds the controller to this capsule's process.
- `MkMmioMap` maps BAR5, the AHCI ABAR register window.
- `MkIrqBind`, `MkIrqPoll`, and `MkIrqAck` own the controller interrupt.
- `MkDmaMap` and `MkDmaUnmap` allocate command-list, received-FIS, command
  table, PRDT, and sector data buffers.
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
| `OP_CAPACITY` | selected block port capacity | status plus sector count |
| `OP_READ_BLOCKS` | read sectors from selected block port | status plus sector bytes |
| `OP_WRITE_BLOCKS` | write sectors to selected block port | status word |
| `OP_FLUSH` | flush selected block port | status word |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0xf8019`).

```text
allowed:   device enumeration, one device claim, ABAR MMIO, IRQ, DMA, IPC
forbidden: PIO, filesystem, admin, debug, raw kernel memory
```

## Privacy and persistence

The capsule reads and writes sector payloads only for explicit block protocol
requests. It does not parse partitions, mount filesystems, cache disk payloads,
or persist controller state. All broker grants are process-lifetime resources
and are revoked by kernel teardown.

## Runtime lifecycle

The capsule discovers one AHCI controller, claims it, maps ABAR, binds the
controller interrupt, enables AHCI mode, identifies the first usable SATA port,
allocates command/data DMA, and then serves IPC. Shutdown releases DMA, IRQ,
ABAR, and the device claim.

## Failure model

Discovery, claim, MMIO map, IRQ bind, AHCI-mode enable, DMA allocation, ATA
identify, and command setup are hard setup barriers. Any failure aborts startup
and rolls back prior broker grants. Runtime requests return protocol errors
rather than touching ports that were not discovered.

## Current implemented surface

- Claims the AHCI controller through the broker.
- Maps ABAR through `MkMmioMap`.
- Binds the controller interrupt.
- Enables AHCI mode and reads controller-global registers.
- Identifies the selected SATA port and records block capacity.
- Allocates command-list, received-FIS, command-table, PRDT, and data DMA.
- Serves capacity, read, write, and flush requests over IPC.
- Reports implemented ports, signatures, PxIS, PxCMD, PxTFD, PxSERR, PxSACT,
  and PxCI through the service endpoint.
- Fails closed when discovery, claim, MMIO, IRQ, DMA, or identify setup fails.

## Wire format

Requests use the capsule's 20-byte protocol header with magic `NAHC`, version
`1`, operation id, request id, and payload length. Replies use the same header
shape and begin with a 4-byte status word. `OP_CONTROLLER_INFO` returns a
24-byte fixed register summary. `OP_CAPACITY` returns an 8-byte sector count.
`OP_READ_BLOCKS` and `OP_WRITE_BLOCKS` use a 12-byte `lba, sector_count`
request header and fixed 512-byte sectors. `OP_PORT_LIST` returns a 4-byte
count followed by fixed 36-byte port records:

```text
u8 index, u8 implemented, u8 present, u8 kind,
u32 PxSSTS, u32 PxSIG, u32 PxIS, u32 PxCMD,
u32 PxTFD, u32 PxSERR, u32 PxSACT, u32 PxCI
```

## State ownership

The capsule owns the AHCI claim epoch, ABAR mapping, IRQ grant id, DMA grants,
controller snapshot, port snapshot, command state, and data buffer. The kernel
owns only the broker records and address-space mappings.

## Operating rules

- Keep partition, filesystem, encryption, and cache policy above this driver.
- Any setup failure must unwind DMA, IRQ, MMIO, and device claim in reverse order.

## Release target

The next AHCI target is broader validation: NCQ where supported, multi-port
selection, timeout recovery, device reset, and repeated real-controller boot
evidence. It remains a driver only: partitions, filesystems, encryption, and
cache policy stay in separate storage capsules.

## Release evidence

Release requires `ich9-ahci` boot validation, port signature proof, teardown
grant-revocation proof, read/write/flush validation, and one real SATA
controller boot dossier.

## Release checklist

- Signed manifest and publisher keys present.
- Kernel mirror embeds and feature-gates `driver.ahci0`.
- QEMU controller probe passes on `ich9-ahci`.
- Teardown proof shows no leaked MMIO/IRQ/device claim.
- Read/write/flush proof passes through the IPC block endpoint.

## Explicit non-goals today

No NCQ, multi-port policy, partition parsing, filesystem, encryption policy,
or disk cache lives in this capsule.

## Verification

- Build: `make -B nonos-mk-driver-ahci`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: the capsule must not import `crate::drivers`,
  `crate::hardware`, `crate::memory`, `crate::paging`, or use inline PIO/DMA.
- Documentation check: this README is required by the static gate and must
  describe authority, privacy, current surface, release evidence, and non-goals.
