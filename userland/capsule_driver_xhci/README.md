# capsule_driver_xhci

## Role

`capsule_driver_xhci` is the USB 3 host-controller capsule. It owns xHCI
controller bring-up, controller rings, event processing, port status, and the
controller-owned slot lifecycle used by USB enumeration. USB class policy
belongs to separate HID, storage, audio, network, and hub capsules above it.

```text
USB class capsules
    |
    | controller service IPC
    v
driver.xhci0 -- MMIO / IRQ / DMA broker grants --> xHCI controller
    |
    +-- command ring
    `-- event ring / ERST
```

## Microkernel contract

The capsule interacts with hardware only through broker grants:

- `MkDeviceList` locates the xHCI controller.
- `MkDeviceClaim` owns the controller claim.
- `MkMmioMap` maps the controller register window.
- `MkIrqBind`, `MkIrqPoll`, and `MkIrqAck` own controller interrupts.
- `MkDmaMap` and `MkDmaUnmap` allocate DCBAA, scratchpads, command ring,
  event ring, and ERST storage.
- `MkIpcRecv` and `MkIpcSend` serve `driver.xhci0` on
  `service:4206:driver.xhci0`.

The kernel does not enumerate USB devices, parse descriptors, or implement USB
class behavior. It grants resources and revokes them.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_CONTROLLER_STATUS` | controller register/ring/slot state | 56-byte status |
| `OP_PORT_STATUS` | port state list | count plus 8-byte entries |
| `OP_ENABLE_SLOT` | issue xHCI Enable Slot command | status plus slot id |
| `OP_DISABLE_SLOT` | issue xHCI Disable Slot command | status word |
| `OP_ADDRESS_DEVICE` | reset a root port and issue Address Device | slot, port, speed, EP0 MPS |
| `OP_GET_DEVICE_DESCRIPTOR` | run EP0 GET_DESCRIPTOR(Device) | 18-byte device descriptor |
| `OP_GET_CONFIG_DESCRIPTOR` | run EP0 GET_DESCRIPTOR(Configuration) | bounded raw config bytes |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0xF8018`). It has no filesystem, input
routing, audio, network, graphics, admin, or debug authority.

```text
allowed:   xHCI claim, MMIO, IRQ, controller DMA rings, controller IPC
forbidden: HID policy, mass-storage policy, CDC network policy, kernel USB
```

## Privacy and persistence

Controller rings and descriptors are runtime-only DMA state. The capsule does
not persist USB topology, device descriptors, keystrokes, storage payloads, or
audio data. Class capsules must request only the information they are
authorized to consume.

## Runtime lifecycle

The capsule claims xHCI, maps MMIO, binds IRQ, allocates controller DMA state,
halts and resets the controller, starts command/event rings, runs a No-op
command, and serves controller status IPC. Runtime enumeration callers may then
enable a controller slot and must disable that slot if enumeration fails or a
device is removed. Teardown releases DMA, IRQ, MMIO, and claim grants.

## Failure model

Setup failure rolls back every prior grant. Controller-not-ready and command
timeout paths abort promotion. Slot enable failures return deterministic
protocol errors and do not mark the slot table. Address Device owns its output
device context, input context, and EP0 ring through the slot table; Disable Slot
clears the matching DCBAA entry and drops those DMA grants. Device class
requests remain outside this capsule.

## Current implemented surface

- Claims the xHCI controller.
- Maps MMIO and binds IRQ.
- Allocates DCBAA, scratchpads, command ring, event ring, and ERST.
- Halts and resets the controller.
- Waits for CNR clear and starts the controller.
- Issues a No-op command.
- Serves controller and port status over IPC.
- Issues Enable Slot and returns the controller-assigned slot id.
- Tracks enabled slots in capsule-local bounded state.
- Issues Disable Slot and clears the local slot table on success.
- Resets connected root ports before Address Device.
- Builds 32-byte or 64-byte xHCI input contexts from HCCPARAMS1.CSZ.
- Installs per-slot output contexts in DCBAA.
- Allocates a per-slot EP0 transfer ring.
- Issues Address Device and stores per-slot enumeration resources.
- Runs EP0 `GET_DESCRIPTOR(Device)` and returns the raw 18-byte descriptor.
- Runs EP0 `GET_DESCRIPTOR(Configuration)` and returns bounded raw bytes for
  class-capsule discovery.

## Wire format

Requests use the `NXHC` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. Controller status
returns 56 bytes. Port status returns a count plus 8-byte port records.
Enable Slot returns a 4-byte payload whose first byte is the slot id. Disable
Slot takes a 1-byte request payload containing the slot id and returns only the
status word. Address Device takes `[slot_id, root_port]` and returns eight
bytes: slot, port, xHCI speed id, reserved, little-endian EP0 max packet size,
and reserved padding. Device Descriptor takes `[slot_id]` and returns the USB
device descriptor bytes exactly as read from EP0. Configuration Descriptor takes
`[slot_id, index, len_lo, len_hi]`, currently accepts index `0`, caps length at
512 bytes, and prefixes the reply body with the returned byte count.

## State ownership

The capsule owns MMIO mapping, IRQ grant, DCBAA, scratchpads, command ring,
event ring, ERST, port state snapshot, enabled-slot table, and controller
command state. USB class capsules own device-class policy.

## Operating rules

- Do not parse HID, hub, mass-storage, audio, or CDC descriptors here.
- Keep descriptor parsing out of the kernel.
- Bound controller-reported port lists.
- Pair every successful Enable Slot with Disable Slot if Address Device or
  descriptor fetch fails.
- Roll back DMA, IRQ, MMIO, and claim grants on setup failure.

## Release target

The finished xHCI capsule is a signed USB host-controller service with slot
enable/disable, Address Device, endpoint-zero control transfers, event processing,
port-change handling, reset recovery, and class-capsule handoff. It owns the
controller mechanics only; HID, storage, audio, CDC, and hub policy stay in
separate USB class capsules.

## Release evidence

Release requires QEMU `qemu-xhci` validation, No-op completion proof, Enable Slot
/ Disable Slot proof, port-change proof, endpoint-zero GetDescriptor validation,
teardown DMA revocation, and class capsule handoff tests.

## Release checklist

- Signed manifest and kernel mirror present.
- QEMU xHCI No-op validation passes.
- Slot enable/disable, port-change, and GetDescriptor validation pass.
- Teardown proof shows DMA/IRQ/MMIO/device claim revocation.
- HID or mass-storage class capsule handoff is proven over IPC.

## Explicit non-goals today

Hub traversal, HID reports, USB mass storage, USB audio, CDC Ethernet,
isochronous scheduling, and persistent USB inventory do not live here. They are
separate class-capsule responsibilities above `driver.xhci0`.

## Verification

- Build: `make -B nonos-mk-driver-xhci`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: xHCI must remain MMIO/IRQ/DMA broker-only and must not
  use raw PIO or kernel USB internals.
- Documentation check: this README is required by the driver docs gate.
