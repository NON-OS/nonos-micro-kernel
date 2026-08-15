# capsule_driver_virtio_gpu

## Role

`capsule_driver_virtio_gpu` is the virtio GPU display-controller capsule. It
owns the PCI device claim, BAR mapping, interrupt grant, control-queue DMA, and
virtio initialization sequence for `driver.virtio_gpu0`.

```text
compositor / display runtime
        |
        v
driver.virtio_gpu0 -- control queue DMA --> virtio-gpu device
```

The capsule is not a compositor, window manager, renderer, toolkit, or input
router. It exposes device configuration and later scanout/resource commands;
all desktop policy stays in userland display/compositor capsules.

## Microkernel contract

The manifest requires `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
`Dma`, and `Pio`. `Debug` is optional: it carries the `probe_3d` and EDID
serial lines, and the kernel grants it only under `capsule-serial-debug`, so a
hardened build drops it and the capsule runs without it.

```text
CAPSULE_REQUIRED_CAPS = 0x1F9019
CAPSULE_OPTIONAL_CAPS = 0x100
CAPSULE_CAPS_CEILING  = 0x1F9119
```

The capsule reaches hardware only through `MkDeviceList`, `MkDeviceClaim`,
`MkMmioMap`/`MkPioGrant`, `MkIrqBind`, and `MkDmaMap`. The kernel brokers
grants, routes IPC, validates the signed manifest, and tears grants down on
exit. It does not
interpret GPU resources, scanout policy, composition, cursor policy, or window
ownership.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | status |
| `OP_CONTROLLER_INFO` | none | device id, queue size, feature bits |
| `OP_DISPLAY_INFO` | none | events, scanout count, capset count |
| `OP_CONTROLQ_STATE` | none | control queue DMA metadata |

Unknown operations reply `E_BAD_OP`. Malformed request bodies reply `E_INVAL`.

## Authority

The capsule may claim the virtio GPU PCI device, map BAR0, bind its IRQ, and
allocate one broker-owned DMA region for control queue 0. It has no framebuffer
MMIO grant, no compositor authority, no input authority, and no filesystem or
network authority.

## Privacy and persistence

The capsule stores no windows, screenshots, surfaces, pointer paths, keystrokes,
or display history. Runtime state is limited to grant ids, queue addresses,
device feature bits, and current virtio-gpu config counters.

## Runtime lifecycle

Startup discovers a virtio-gpu PCI function, claims it, maps BAR0, binds INTx,
allocates queue DMA, runs ACK/DRIVER/FEATURES_OK/DRIVER_OK, selects control
queue 0, and serves IPC. Shutdown relies on process teardown and broker grant
revocation.

## Failure model

Every setup phase rolls back prior broker grants on failure. Missing IRQ,
missing BAR0, zero queue size, or FEATURES_OK rejection prevents the capsule
from serving. Runtime requests read side-effect-free state unless a later
command-submission slice explicitly posts a controlq command.

## Current implemented surface

- Virtio GPU PCI discovery for transitional and modern IDs.
- Brokered device claim, MMIO map, IRQ bind, and DMA queue allocation.
- Legacy virtio control-queue initialization.
- Config reads for events, scanout count, and capset count.
- IPC health, controller info, display info, and control queue state.
- Static gates for capability boundary and endpoint ownership.

## Wire format

Requests use the `NVGP` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte signed status word. All multi-byte
integers are little-endian.

## State ownership

`driver.virtio_gpu0` owns only hardware-facing GPU state: PCI claim, MMIO grant,
IRQ grant, control queue DMA grant, queue size, feature mask, and config
counters. The compositor owns surfaces, damage, focus, cursor, z-order, and
presentation policy.

## Operating rules

- Do not draw or composite in the driver capsule.
- Do not expose framebuffer pointers to userland clients.
- Keep scanout/resource policy above the driver.
- Keep queue DMA broker-owned and revoked on capsule exit.
- Add command submission only with bounded command/response buffers.

## Release target

The target chain is:

```text
driver.virtio_gpu0 -> display runtime -> compositor -> wm/toolkit/apps
```

The next runtime slice is controlq command posting for GET_DISPLAY_INFO,
RESOURCE_CREATE_2D, ATTACH_BACKING, SET_SCANOUT, TRANSFER_TO_HOST_2D, and
RESOURCE_FLUSH, followed by QEMU virtio-gpu scanout validation.

## Release evidence

Release evidence requires a QEMU `virtio-gpu-pci` boot, signed capsule spawn,
successful display-info controlq response, a bounded 2D resource flush to
scanout 0, and compositor presentation through the display runtime.

## Release checklist

- Capsule builds with zero warnings.
- Static gates confirm brokered MMIO/IRQ/DMA authority and endpoint.
- Kernel profile `microkernel-driver-virtio-gpu` resolves signed artifacts.
- QEMU controlq GET_DISPLAY_INFO validation passes.
- QEMU resource flush validation presents visible pixels.

## Explicit non-goals today

This slice does not implement a compositor, 3D acceleration, virgl, Venus,
Wayland, font rendering, app surfaces, input routing, cursor policy, or window
management.

## Verification

- Build: `make -B nonos-mk-driver-virtio-gpu`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-virtio-gpu`
- Static gate: `bash nonos-ci/run-static-checks.sh`
