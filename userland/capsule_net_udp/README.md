# capsule_net_udp

## Role

`capsule_net_udp` is the UDP transport capsule. It sits above `net.ip`, owns
UDP header validation and construction, and serves datagram operations to DHCP,
DNS, sockets, and direct network clients.

```text
DHCP / DNS / sockets / client capsule
    |
    | datagram IPC
    v
net.udp -- UDP parse/build/checksum --> net.ip
```

## Microkernel contract

The capsule is a signed IPC service:

- `MkIpcRecv` receives requests on `service:4420:net.udp`.
- `MkIpcSend` replies through `reply:4421:endpoint.4294967340`.
- Its wire magic is `NUDP`.
- Its endpoint name is `net.udp`.
- Its kernel mirror target is `src/network/udp_capsule`.

The kernel does not allocate ports, parse UDP, or own datagram queues.

## Interface contract

| Operation | Meaning |
|---|---|
| `OP_HEALTHCHECK` | server liveness |
| `OP_BIND` / `OP_UNBIND` | own a UDP port in capsule state |
| `OP_SEND` | send one datagram through `net.ip` |
| `OP_RECV` | poll one datagram for a bound port |

## Authority

The manifest grants IPC and memory only:
`CAPSULE_REQUIRED_CAPS = 0x00018`. It has no driver, MMIO, IRQ, DMA, PIO,
filesystem, admin, debug, or direct network-device authority.

## Privacy and persistence

UDP payloads are transient. The capsule should hold only runtime port binding
and receive-queue state. It does not persist datagrams, record peers, or keep
traffic logs.

## Runtime lifecycle

The capsule owns the port table, accepts binds, validates UDP headers and
checksums, dispatches outbound datagrams through `net.ip`, and queues inbound
datagrams for callers.

## Failure model

No port, port in use, no IP link, bad payload, and empty RX return protocol
errors. Datagram loss is surfaced to callers; there is no retry or stream
semantics.

## Current implemented surface

- UDP header representation is present.
- UDP parse/build helpers are present.
- RFC 768 pseudo-header checksum code is present.
- Protocol constants cover health, bind, unbind, send, and receive.
- The maximum payload is pinned to MTU 1500 minus IPv4 and UDP headers.

## Wire format

Requests use the `NUDP` protocol magic. Bind/unbind requests carry port
numbers. Send requests carry destination address, port fields, and datagram
payload. Receive replies carry source address, source port, and payload bytes.

## State ownership

The capsule owns port bindings, receive queues, and datagram dispatch state.
`net.ip` owns packet routing. DHCP, DNS, and sockets own their higher-level
protocol state.

## Operating rules

- Enforce one owner per bound port.
- Drop or report datagrams without a bound receiver.
- Do not retry or order datagrams.
- Never add DHCP/DNS/socket policy here.

## Release target

The finished UDP capsule owns port binding, datagram queues, checksum handling,
send/receive dispatch through `net.ip`, and deterministic errors for empty
queues, missing ports, and IP faults. DHCP, DNS, and sockets use it as a
transport service rather than duplicating UDP logic.

## Release evidence

Release evidence is bind/send/receive validation, port-collision test, checksum
failure test, and DHCP/DNS clients using this capsule instead of duplicating
UDP parsing.

## Release checklist

- Bind/send/receive validation passes.
- Port collision returns deterministic error.
- Checksum failure is tested.
- DHCP and DNS route through this capsule.
- Static gate confirms no kernel UDP parser.

## Explicit non-goals today

No IP routing, fragmentation, DHCP state, DNS cache, TCP semantics, socket fd
table, firewall, packet capture, retransmission, or hardware access lives
here. The server loop and `net.ip` client still need promotion for runtime use.

## Verification

- Static gate: `bash nonos-ci/run-static-checks.sh`
- Build gate, once `src/main.rs` lands: `make -B nonos-mk-net-udp`
- Runtime proof: bind a UDP port, send a datagram through `net.ip`, receive a
  datagram back, and prove port state is process-local to the capsule.
