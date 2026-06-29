# capsule_driver_i2c_hid

## Role

`capsule_driver_i2c_hid` is the HID-over-I2C class capsule. It sits above
`driver.i2c_pci0`, discovers a bounded HID descriptor, and exposes descriptor
state for the input runtime.

```text
input runtime
        |
        v
driver.i2c_hid0 -- IPC --> driver.i2c_pci0 -- MMIO/IRQ --> I2C controller
```

The capsule is not an I2C controller driver and has no direct hardware grants.

## Microkernel contract

The manifest grants `IPC` and `Memory` only:

```text
CAPSULE_REQUIRED_CAPS = 0x200019
```

The capsule resolves `driver.i2c_pci0` with `MkServiceLookup`, sends bounded IPC
requests with `MkIpcSend`, receives replies with `MkIpcRecv`, and serves callers
with `MkIpcRecvFrom` plus `MkIpcSendToPid`.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | found flag, address, probe count |
| `OP_PROBE` | none | refreshed descriptor state |
| `OP_DESCRIPTOR` | none | cached HID descriptor bytes |

Unknown operations reply `E_BAD_OP`. Malformed bodies reply `E_INVAL`.

## Authority

The capsule may talk to the I2C controller capsule over IPC. It has no PCI,
MMIO, IRQ, DMA, PIO, filesystem, network, display, or focus-routing authority.

## Privacy and persistence

The capsule stores no touch events, gestures, keystrokes, sensor samples, or
history. It keeps only the I2C address, probe counter, and current HID
descriptor bytes in volatile memory.

## Runtime lifecycle

Startup resolves `driver.i2c_pci0`, probes common HID-over-I2C addresses, reads
the 30-byte HID descriptor from register `0x0001`, validates descriptor length
and BCD version, and then serves IPC. A later interrupt-report slice will feed
bounded reports into the input router.

## Failure model

If the controller service is missing, startup fails closed. If no HID descriptor
is found, the capsule still serves health/probe calls but returns `E_NOT_FOUND`
for descriptor reads until a later probe succeeds.

## Current implemented surface

- Runtime service lookup for `driver.i2c_pci0`.
- Bounded HID descriptor reads through `OP_TRANSFER`.
- Probe list covering common ELAN, Synaptics, FocalTech, and alternate HID
  addresses.
- IPC health, reprobe, and descriptor export.
- No persistent input history and no direct hardware access.

## Wire format

Requests use the `NHID` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte signed status word. All multi-byte
integers are little-endian.

## State ownership

`driver.i2c_hid0` owns HID-over-I2C class state: descriptor bytes, selected I2C
address, and probe counters. `driver.i2c_pci0` owns controller registers and bus
transactions. The input router owns focus, event routing, and policy.

## Operating rules

- Do not map hardware or request Driver/DeviceEnum/Mmio/Irq/Dma/Pio caps.
- Do not persist reports, touches, gestures, or keystrokes.
- Do not route input focus here.
- Keep every transfer bounded by the I2C controller capsule limits.

## Release target

The target chain is:

```text
driver.i2c_pci0 -> driver.i2c_hid0 -> input router -> compositor / apps
```

The next slice is interrupt-report fetch/ack and normalized key/mouse/touch
events delivered to the input router.

## Release evidence

Release evidence requires signed spawn of both capsules, successful service
lookup, a bounded descriptor read from a real HID-over-I2C device, and a decoded
descriptor delivered to the input runtime without kernel-resident input policy.

## Release checklist

- Capsule builds with zero warnings.
- Static gates confirm IPC-only authority and endpoint ownership.
- Kernel profile `microkernel-driver-i2c-hid` resolves signed artifacts.
- Descriptor probe succeeds on supported hardware.
- No report history is persisted.

## Explicit non-goals today

This slice does not implement interrupt reports, gesture recognition, keyboard
layout mapping, touch filtering, focus routing, ACPI enumeration, or power
management.

## Verification

- Build: `make -B nonos-mk-driver-i2c-hid`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-i2c-hid`
- Static gate: `bash nonos-ci/run-static-checks.sh`
