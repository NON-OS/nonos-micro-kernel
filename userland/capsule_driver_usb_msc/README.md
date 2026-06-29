# capsule_driver_usb_msc

## Role

`capsule_driver_usb_msc` is the USB Mass Storage class capsule. It classifies
USB configuration descriptors, records bulk-in and bulk-out endpoints for
SCSI-transparent BOT devices, and builds bounded command block wrappers for the
storage stack.

```text
USB flash / disk
        |
        v
driver.xhci0 -- descriptors + bulk transfers
        |
        v
driver.usb_msc0 -- BOT/SCSI framing --> block/storage capsules
```

The capsule is not a host-controller driver and is not a filesystem. PCI
ownership, MMIO, IRQ routing, DMA, xHCI rings, endpoint configuration, and bulk
transfer scheduling stay in `driver.xhci0`. Filesystems, partitioning, caching,
and encryption stay above the block layer.

## Microkernel contract

The manifest grants only `IPC` and `Memory`:

```text
CAPSULE_REQUIRED_CAPS = 0x19
```

The service receives requests with `MkIpcRecvFrom` and replies with
`MkIpcSendToPid`. It does not call device enumeration, MMIO, IRQ, DMA, or PIO
broker syscalls. The only persistent state is process-local runtime state:
last classified endpoints, monotonic BOT tags, and counters.

## Authority

This capsule has no hardware authority. It cannot enumerate USB controllers,
claim PCI devices, map controller registers, bind interrupts, allocate DMA, or
touch I/O ports. It receives descriptor bytes and command status wrappers over
IPC, then returns class-driver decisions and BOT command wrappers to its caller.

```text
allowed:   descriptor classification, BOT/SCSI framing, status accounting
forbidden: xHCI ownership, USB scheduling, DMA buffers, block cache, filesystem policy
```

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | status |
| `OP_PROBE_CONFIG` | raw USB configuration descriptor | MSC bindings |
| `OP_BUILD_INQUIRY` | none | BOT CBW for SCSI INQUIRY |
| `OP_BUILD_READ_CAPACITY10` | none | BOT CBW for READ CAPACITY(10) |
| `OP_BUILD_READ10` | `lba_le32, blocks_le16` | BOT CBW for READ(10) |
| `OP_BUILD_WRITE10` | `lba_le32, blocks_le16` | BOT CBW for WRITE(10) |
| `OP_ACCEPT_CSW` | 13-byte BOT CSW | status |
| `OP_GET_STATE` | none | counters and endpoint snapshot |

Unknown operations reply `E_BAD_OP`. Malformed descriptors or command bodies
reply `E_INVAL`. Valid descriptors without a SCSI-transparent BOT interface
reply `E_NO_MSC`.

## Privacy and persistence

The capsule does not store product strings, serial numbers, raw descriptors,
SCSI payloads, or block data. It keeps only the current endpoint binding table,
the last issued BOT tag, and diagnostic counters in process memory. Capsule
teardown drops that memory through normal userland process cleanup.

## Runtime lifecycle

At startup the capsule initializes its heap and waits on the service inbox. A
caller probes a configuration descriptor first; successful probes replace the
current endpoint snapshot. Block-layer callers then request BOT command wrappers
for INQUIRY, READ CAPACITY(10), READ(10), or WRITE(10). The host-controller
capsule performs the actual bulk transfers and returns the CSW for validation.

## Failure model

Malformed descriptor records fail closed without mutating the endpoint snapshot.
Oversized transfer counts are rejected before a CBW is emitted. Invalid CSW
signatures or illegal CSW status values are reported as protocol errors. A CSW
tag mismatch increments the phase-error counter so recovery code can reset the
USB mass-storage transport before issuing more commands.

## Current implemented surface

- USB configuration descriptor walk.
- SCSI-transparent BOT interface detection.
- Bulk IN / bulk OUT endpoint extraction.
- BOT command block wrapper construction.
- BOT command status wrapper validation.
- SCSI INQUIRY, READ CAPACITY(10), READ(10), and WRITE(10) CDB construction.
- Bounded transfer-length validation.
- Kernel-spawnable capsule metadata and stable endpoint contract.

## Wire format

Requests use the `NUMS` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte signed status word.

`OP_PROBE_CONFIG` returns a 32-bit binding count followed by 8-byte binding
records:

```text
interface, bulk_in, bulk_out, pad, max_packet_in_le16, max_packet_out_le16
```

CBW replies are the USB BOT 31-byte command block wrapper. CSW inputs are the
USB BOT 13-byte command status wrapper. READ(10) and WRITE(10) request bodies
are:

```text
lba_le32, block_count_le16
```

## State ownership

`driver.usb_msc0` owns only class-local state: endpoint bindings, BOT tag
generation, CSW pass/fail counters, phase-error counters, and residue totals.
`driver.xhci0` owns USB device slots, endpoint contexts, transfer rings, and
bulk scheduling. Storage capsules own block device registration, filesystems,
encryption, cache policy, and mount lifecycle.

## Operating rules

- Keep endpoint scheduling and all USB transfer mechanics in `driver.xhci0`.
- Keep filesystems, partitions, mount policy, and encryption above this driver.
- Do not add MMIO, PIO, IRQ, DMA, or device-enumeration authority here.
- Do not persist product strings, serial numbers, descriptors, or block data.
- Do not accept unbounded reads or writes; callers must provide bounded counts.

## Release target

The intended runtime chain is:

```text
driver.xhci0 -> driver.usb_msc0 -> block service -> filesystem capsules
```

The first release target is a USB flash device on QEMU xHCI: classify the MSC
interface, run INQUIRY, run READ CAPACITY(10), complete one bounded READ(10),
validate the CSW tag, and publish the resulting block geometry to the block
service without moving storage policy into the kernel.

## Release evidence

Build evidence covers the signed capsule ELF, endpoint contract, descriptor
parser, BOT/SCSI framing, and architecture gates. Runtime release evidence
requires a QEMU `qemu-xhci` boot with a USB storage device, descriptor
classification on serial, successful INQUIRY and READ CAPACITY(10), and a
bounded READ(10) transfer through the xHCI bulk-transfer service.

## Release checklist

- Capsule builds with zero warnings.
- Static gates confirm README, capability boundary, matrix row, and endpoint.
- Kernel profile `microkernel-driver-usb-msc` resolves with the signed capsule.
- Descriptor parser rejects malformed record lengths and missing bulk endpoints.
- CBW builders emit fixed-length, bounded BOT command wrappers.
- CSW validator rejects bad signatures, bad status values, and tag drift.
- QEMU xHCI USB storage validation passes INQUIRY, capacity, and bounded read.

## Explicit non-goals today

This slice does not implement xHCI bulk-transfer scheduling, USB reset recovery,
multi-LUN enumeration, UASP, SCSI sense decoding, filesystem mounting, writeback
caching, partition parsing, encryption, or block-device publication. Those are
separate capsules or later controller-transfer slices.

## Verification

- Build: `make -B nonos-mk-driver-usb-msc`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-usb-msc`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Runtime proof target: QEMU xHCI USB storage probe with INQUIRY, capacity, and
  one bounded read.
