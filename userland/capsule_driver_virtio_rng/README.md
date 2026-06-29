# capsule_driver_virtio_rng

## Role

`capsule_driver_virtio_rng` is the virtio entropy-device capsule. It owns the
device-facing virtqueue and serves raw entropy bytes over IPC. It does not
mix entropy, stretch entropy, run a CSPRNG, or make cryptographic policy
decisions; those belong to entropy and crypto capsules above it.

```text
entropy consumer
    |
    | IPC request
    v
driver.virtio_rng -- virtqueue DMA --> virtio-rng device
    |
    `-- IPC reply with transient bytes
```

## Microkernel contract

The capsule is a normal signed user process:

- `MkDeviceList` discovers the virtio RNG device record.
- `MkDeviceClaim` gives this process the device claim.
- `MkMmioMap` maps the virtio MMIO register window.
- `MkIrqBind`, `MkIrqPoll`, and `MkIrqAck` own completion interrupts.
- `MkDmaMap` and `MkDmaUnmap` allocate virtqueue and entropy buffers.
- `MkIpcRecv` and `MkIpcSend` serve `driver.virtio_rng` on
  `service:4200:driver.virtio_rng`.

The kernel owns scheduling, address-space isolation, capability checks, and
grant revocation. It does not provide user-facing entropy policy from inside
the kernel.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_FILL_RANDOM` | fill caller buffer from virtio entropy | status plus bytes |
| `OP_HEALTHCHECK` | server liveness | status word |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0x1F8019`). It has no filesystem, network,
graphics, admin, debug, or raw kernel-memory authority.

```text
allowed:   one virtio RNG claim, MMIO, IRQ, DMA queue, IPC
forbidden: persistent storage, crypto policy, admin control, packet IO
```

## Privacy and persistence

Entropy bytes are sensitive and short-lived. The capsule does not persist
samples, write logs, expose device state to unrelated capsules, or keep a
long-term entropy pool. DMA memory is revoked on exit.

## Runtime lifecycle

The capsule claims the virtio RNG, maps MMIO, binds IRQ, allocates queue and
entropy DMA, initializes the virtqueue, performs a sanity fill, and serves IPC.
Teardown releases DMA, IRQ, MMIO, and claim grants.

## Failure model

Setup failure aborts startup. Fill failure returns an error and never falls
back to a fake software source. Request lengths are bounded by `MAX_FILL_BYTES`.

## Current implemented surface

- Discovers and claims the virtio RNG device.
- Maps MMIO and binds IRQ.
- Allocates DMA for virtqueue and entropy buffers.
- Initializes the queue and performs a sanity fill.
- Serves fill requests by submitting descriptors and waiting for used-ring
  completion.
- Fails closed if the hardware path cannot be established.

## Wire format

Requests use the `NORD` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. Fill replies return
bounded entropy bytes. `MAX_FILL_BYTES` is 4096.

## State ownership

The capsule owns the virtqueue, entropy DMA buffer, MMIO mapping, IRQ grant,
and device claim. The entropy service owns pool policy. The crypto capsule owns
cryptographic use of entropy.

## Operating rules

- Never fabricate entropy.
- Never persist samples.
- Bound every fill request.
- Fail closed if broker setup or device completion fails.

## Release target

The finished virtio-rng capsule is a signed entropy-source service with
startup health checks, refill handling, interrupt recovery, QEMU validation,
hardware-equivalent virtio proof, and strict delivery to the entropy service.
It provides source bytes only and never becomes the system CSPRNG or key
generator.

## Release evidence

Release requires QEMU fill validation, entropy-service handoff proof, request-bound
tests, teardown DMA revocation proof, and no fallback path that fabricates
entropy.

## Release checklist

- Signed manifest and kernel mirror present.
- QEMU fill validation passes.
- Entropy capsule consumes the source through IPC.
- Bounds tests reject oversized requests.
- Teardown proof shows DMA/IRQ/MMIO/device claim revocation.

## Explicit non-goals today

No entropy mixing, no software fallback RNG, no persistent health telemetry,
no key generation, and no crypto API live in this capsule.

## Verification

- Build: `make -B nonos-mk-driver-virtio-rng`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: the capsule must stay free of kernel driver imports and
  direct hardware access.
- Documentation check: this README is required by CI and describes authority,
  privacy, current surface, and non-goals.
