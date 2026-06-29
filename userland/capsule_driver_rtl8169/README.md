# capsule_driver_rtl8169

## Role

`capsule_driver_rtl8169` is the Realtek RTL8168/RTL8169 gigabit Ethernet
capsule. It owns MMIO device registers, interrupt delivery, DMA rings, and raw
Ethernet frame movement.

```text
network capsules
    |
    | raw frame IPC
    v
driver.rtl8169_0 -- MMIO / IRQ / DMA broker grants --> RTL8169 NIC
```

## Microkernel contract

The capsule uses the broker for all hardware access:

- `MkDeviceList` locates the Realtek gigabit NIC.
- `MkDeviceClaim` owns the NIC claim.
- `MkMmioMap` maps the register BAR.
- `MkIrqBind`, `MkIrqPoll`, and `MkIrqAck` own INTx interrupts.
- `MkDmaMap` and `MkDmaUnmap` allocate RX/TX descriptors and packet buffers.
- `MkIpcRecv` and `MkIpcSend` serve `driver.rtl8169_0` on
  `service:4214:driver.rtl8169_0`.

The kernel mediates grants and teardown only. Network policy is userland.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_LINK_STATUS` | link-up state | 1 byte |
| `OP_MAC_ADDRESS` | hardware MAC address | 6 bytes |
| `OP_TX_PACKET` | transmit one Ethernet frame | status word |
| `OP_RX_PACKET` | poll one received frame | length plus frame bytes |
| `OP_STATS` | non-mutating register and ring snapshot | 48 bytes |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0xF8019`). It has no PIO, socket,
filesystem, admin, debug, or routing authority.

```text
allowed:   NIC claim, MMIO, IRQ, DMA rings, raw-frame IPC
forbidden: PIO, IP stack, sockets, persistent packet store, firewall
```

## Privacy and persistence

Network frames are transient. The capsule holds payloads in DMA buffers and
IPC buffers only while servicing RX/TX. It does not persist traffic, keep peer
identity, or implement analytics.

## Runtime lifecycle

The capsule claims the NIC, maps MMIO, allocates RX/TX DMA rings, binds INTx,
programs the device, and serves raw-frame IPC. Teardown disables the device
path and releases DMA, IRQ, MMIO, and claim grants.

## Failure model

Setup failure rolls back previous grants. Runtime TX/RX errors are reported to
callers as device faults. Empty RX and link-down are ordinary reported states,
not kernel events.

## Current implemented surface

- Builds as a signed driver capsule.
- Uses MMIO/IRQ/DMA broker primitives.
- Advertises `driver.rtl8169_0`.
- Keeps packet protocol state above the NIC driver boundary.
- Reports command, PHY, interrupt, config, ring-size, and software cursor state
  without reading counters that clear on access.
- Is ready for kernel mirror and validation wiring.

## Wire format

Requests use the `NR69` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. MAC replies carry
6 bytes. Link replies carry 1 byte. RX replies carry length plus frame bytes.
TX requests carry one Ethernet frame bounded by `MAX_ETHERNET_FRAME`.
`OP_STATS` returns twelve little-endian `u32` fields:
`CMD`, `PHY_STATUS`, `ISR`, `IMR`, `RX_CONFIG`, `TX_CONFIG`, `RMS`, `rx_cur`,
`tx_cur`, `rx_desc_count`, `tx_desc_count`, and `reserved`.

## State ownership

The capsule owns MMIO mapping, IRQ grant, DMA descriptor rings, packet buffers,
MAC/link state, and TX/RX device state. `net.l2` and higher capsules own all
protocol interpretation.

## Operating rules

- Keep RTL8169 MMIO-only; do not request PIO grants.
- Keep payload retention bounded to RX/TX service paths.
- Report link-down and RX-empty as ordinary states.
- Do not add IP, socket, or firewall logic to the driver.

## Release target

The finished RTL8169 capsule is a signed gigabit raw-frame service with MMIO
register ownership, DMA descriptor lifecycle, interrupt recovery, link/MAC
reporting, side-effect-free telemetry, and frame delivery to `net.l2`.
Promotion requires kernel mirror, spawn, QEMU validation, and compatible hardware
proof.

## Release evidence

Release requires RTL8169-compatible hardware proof, emulator validation where
available, `net.l2` frame round trip, teardown grant proof, and link-change
behavior.

## Release checklist

- Signed manifest and kernel mirror present.
- Compatible hardware boot proves MAC/link/TX/RX.
- `net.l2` frame round trip passes.
- Teardown proof shows MMIO/IRQ/DMA/device claim revocation.
- Static gate confirms no PIO use.

## Explicit non-goals today

No ARP, IP, DHCP, DNS, TCP, UDP, sockets, routing, firewall, capture store,
or hardware-offload policy is implemented here.

## Verification

- Build: `make -B nonos-mk-driver-rtl8169`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: RTL8169 must stay MMIO-only and must not request PIO
  grants.
- Documentation check: this README is required by the driver docs gate.
