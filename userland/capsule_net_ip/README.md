# capsule_net_ip

## Role

`capsule_net_ip` is the IPv4 network-layer capsule. It consumes Ethernet
delivery from `net.l2`, validates IPv4 packets, builds outbound IPv4 packets,
owns interface configuration, and routes protocol payloads toward ICMP, UDP,
and TCP capsules.

```text
net.udp / net.tcp / ICMP client
    |
    | transport payload
    v
net.ip -- IPv4 parse/build + route table --> net.l2
    |
    `-- Ethernet delivery / ARP resolution
```

## Microkernel contract

The capsule is IPC-only:

- `MkIpcRecv` receives requests on `service:4402:net.ip`.
- `MkIpcSend` replies through `reply:4403:endpoint.4294967330`.
- Its wire magic is `NIP4`.
- Its endpoint name is `net.ip`.
- Its kernel mirror target is `src/network/ip_capsule`.

The kernel does not parse IP headers, own route tables, fragment packets, or
dispatch transport protocols.

## Interface contract

| Operation | Meaning |
|---|---|
| `OP_HEALTHCHECK` | server liveness |
| `OP_GET_CONFIG` / `OP_SET_CONFIG` | interface address, prefix, gateway, MTU |
| `OP_SEND_PACKET` / `OP_POLL_PACKET` | IPv4 payload movement |
| `OP_ROUTE_ADD` / `OP_ROUTE_CLEAR` | runtime route table control |

## Authority

The manifest grants IPC and memory only:
`CAPSULE_REQUIRED_CAPS = 0x00018`. It has no hardware, driver, DMA, MMIO,
PIO, filesystem, admin, debug, or socket authority.

## Privacy and persistence

The capsule holds runtime interface state: MAC, IPv4 address, prefix, gateway,
MTU, upstream L2 port, and packet identification counter. That state is
ephemeral and disappears when the process exits. Packet payloads are not
persisted.

## Runtime lifecycle

The capsule receives interface config, maintains route entries, validates
inbound IPv4 packets, builds outbound IPv4 packets, asks `net.l2` for delivery,
and dispatches payloads by protocol number.

## Failure model

No config, no route, no neighbour, L2 fault, bad checksum, unsupported
protocol, RX empty, and table-full conditions return explicit protocol errors.
Fragmented or option-bearing packets are rejected in this slice.

## Current implemented surface

- IPv4 address helpers are present.
- RFC 791 header parse/build and checksum code are present.
- ICMP parse/build/echo helpers are present.
- A 16-entry longest-prefix route table is present.
- Protocol constants cover health, get/set config, send packet, poll packet,
  route add, and route clear.

## Wire format

Requests use the `NIP4` protocol magic. Config requests carry address, prefix,
gateway, MTU, and L2 endpoint fields. Packet requests carry protocol number,
destination/source address fields, and payload bytes bounded by MTU.

## State ownership

The capsule owns interface config, route table, packet identification counter,
and protocol demux state. `net.l2` owns neighbour resolution. UDP/TCP capsules
own transport state.

## Operating rules

- Reject malformed headers, bad checksum, fragments, and options in this slice.
- Keep route table bounded.
- Do not store payloads after dispatch.
- Never add socket or firewall policy here.

## Release target

The finished IP capsule owns IPv4 configuration, route lookup, checksum
validation, ICMP echo, transport demux to UDP/TCP, and L2 delivery through ARP.
It has validation evidence for address setup, ICMP round trip, route miss, checksum
failure, and no kernel IP parser.

## Release evidence

Release evidence is ICMP echo validation, route miss test, checksum failure test,
configuration update proof, and static proof that no kernel IP parser exists.

## Release checklist

- Address/gateway configuration validation passes.
- ICMP echo round trip passes through `net.l2`.
- Route miss and checksum failure are tested.
- Fragment/options rejection is tested.
- Static gate confirms no kernel IP parser.

## Explicit non-goals today

No IPv6, fragmentation/reassembly, IP options, multicast routing, firewall,
NAT, socket API, packet capture, persistent interface database, or hardware
access lives here. The L2 client, server loop, and main entry point still need
promotion before end-to-end runtime use.

## Verification

- Static gate: `bash nonos-ci/run-static-checks.sh`
- Build gate, once `src/main.rs` lands: `make -B nonos-mk-net-ip`
- Runtime proof: configure address/gateway, send ICMP echo through `net.l2`,
  receive echo reply, and prove the kernel never parses IP.
