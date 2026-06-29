# capsule_driver_i2c_pci

## Role

`capsule_driver_i2c_pci` is the Intel LPSS DesignWare I2C controller capsule.
It owns PCI discovery, BAR0 mapping, interrupt binding, controller identity,
clock metadata, and safe register telemetry for `driver.i2c_pci0`.

```text
touchpad / sensor / HID runtime
        |
        v
driver.i2c_pci0 -- brokered MMIO/IRQ --> Intel LPSS I2C controller
```

The capsule is not a HID parser, touchpad driver, sensor hub, ACPI policy
engine, or input router. Those layers stay above the controller driver.

## Microkernel contract

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, and `Irq`:

```text
CAPSULE_REQUIRED_CAPS = 0x78019
```

The capsule reaches hardware only through `MkDeviceList`, `MkDeviceClaim`,
`MkMmioMap`, and `MkIrqBind`. The kernel validates the signed manifest, brokers
grants, routes IPC, and revokes all grants on capsule exit.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | status |
| `OP_CONTROLLER_INFO` | none | PCI id, clock, MMIO/IRQ grants |
| `OP_REGISTER_SNAPSHOT` | none | DesignWare status/FIFO/config registers |
| `OP_TIMING_INFO` | none | standard/fast-mode SCL count registers |
| `OP_TRANSFER` | address, write bytes, read length | status, abort source, read bytes |
| `OP_PROBE` | 7-bit address | present / absent |

Unknown operations reply `E_BAD_OP`. Non-empty bodies on fixed-width requests
reply `E_INVAL`.

## Authority

The capsule may enumerate PCI devices, claim one Intel LPSS I2C function, map
BAR0, and bind the device IRQ. It has no DMA, PIO, filesystem, network, display,
credential, or input focus authority.

## Privacy and persistence

The capsule stores no touch events, gestures, sensor readings, device names, HID
reports, or ACPI tables. Runtime state is limited to grant ids, PCI identity,
controller clock metadata, and side-effect-free register snapshots.

## Runtime lifecycle

Startup discovers a supported Intel LPSS controller, claims it, maps BAR0, binds
the IRQ, verifies the DesignWare component type when exposed, masks controller
interrupts, clears pending interrupt state, and serves IPC. Process teardown and
broker revocation release every hardware grant.

## Failure model

Unsupported PCI IDs, missing BAR0, missing IRQ, failed grants, or an unreadable
controller window prevent the capsule from serving. Transfers are bounded to
small controller-local buffers, carry an explicit timeout, and surface
DesignWare abort state so higher layers can distinguish NACK from bus failure.

## Current implemented surface

- Intel LPSS I2C PCI discovery across Skylake through Meteor Lake-era ids.
- Brokered device claim, BAR0 MMIO map, and IRQ bind.
- DesignWare component-type read, enable/status/timing/FIFO register telemetry.
- Interrupt mask and clear during setup.
- Bounded master write/read transaction engine with timeout and TX-abort
  reporting.
- IPC health, controller info, register snapshot, timing info, probe, and
  transfer operations.
- Static gates for broker-only access and endpoint ownership.

## Wire format

Requests use the `NI2C` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte signed status word. All multi-byte
integers are little-endian.

## State ownership

`driver.i2c_pci0` owns only controller-facing I2C state: PCI identity, MMIO
grant, IRQ grant, component id, controller enable/status values, FIFO levels,
and timing registers. HID-over-I2C owns descriptors and reports; the input
router owns focus, routing, and policy.

## Operating rules

- Do not parse HID reports in this capsule.
- Do not persist touch, gesture, or sensor data.
- Do not import kernel driver, memory, paging, or hardware internals.
- Do not add inline architecture assembly or raw PIO.
- Keep bus transfers bounded and fail closed on timeout or controller abort.

## Release target

The target chain is:

```text
driver.i2c_pci0 -> i2c-hid runtime -> input router -> compositor / apps
```

The next runtime slice is IRQ-aware completion and ACPI-enumerated device
matching above the same bounded transfer primitive.

## Release evidence

Release evidence requires signed capsule spawn on Intel LPSS hardware, confirmed
controller identity, stable register snapshot IPC, successful bounded write-read
against an I2C HID descriptor register, and delivery of a decoded HID descriptor
to the higher-level HID runtime without kernel-resident input policy.

## Release checklist

- Capsule builds with zero warnings.
- Static gates confirm brokered MMIO/IRQ authority and endpoint ownership.
- Kernel profile `microkernel-driver-i2c-pci` resolves signed artifacts.
- Controller identity and timing registers are readable on supported hardware.
- Bounded write-read transaction validation passes without storing input history.

## Explicit non-goals today

This slice does not implement touchpad gestures, ACPI device matching,
interrupt-driven transfers, DMA, SMBus, sensor fusion, or input focus.

## Verification

- Build: `make -B nonos-mk-driver-i2c-pci`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-i2c-pci`
- Static gate: `bash nonos-ci/run-static-checks.sh`
