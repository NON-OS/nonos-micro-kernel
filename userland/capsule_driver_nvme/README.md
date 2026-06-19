# capsule_driver_nvme

## Role

`capsule_driver_nvme` is the NVMe controller capsule. It moves NVMe controller
logic out of the kernel and into a signed userland process that receives only
the hardware authority it needs.

The current production slice reaches the admin queue: it claims the PCI NVMe
device, maps BAR0, binds MSI-X, allocates broker DMA for the admin submission
queue, admin completion queue, and admin data buffer, enables the controller,
issues Identify Controller plus Identify Namespace for NSID 1, and snapshots
the controller SMART / health log.

```text
driver.nvme0
    |
    | MkDeviceClaim + MkPciConfigWrite(bus master)
    v
NVMe PCI function
    |
    +-- MkMmioMap(BAR0) ----------> controller registers
    +-- MkIrqBind(MSI-X) ---------> admin completion interrupt
    `-- MkDmaMap -----------------> ASQ / ACQ / identify buffer
```

## Microkernel contract

The capsule uses the microkernel as mechanism, not as an NVMe driver:

- `MkDeviceList` finds PCI class `0x010802`.
- `MkDeviceClaim` owns the controller claim and claim epoch.
- `MkPciConfigWrite` enables bus mastering through the broker.
- `MkMmioMap` maps BAR0 controller registers.
- `MkIrqBind`, `MkIrqPoll`, and `MkIrqAck` own the MSI-X interrupt path.
- `MkDmaMap` and `MkDmaUnmap` allocate and revoke admin queue DMA.
- `MkIpcRecv` and `MkIpcSend` serve `driver.nvme0` on
  `service:4220:driver.nvme0`.

The kernel never embeds NVMe opcodes, queue policy, namespace interpretation,
or block I/O. It validates the token, grants resources, routes IPC, and
revokes every grant on exit.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_CONTROLLER_INFO` | BAR/register and setup snapshot | 52-byte controller record |
| `OP_IDENTIFY_CONTROLLER` | selected Identify Controller fields | 88-byte identity record |
| `OP_IDENTIFY_NAMESPACE` | selected Identify Namespace fields for NSID 1 | 36-byte namespace record |
| `OP_SMART_HEALTH` | selected Get Log Page SMART / health fields | 177-byte health record |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0xF8018`). It has no filesystem, storage
policy, admin, debug, network, or raw physical-memory authority.

```text
allowed:   PCI claim, BAR0 registers, MSI-X, broker DMA, IPC
forbidden: filesystem policy, partition policy, raw physmem, kernel drivers
```

## Privacy and persistence

The capsule currently reads controller identity, namespace identity, and the
standard controller health log only. It does not read user sectors, write media,
parse filesystems, or persist metadata. Queue memory and admin buffers are
broker DMA grants and are revoked when the capsule exits.

## Runtime lifecycle

The capsule discovers one NVMe PCI function, claims it, enables bus mastering,
maps BAR0, binds MSI-X, allocates admin queue DMA, disables the controller,
programs AQA/ASQ/ACQ, enables the controller, runs identify commands, reads the
SMART / health log, and then serves IPC. Teardown unmaps DMA, unbinds IRQ,
unmaps MMIO, and releases the device claim.

## Failure model

Every setup phase is a barrier with reverse-order rollback. Controller timeout,
admin completion error, stale claim, MSI-X bind failure, or DMA allocation
failure prevents service start. Runtime identity requests return only data that
was captured successfully during setup.

## Current implemented surface

- Claims a real NVMe PCI function.
- Enables bus mastering through brokered PCI config write.
- Maps controller registers.
- Binds MSI-X for admin completion.
- Allocates and zeroes admin queue DMA through the broker.
- Programs AQA/ASQ/ACQ and enables the controller.
- Issues Identify Controller.
- Issues Identify Namespace for NSID 1 when the controller reports namespaces.
- Issues Get Log Page for the standard SMART / health log.
- Exposes controller and namespace identity over IPC.
- Exposes selected health counters over IPC without exposing raw log DMA.

## Wire format

Requests use the `NNVM` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. Controller-info
returns 52 bytes. Identify Controller returns 88 bytes of selected fields.
Identify Namespace returns 36 bytes for NSID 1. Raw 4096-byte identify pages
remain internal DMA data unless a later protocol explicitly exposes them.
SMART / health returns 177 bytes of selected fields, including the controller
warning bits, composite temperature, spare percentage, lifetime counters, media
errors, and error-log count.

## State ownership

The capsule owns the controller claim epoch, BAR0 mapping, MSI-X grant, admin
submission queue, admin completion queue, identify DMA buffer, controller
snapshot, and namespace snapshot. The kernel owns capability validation, grant
records, IRQ routing, and teardown only.

## Operating rules

- Do not expose a block endpoint until IO queues and PRP/SGL data movement are
  implemented.
- Keep namespace and controller command logic inside the capsule.
- Do not parse partitions, filesystems, or encrypted volume headers here.
- Every setup phase must have reverse-order rollback.

## Release target

The finished NVMe capsule is a signed block-controller service with admin and
IO queue pairs, namespace scan, PRP/SGL data movement, read/write/flush
commands, timeout/error recovery, MSI-X completion handling, teardown rollback,
and a block endpoint per usable namespace. It does not parse partitions,
filesystems, encryption headers, or application data.

## Release evidence

Release requires QEMU `-device nvme` identify validation, IO queue creation validation,
single read/write/flush proof, PRP/SGL boundary tests, teardown DMA revocation,
and one real NVMe controller boot.

## Release checklist

- Signed manifest and publisher trust entries present.
- Kernel mirror embeds and feature-gates `driver.nvme0`.
- QEMU identify validation reports controller and NSID 1.
- IO queue creation and single read/write/flush validation pass.
- PRP/SGL boundary tests pass.
- Teardown proof shows admin and IO DMA grants are revoked.

## Explicit non-goals today

No IO queue pairs, PRP/SGL read/write path, flush, discard, namespace scanning
beyond NSID 1, multipath, partition table, filesystem, encryption, or block
service endpoint is exposed yet.

## Verification

- Build: `make -B nonos-mk-driver-nvme`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: NVMe must not import kernel driver or memory internals;
  it must use `MkMmioMap`, `MkIrqBind`, and `MkDmaMap`.
- Broker check: setup rollback must unmap DMA, unbind IRQ, unmap MMIO, and
  release the device claim on failure.
- Documentation check: the static gate requires this README and its contract,
  authority, lifecycle, failure model, release evidence, and verification sections.
