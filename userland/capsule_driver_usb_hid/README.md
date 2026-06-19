# capsule_driver_usb_hid

## Role

`capsule_driver_usb_hid` is the USB HID class capsule. It consumes USB
configuration descriptors and HID boot reports supplied by a USB host-controller
capsule, classifies boot keyboard and boot mouse interfaces, and normalizes
reports into bounded input-event queues.

```text
USB keyboard / mouse
        |
        v
driver.xhci0 -- EP0 descriptors + interrupt reports
        |
        v
driver.usb_hid0 -- normalized key/mouse events --> input runtime
```

The capsule is not a host-controller driver. PCI enumeration, MMIO, IRQ,
DMA, xHCI command rings, event rings, port reset, slot lifecycle, endpoint
configuration, and interrupt-transfer scheduling remain in `driver.xhci0`.

## Microkernel contract

The manifest grants only `IPC` and `Memory`:

```text
CAPSULE_REQUIRED_CAPS = 0x18
```

The service receives requests with `MkIpcRecvFrom` and replies with
`MkIpcSendToPid`. It does not call `MkDeviceList`, `MkDeviceClaim`,
`MkMmioMap`, `MkIrqBind`, `MkDmaMap`, or `MkPioGrant`. Descriptor bytes and HID
reports reach this capsule over IPC after controller mechanics have completed
elsewhere.

The kernel mirror embeds and spawns the signed capsule, then talks to it through
the same request/reply envelope used by other capsule clients. The client is a
thin transport and decode layer only; HID classification, report normalization,
queueing, and input semantics remain inside this userland capsule.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | status |
| `OP_PROBE_CONFIG` | raw USB configuration descriptor | HID bindings |
| `OP_FEED_KEYBOARD_REPORT` | 8-byte HID boot keyboard report | status |
| `OP_FEED_MOUSE_REPORT` | 3- or 4-byte HID boot mouse report | status |
| `OP_POLL_KEYS` | none | bounded key-event batch |
| `OP_POLL_MOUSE` | none | bounded mouse-event batch |
| `OP_GET_STATE` | none | counters and queue depths |

`OP_PROBE_CONFIG` validates the descriptor header, walks variable-length USB
descriptor records, and returns boot-protocol HID interfaces with interrupt IN
endpoints. It does not parse vendor report descriptors or infer policy from
manufacturer, product, or serial strings.

## Authority

The capsule has no hardware authority. Its capability mask is `0x18`, which is
`IPC | Memory`. It cannot enumerate PCI devices, claim USB controllers, map
registers, bind interrupts, allocate DMA, or touch I/O ports.

```text
allowed:   descriptor parsing, boot-report normalization, IPC replies
forbidden: controller ownership, input routing, focus policy, persistence
```

## Privacy and persistence

The capsule keeps only runtime queues and counters. It does not persist
keystrokes, mouse movement, USB topology, serial numbers, product strings, or
descriptor snapshots. Polling drains event queues; process exit destroys the
remaining process-local memory under normal capsule teardown.

## Runtime lifecycle

The capsule initializes its heap, creates empty keyboard and mouse state, and
waits on its service inbox. Callers first probe a configuration descriptor to
discover boot HID bindings, then feed reports retrieved by the controller
capsule. Input consumers poll normalized key and mouse events.

## Failure model

Malformed descriptors return `E_INVAL`. Valid descriptors without boot HID
interfaces return `E_NO_HID`. Unknown operations return `E_BAD_OP`. Oversized
or malformed reports are rejected and do not mutate event state.

## Current implemented surface

- USB configuration descriptor validation.
- Boot HID keyboard and mouse interface discovery.
- Interrupt IN endpoint extraction.
- Boot keyboard report normalization.
- Boot mouse report normalization.
- Bounded key and mouse event queues.
- State counters for descriptor probes and report ingestion.
- Kernel-side client for health, descriptor probe, report feed, event poll, and
  state readback.

## Wire format

Requests use the `NUHI` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte signed status word.

`OP_PROBE_CONFIG` returns a 32-bit binding count followed by 8-byte binding
records:

```text
kind, interface_number, endpoint_address, interval, max_packet_size_le16, pad[2]
```

Key events are 8 bytes:

```text
hid_usage, ascii, modifiers, pressed, pad[4]
```

Mouse events are 8 bytes:

```text
dx_le16, dy_le16, wheel_i8, buttons, flags, pad[1]
```

## State ownership

The capsule owns the keyboard previous-report snapshot, Caps Lock state,
keyboard event queue, mouse button snapshot, mouse event queue, and diagnostic
counters. The input runtime owns routing, focus, cursor position, acceleration,
gestures, and compositor delivery.

## Operating rules

- Do not add MMIO, PIO, IRQ, DMA, or device-enumeration authority here.
- Do not parse HID reports in the kernel.
- Do not persist keys, movement, descriptor bytes, or USB identity strings.
- Keep endpoint scheduling in `driver.xhci0`.
- Keep focus, cursor, and gesture policy above this capsule.

## Release target

The finished USB HID path is a signed class-driver chain:

```text
driver.xhci0 -> driver.usb_hid0 -> input runtime -> compositor
```

The remaining controller-side requirement is xHCI interrupt endpoint
configuration and periodic transfer polling. Once that lands, this capsule can
consume live reports instead of caller-supplied reports.

## Release evidence

Release evidence requires a QEMU `qemu-xhci` boot with a USB keyboard and USB
pointer device, descriptor classification on serial, key press/release polling,
mouse delta polling, malformed-descriptor rejection, and no grant requests
outside `IPC | Memory`.

The current build evidence covers the signed capsule ELF, kernel embed/spawn
mirror, kernel client compile path, static architecture gates, and diff hygiene.
Live interrupt endpoint feed from `driver.xhci0` is the next runtime proof.

## Release checklist

- Capsule builds with zero warnings.
- Static gates confirm README, capability boundary, and matrix row.
- Kernel profile `microkernel-driver-usb-hid` resolves with the capsule client.
- Descriptor parser rejects malformed lengths and oversized payloads.
- Keyboard and mouse report feeds produce bounded event batches.
- QEMU xHCI live interrupt-report validation passes.

## Explicit non-goals today

Report-descriptor parsing, vendor HID layouts, multitouch, tablet absolute
coordinates, LED output reports, endpoint scheduling, USB hub traversal, and
compositor focus policy do not live in this slice.

## Verification

- Build: `make -B nonos-mk-driver-usb-hid`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-usb-hid`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: no `Driver`, `DeviceEnum`, `Mmio`, `Irq`, `Dma`, or `Pio`
  capability appears in `Capsule.mk`.
- Runtime proof target: QEMU `qemu-xhci` keyboard and pointer device, with
  descriptor classification and event polling confirmed on serial.
