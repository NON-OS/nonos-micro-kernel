# capsule_driver_virtio_blk

## Role

`capsule_driver_virtio_blk` is the virtio block-device capsule. It owns the
virtio block queue and exposes sector-oriented read/write/flush operations over
IPC. It deliberately does not own filesystems, partitions, encryption, or cache
policy.

```text
storage capsules
    |
    | sector request IPC
    v
driver.virtio_blk0 -- virtqueue DMA --> virtio-blk device
    |
    `-- IRQ/used-ring completion
```

## Microkernel contract

The capsule uses brokered hardware authority:

- `MkDeviceList` locates the virtio block device.
- `MkDeviceClaim` owns the device claim.
- `MkMmioMap` maps the virtio register window.
- `MkIrqBind`, `MkIrqPoll`, and `MkIrqAck` own completion interrupts.
- `MkDmaMap` and `MkDmaUnmap` allocate queue, request, and data buffers.
- `MkIpcRecv` and `MkIpcSend` serve `driver.virtio_blk0` on
  `service:4202:driver.virtio_blk0`.

The kernel never embeds block-device policy. It validates capabilities,
mediates grants, routes IPC, and tears grants down on exit.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_CAPACITY` | sector capacity | 8-byte capacity |
| `OP_READ_BLOCKS` | read sectors into reply payload | status plus bytes |
| `OP_WRITE_BLOCKS` | write sectors from request payload | status word |
| `OP_FLUSH` | force device flush | status word |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0x1F8019`). It has no filesystem,
partition, crypto, admin, debug, or raw physical-memory authority.

```text
allowed:   virtio block claim, MMIO, IRQ, DMA queue, sector IPC
forbidden: filesystem parsing, partition ownership, writeback cache, LUKS
```

## Privacy and persistence

Sector payloads pass through broker DMA buffers for the active request. The
capsule does not persist a cache, inspect filesystem semantics, index file
contents, or keep block data after reply completion.

## Runtime lifecycle

The capsule claims the virtio block device, maps MMIO, binds IRQ, allocates
queue/request/data DMA, initializes the queue, probes capacity, and serves IPC.
Teardown releases DMA, IRQ, MMIO, and claim grants.

## Failure model

Setup failure rolls back grants. Runtime request failures return block errors
without retry loops in the kernel. Capacity and request-size bounds are checked
before DMA is submitted.

## Current implemented surface

- Discovers and claims the virtio block device.
- Initializes the virtqueue.
- Probes device capacity.
- Handles read, write, and flush requests over IPC.
- Waits through IRQ polling plus used-ring fallback.
- Unwinds broker grants on setup failure and shutdown.

## Wire format

Requests use the `NBLK` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. Capacity replies
return 8 bytes. Read/write requests use a 12-byte block header and data bounded
by `MAX_RW_PAYLOAD_BYTES`.

## State ownership

The capsule owns the virtqueue, request headers, data buffers, device capacity,
MMIO mapping, IRQ grant, and DMA grants. Filesystem and partition capsules own
all interpretation above sector reads and writes.

## Operating rules

- Validate sector range and payload length before submitting DMA.
- Keep reads/writes sector-oriented.
- Never cache filesystem contents here.
- Roll back DMA, IRQ, MMIO, and claim grants on setup failure.

## Release target

The finished virtio-blk capsule is a signed block-device service with stable
capacity reporting, read/write/flush semantics, queue reset recovery,
teardown-safe DMA handling, QEMU validation, and hardware-equivalent virtio proof.
It exposes sectors only; all filesystems and storage policy remain above it.

## Release evidence

Release requires QEMU read/write/flush validation, bounds tests for request length,
teardown DMA revocation proof, and a filesystem capsule test mounted above it.

## Release checklist

- Signed manifest and kernel mirror present.
- QEMU read/write/flush validation passes.
- Request length and capacity bounds are tested.
- Teardown proof shows all DMA grants are revoked.
- VFS/filesystem test works above the block endpoint.

## Explicit non-goals today

No partition table, filesystem, encryption layer, snapshotting, writeback
cache, allocator, volume manager, or fsck policy lives in this capsule.

## Verification

- Build: `make -B nonos-mk-driver-virtio-blk`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: virtio-blk must use broker MMIO/IRQ/DMA and must not
  import kernel memory or driver internals.
- Documentation check: this README is required for the capsule to pass CI.
