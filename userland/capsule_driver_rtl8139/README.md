# capsule_driver_rtl8139

## Role

`capsule_driver_rtl8139` is the Realtek RTL8139 Fast Ethernet capsule. It is a
PIO NIC driver that moves raw Ethernet frames between the device and userland
network capsules.

```text
RTL8139 port BAR -- MkPio* --> driver.rtl8139_0
       |
       +-- MkDmaMap --------> packet buffers
       `-- MkIrqBind -------> frame/completion events
```

## Microkernel contract

The capsule reaches the NIC through broker authority only:

- `MkDeviceList` locates the RTL8139 PCI function.
- `MkDeviceClaim` owns the NIC claim.
- `MkPioGrant`, `MkPioRead`, and `MkPioWrite` access the port BAR.
- `MkIrqBind`, `MkIrqPoll`, and `MkIrqAck` own INTx interrupts.
- `MkDmaMap` and `MkDmaUnmap` allocate packet buffers.
- `MkIpcRecv` and `MkIpcSend` serve `driver.rtl8139_0` on
  `service:4212:driver.rtl8139_0`.

The kernel does not route packets or retain network policy.

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

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Irq`, `Dma`,
and `Pio` (`CAPSULE_REQUIRED_CAPS = 0x1D8018`). It has no MMIO, socket,
filesystem, admin, debug, or routing authority.

```text
allowed:   port BAR PIO, IRQ, DMA packet buffers, raw-frame IPC
forbidden: MMIO, IP stack, socket table, packet capture storage, firewall
```

## Privacy and persistence

RX/TX payloads are ephemeral. The capsule does not persist frames, keep peer
history, store packet captures, or log payloads. Buffers are broker resources
and are revoked on exit.

## Runtime lifecycle

The capsule claims the NIC, grants the PIO BAR, allocates packet buffers, binds
INTx, initializes device registers, and serves raw-frame IPC. Teardown releases
DMA, IRQ, PIO, and claim grants.

## Failure model

Setup failure rolls back previous grants. TX/RX errors are reported as NIC
faults. Empty RX is non-fatal. Port access remains broker-mediated for every
hardware read/write.

## Current implemented surface

- Builds as a signed driver capsule.
- Claims the NIC and owns the port/IRQ/DMA broker path.
- Advertises `driver.rtl8139_0`.
- Keeps protocol handling above the driver boundary.
- Reports command, link, interrupt, RX, TX, and software cursor state without
  reading counters that clear on access.
- Is ready for boot validation wiring against the network capsule stack.

## Wire format

Requests use the `NR89` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. MAC replies carry
6 bytes. Link replies carry 1 byte. RX replies carry length plus frame bytes.
TX requests carry one Ethernet frame bounded by `MAX_ETHERNET_FRAME`.
`OP_STATS` returns twelve little-endian `u32` fields:
`CMD`, `MSR`, `ISR`, `RCR`, `TCR`, `CAPR`, `TXSTATUS0..3`, `rx_offset`,
and `tx_cur`.

## State ownership

The capsule owns the PIO grant, IRQ grant, DMA packet buffers, MAC state, link
state, and TX/RX device state. Network protocol capsules own every byte above
the Ethernet frame boundary.

## Operating rules

- Keep RTL8139 as PIO-only; do not add MMIO grants.
- Keep all port access broker-mediated.
- Never persist packet payloads.
- Report RX-empty, link-down, and TX fault explicitly.

## Release target

The finished RTL8139 capsule is a signed raw-frame NIC service with port-BAR
PIO access through the broker, RX/TX buffer management, interrupt recovery,
link/MAC reporting, side-effect-free telemetry, and frame delivery to `net.l2`.
It must pass QEMU and hardware validation before being promoted beyond build-only.

## Release evidence

Release requires QEMU `rtl8139` round trip, broker PIO proof, RX/TX validation
through `net.l2`, teardown grant proof, and one compatible hardware boot.

## Release checklist

- Signed manifest and kernel mirror present.
- QEMU RTL8139 TX/RX validation passes through `net.l2`.
- PIO gate proves no inline port assembly.
- Teardown proof shows PIO/IRQ/DMA/device claim revocation.
- Hardware boot records link, MAC, RX, and TX behavior.

## Explicit non-goals today

No ARP, IP, DHCP, DNS, TCP, UDP, sockets, routing, firewall, offload policy,
or packet capture facility belongs in this capsule.

## Verification

- Build: `make -B nonos-mk-driver-rtl8139`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: RTL8139 must not map MMIO and must use broker PIO
  wrappers rather than inline assembly.
- Documentation check: this README is part of CI's driver-capsule contract.
