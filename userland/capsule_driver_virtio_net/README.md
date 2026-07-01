# capsule_driver_virtio_net

## Role

`capsule_driver_virtio_net` is the virtio network driver capsule. It sends and
receives raw Ethernet frames and exposes the frame service to userland network
capsules. It does not contain ARP, IP, TCP, UDP, DHCP, DNS, sockets, routing,
or firewall policy.

```text
net.l2 / net.ip capsules
    |
    | raw frame IPC
    v
driver.virtio_net0 -- virtqueue DMA --> virtio-net device
```

## Microkernel contract

The capsule uses only Mk/broker interfaces:

- `MkDeviceList` locates the virtio network device.
- `MkDeviceClaim` owns the device claim.
- `MkMmioMap` maps the virtio register window.
- `MkIrqBind`, `MkIrqPoll`, and `MkIrqAck` own completion interrupts.
- `MkDmaMap` and `MkDmaUnmap` allocate RX/TX rings and packet buffers.
- `MkIpcRecv` and `MkIpcSend` serve `driver.virtio_net0` on
  `service:4204:driver.virtio_net0`.

The kernel remains mechanism only: capability checks, address spaces, IPC, and
grant revocation. Network protocol logic belongs to userland network capsules.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_LINK_STATUS` | link-up state | 1 byte |
| `OP_MAC_ADDRESS` | virtio MAC address | 6 bytes |
| `OP_TX_PACKET` | transmit one Ethernet frame | status word |
| `OP_RX_PACKET` | poll one received frame | length plus frame bytes |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
and `Dma` (`CAPSULE_REQUIRED_CAPS = 0x1F8019`). It has no socket, route table,
firewall, DNS, DHCP, filesystem, admin, or debug authority.

```text
allowed:   virtio NIC claim, MMIO, IRQ, DMA rings, raw-frame IPC
forbidden: IP stack, sockets, packet store, routing policy, kernel drivers
```

## Privacy and persistence

Frames live only in RX/TX DMA buffers and IPC payloads. The capsule does not
persist traffic, track application identity, keep captures, or log packet
payloads.

## Runtime lifecycle

The capsule claims the virtio NIC, maps MMIO, binds IRQ, allocates RX/TX
virtqueues, reads MAC state, enables queues, and serves raw-frame IPC. Teardown
releases DMA, IRQ, MMIO, and claim grants.

## Failure model

Setup failure rolls back all earlier grants. TX failure and device faults are
returned as protocol errors. Empty RX is non-fatal. Protocol parsing never
enters this driver.

## Current implemented surface

- Initializes RX/TX queues.
- Validates queue physical addresses.
- Reads and reports the device MAC.
- Serves link, MAC, status, RX, and TX operations over IPC.
- Keeps protocol parsing above the driver boundary.

## Wire format

Requests use the `NNET` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte status word. MAC replies carry
6 bytes. Link replies carry 1 byte. RX replies carry length plus frame bytes.
TX requests carry one Ethernet frame bounded by `MAX_ETHERNET_FRAME`.

## State ownership

The capsule owns RX/TX virtqueues, packet buffers, MMIO mapping, IRQ grant, MAC
state, and link state. `net.l2` owns Ethernet/ARP behavior. The kernel owns no
packet buffer beyond broker mappings.

## Operating rules

- Keep the driver protocol-blind.
- Never persist frames or peer identity.
- Treat RX-empty and link-down as ordinary states.
- Keep all hardware access behind broker grants.

## Release target

The finished virtio-net capsule is a signed raw-frame NIC service with stable
RX/TX queue refill, interrupt recovery, link/MAC reporting, QEMU validation, and
delivery into `net.l2`. It remains protocol-blind: no ARP, IP, sockets,
firewall, or packet history is allowed in the driver.

## Release evidence

Release requires QEMU `virtio-net` frame round trip, `net.l2` ARP proof,
teardown DMA revocation proof, and host-network validation with packet boundaries
intact.

## Release checklist

- Signed manifest and kernel mirror present.
- QEMU virtio-net frame round trip passes.
- `net.l2` ARP proof passes above this driver.
- Teardown proof shows DMA/IRQ/MMIO/device claim revocation.
- Static gate proves no kernel network stack is imported.

## Explicit non-goals today

No ARP, IP, TCP, UDP, DHCP, DNS, sockets, routing, firewall, NAT, packet
capture, or traffic analytics live in this capsule.

## Verification

- Build: `make -B nonos-mk-driver-virtio-net`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: virtio-net must stay broker-only and must not import
  kernel network or driver code.
- Documentation check: this README is part of the driver-capsule acceptance
  criteria.
