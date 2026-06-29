# capsule_driver_nvme

## Role

`capsule_driver_nvme` is the NVMe controller capsule. It moves NVMe controller
logic out of the kernel and into a signed userland process that receives only
the hardware authority it needs.

The current production slice reaches the admin queue and one NVM IO queue pair:
it claims the PCI NVMe device, maps BAR0, binds MSI-X, allocates broker DMA for
admin queues, IO queues, PRP list, and data buffers, enables the controller,
issues Identify Controller plus Identify Namespace for NSID 1, snapshots the
controller SMART / health log, and serves read/write/flush block requests.

```text
driver.nvme0
    |
    | MkDeviceClaim + MkPciConfigWrite(bus master)
    v
NVMe PCI function
    |
    +-- MkMmioMap(BAR0) ----------> controller registers
    +-- MkIrqBind(MSI-X) ---------> completion interrupt
    `-- MkDmaMap -----------------> admin queues / IO queues / data buffers
```

## Microkernel contract

The capsule uses the microkernel as mechanism, not as an NVMe driver:

- `MkDeviceList` finds PCI class `0x010802`.
- `MkDeviceClaim` owns the controller claim and claim epoch.
- `MkPciConfigWrite` enables bus mastering through the broker.
- `MkMmioMap` maps BAR0 controller registers.
- `MkIrqBind`, `MkIrqPoll`, and `MkIrqAck` own the MSI-X interrupt path.
- `MkDmaMap` and `MkDmaUnmap` allocate and revoke admin queue, IO queue, PRP,
  identify, health, and sector data DMA.
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
| `OP_CAPACITY` | selected namespace capacity | status plus sector count |
| `OP_READ_BLOCKS` | read sectors from NSID 1 | status plus sector bytes |
| `OP_WRITE_BLOCKS` | write sectors to NSID 1 | status word |
| `OP_FLUSH` | flush NSID 1 | status word |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0xF8019`). It has no filesystem, storage
policy, admin, debug, network, or raw physical-memory authority.

```text
allowed:   PCI claim, BAR0 registers, MSI-X, broker DMA, IPC
forbidden: filesystem policy, partition policy, raw physmem, kernel drivers
```

## Privacy and persistence

The capsule reads and writes sector payloads only for explicit block protocol
requests. It does not parse partitions, mount filesystems, cache disk payloads,
or persist metadata. Queue memory and data buffers are broker DMA grants and
are revoked when the capsule exits.

## Runtime lifecycle

The capsule discovers one NVMe PCI function, claims it, enables bus mastering,
maps BAR0, binds MSI-X, allocates admin and IO queue DMA, disables the
controller, programs AQA/ASQ/ACQ, enables the controller, runs identify
commands, creates one IO queue pair, reads the SMART / health log, and then
serves IPC. Teardown unmaps DMA, unbinds IRQ, unmaps MMIO, and releases the
device claim.

## Failure model

Every setup phase is a barrier with reverse-order rollback. Controller timeout,
admin completion error, stale claim, MSI-X bind failure, or DMA allocation
failure prevents service start. Runtime block requests validate size and
capacity before submitting commands.

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
- Creates one IO submission/completion queue pair.
- Allocates PRP list and sector data DMA.
- Serves capacity, read, write, and flush requests over IPC.
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
errors, and error-log count. `OP_CAPACITY` returns an 8-byte sector count.
`OP_READ_BLOCKS` and `OP_WRITE_BLOCKS` use a 12-byte `lba, sector_count`
request header and fixed 512-byte sectors.

## State ownership

The capsule owns the controller claim epoch, BAR0 mapping, MSI-X grant, admin
queues, IO queues, identify DMA, health DMA, PRP list, sector data buffer,
controller snapshot, and namespace snapshot. The kernel owns capability
validation, grant records, IRQ routing, and teardown only.

## Operating rules

- Keep namespace and controller command logic inside the capsule.
- Do not parse partitions, filesystems, or encrypted volume headers here.
- Every setup phase must have reverse-order rollback.

## Release target

The next NVMe target is wider validation: namespace scanning, multi-queue
support, PRP boundary stress, timeout/error recovery, MSI-X completion
handling under load, teardown rollback, and one real NVMe controller boot. It
does not parse partitions, filesystems, encryption headers, or application data.

## Release evidence

Release requires QEMU `-device nvme` identify validation, IO queue creation
validation, single read/write/flush proof, PRP boundary tests, teardown DMA
revocation, and one real NVMe controller boot.

## Release checklist

- Signed manifest and publisher trust entries present.
- Kernel mirror embeds and feature-gates `driver.nvme0`.
- QEMU identify validation reports controller and NSID 1.
- IO queue creation and single read/write/flush validation pass.
- PRP/SGL boundary tests pass.
- Teardown proof shows admin and IO DMA grants are revoked.

## Explicit non-goals today

No discard, namespace scanning beyond NSID 1, multipath, partition table,
filesystem, encryption, or cache policy is exposed here.

## Verification

- Build: `make -B nonos-mk-driver-nvme`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: NVMe must not import kernel driver or memory internals;
  it must use `MkMmioMap`, `MkIrqBind`, and `MkDmaMap`.
- Broker check: setup rollback must unmap DMA, unbind IRQ, unmap MMIO, and
  release the device claim on failure.
- Documentation check: the static gate requires this README and its contract,
  authority, lifecycle, failure model, release evidence, and verification sections.
