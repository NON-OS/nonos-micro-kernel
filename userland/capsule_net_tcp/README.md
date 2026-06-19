# capsule_net_tcp

## Role

`capsule_net_tcp` is the TCP transport capsule. It owns TCP header
validation, segment construction, checksums, connection state, and per-flow
control blocks. It sits above `net.ip` and below `net.sockets`.

```text
net.sockets
    |
    | stream operation IPC
    v
net.tcp -- TCB + TCP state machine --> net.ip
```

## Microkernel contract

The capsule uses IPC and memory only:

- `MkIpcRecv` receives requests on `service:4430:net.tcp`.
- `MkIpcSend` replies through `reply:4431:endpoint.4294967350`.
- Its wire magic is `NTCP`.
- Its endpoint name is `net.tcp`.
- Its kernel mirror target is `src/network/tcp_capsule`.

The kernel does not own connection state, retransmission state, stream buffers,
port ownership, or TCP timers.

## Interface contract

| Operation | Meaning |
|---|---|
| `OP_HEALTHCHECK` | server liveness |
| `OP_LISTEN` / `OP_ACCEPT` | passive open and accepted connection handoff |
| `OP_CONNECT` | active open |
| `OP_SEND` / `OP_RECV` | stream byte movement |
| `OP_CLOSE` / `OP_SHUTDOWN` | orderly or half-close teardown |

## Authority

The manifest grants IPC and memory only:
`CAPSULE_REQUIRED_CAPS = 0x00018`. It has no driver, MMIO, IRQ, DMA, PIO,
filesystem, admin, debug, or direct NIC authority.

## Privacy and persistence

Connection state and stream buffers are runtime-only. The capsule must not
persist payloads, peer histories, packet captures, or socket identities across
process exit or reboot.

## Runtime lifecycle

The capsule owns listener state, active TCBs, send/receive variables, segment
validation, and transition logic. It exchanges packets with `net.ip` and
exposes stream operations to `net.sockets`.

## Failure model

No socket, port in use, refused connection, timeout, reset, closed state, bad
segment, and empty receive queue return protocol errors. Timer and retransmit
behavior must remain in this capsule, not the kernel.

## Current implemented surface

- TCP header parse/build helpers are present.
- TCP pseudo-header checksum code is present.
- The 11-state TCP state enum is present.
- TCB structures for send and receive variables are present.
- Protocol constants cover health, listen, connect, accept, send, receive,
  close, and shutdown.

## Wire format

Requests use the `NTCP` protocol magic. Listen/connect requests carry address
and port fields. Send requests carry socket id and bytes. Receive replies carry
socket id, status, and stream bytes. Segment wire format remains TCP over
`net.ip`, not a kernel syscall ABI.

## State ownership

The capsule owns listener tables, TCBs, sequence variables, receive buffers,
send buffers, retransmit timers, and close state. `net.sockets` owns caller
handles. The kernel owns no TCP state.

## Operating rules

- Keep timers and retransmission in userland.
- Bound receive and send buffers per connection.
- Return explicit errors for refused, timeout, reset, and closed states.
- Do not add TLS, DNS, or socket fd tables here.

## Release target

The finished TCP capsule owns listener tables, active connection TCBs,
handshake, data transfer, retransmit timers, close, reset handling, and
backpressure across `net.ip`. It has validation evidence for connect, accept, send,
receive, close, timeout, and reset without adding socket syscalls to the
kernel.

## Release evidence

Release evidence is handshake validation, listener accept validation, send/receive
transfer, FIN close, timeout, reset handling, and static proof that no kernel
TCP or socket syscall path exists.

## Release checklist

- Connect and accept validation passes.
- Send/receive transfer passes.
- FIN close, reset, and timeout are tested.
- Buffer bounds are enforced.
- Static gate confirms no kernel TCP/socket syscall path.

## Explicit non-goals today

No TLS, DNS, socket fd table, firewall, congestion-control tuning surface,
packet capture, persistent connection log, hardware access, or kernel TCP path
lives here. Runtime timers, server loop, `net.ip` client, and sockets
integration still need promotion before production networking can use it.

## Verification

- Static gate: `bash nonos-ci/run-static-checks.sh`
- Build gate, once `src/main.rs` lands: `make -B nonos-mk-net-tcp`
- Runtime proof: three-way handshake, data transfer, FIN close, timeout path,
  and RST handling through `net.ip` with no kernel TCP parser.
