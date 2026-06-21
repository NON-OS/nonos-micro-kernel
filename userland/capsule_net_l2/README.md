# capsule_net_l2

## Role

`capsule_net_l2` is the Ethernet and ARP capsule. It sits above one raw NIC
driver capsule and below `capsule_net_ip`. Its responsibility is narrow:
Ethernet header handling, MAC/link discovery, ARP resolution, ARP cache state,
and frame delivery to the next network layer.

```text
net.ip
    |
    | NIP4 packets / neighbour requests
    v
net.l2 -- ARP cache + Ethernet framing --> driver.virtio_net0 / e1000 / rtl*
    |
    `-- raw Ethernet frame IPC
```

## Microkernel contract

The capsule has no hardware grants. It is an IPC service:

- `MkIpcRecv` receives requests on `service:4400:net.l2`.
- `MkIpcSend` replies through `reply:4401:endpoint.4294967320`.
- Its wire magic is `NL2`.
- Its endpoint name is `net.l2`.
- Its kernel mirror target is `src/network/l2_capsule`.

The kernel does not parse Ethernet, maintain ARP state, or choose network
routes. It only routes IPC between signed capsules.

## Interface contract

| Operation | Meaning |
|---|---|
| `OP_HEALTHCHECK` | server liveness |
| `OP_GET_MAC` / `OP_GET_LINK` | NIC identity and link state through the selected driver |
| `OP_SEND_FRAME` / `OP_POLL_FRAME` | raw Ethernet frame movement |
| `OP_ARP_RESOLVE` | resolve IPv4 next hop to MAC |
| `OP_ARP_SNAPSHOT` | inspect bounded ARP cache state |

## Authority

The manifest grants IPC and memory only:
`CAPSULE_REQUIRED_CAPS = 0x00018`. The capsule has no `Driver`,
`DeviceEnum`, `Mmio`, `Irq`, `Dma`, `Pio`, filesystem, admin, or debug
authority.

## Privacy and persistence

The capsule observes link-layer addresses and raw Ethernet payloads in transit.
It keeps a bounded ARP cache in memory. It does not persist frames, write packet
captures, store peer history across reboot, or inspect application identities.

## Runtime lifecycle

The capsule binds to one NIC driver service, learns local MAC/link state,
handles ARP traffic, maintains the neighbour cache, and forwards frames between
the NIC driver and `net.ip`.

## Failure model

No link, unknown neighbour, TX busy, RX empty, malformed frame, and NIC fault
return protocol errors. ARP cache exhaustion evicts by policy rather than
growing unbounded.

## Current implemented surface

- Ethernet header parse/write helpers are present.
- ARP packet parse/build helpers are present.
- ARP cache and inbound ARP handling are present.
- Protocol constants cover health, MAC, link, send frame, poll frame, ARP
  resolve, and ARP snapshot operations.

## Wire format

Requests use the `NL2` protocol magic. Frame operations carry raw Ethernet
frames bounded by MTU. ARP snapshot replies carry a bounded list of neighbour
records. Every reply returns a status code before operation-specific data.

## State ownership

The capsule owns the ARP cache, local MAC/link snapshot, selected NIC endpoint,
and pending RX/TX service state. NIC drivers own hardware rings. `net.ip` owns
IP configuration and routing.

## Operating rules

- Keep ARP cache bounded.
- Keep raw frame payloads transient.
- Never add IP route policy here.
- Treat no-link, no-neighbour, RX-empty, and NIC fault as explicit states.

## Release target

The finished L2 capsule binds to one selected NIC driver service, performs ARP
request/reply exchange, maintains a bounded neighbour cache, forwards raw
frames to `net.ip`, exposes link/MAC state, and passes a QEMU ARP round trip
without kernel packet parsing.

## Release evidence

Release evidence is a QEMU ARP request/reply validation, neighbour-cache snapshot,
raw frame send/poll proof, and static proof that the kernel parses no Ethernet
or ARP state.

## Release checklist

- NIC client wiring selects one driver endpoint.
- ARP request/reply validation passes.
- Neighbour cache snapshot is bounded.
- Raw frame send/poll works through a driver capsule.
- Static gate confirms no kernel Ethernet/ARP parser.

## Explicit non-goals today

No NIC hardware access, IP routing, TCP/UDP policy, DHCP, DNS, socket table,
firewall, packet capture store, or persistent neighbour database lives here.
The server loop and upstream NIC-client wiring still need promotion before this
capsule can be called end-to-end.

## Verification

- Static gate: `bash nonos-ci/run-static-checks.sh`
- Build gate, once `src/main.rs` lands: `make -B nonos-mk-net-l2`
- Runtime proof: ARP request/reply through a NIC capsule, followed by a
  populated in-memory neighbour entry and no kernel packet parsing.
