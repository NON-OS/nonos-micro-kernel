# capsule_net_sockets

## Role

`capsule_net_sockets` is the socket multiplexer capsule. It gives application
capsules one IPC-facing socket API while delegating transport behavior to
`net.tcp`, `net.udp`, `net.nym`, and name lookup to `net.dns`.

```text
application capsule
    |
    | socket IPC
    v
net.sockets -- per-pid handle table --> net.tcp / net.udp / net.nym / net.dns
```

## Microkernel contract

The capsule uses IPC and memory only:

- `MkIpcRecv` receives requests on `service:4460:net.sockets`.
- `MkIpcSend` replies through `reply:4461:endpoint.4294967380`.
- Its wire magic is `NSKT`.
- Its endpoint name is `net.sockets`.
- Its kernel mirror target is `src/network/sockets_capsule`.

The kernel does not expose a POSIX socket syscall table. Socket handles,
ownership, bind/connect/listen/accept/send/receive/close behavior, and
transport dispatch are userland policy.

## Interface contract

| Operation | Meaning |
|---|---|
| `OP_SOCKET` | allocate a caller-owned socket handle |
| `OP_BIND` / `OP_LISTEN` / `OP_ACCEPT` | server-side socket flow |
| `OP_CONNECT` | client-side connection flow |
| `OP_SEND` / `OP_RECV` | data movement through selected transport |
| `OP_CLOSE` | release handle and transport state |
| `OP_GETSOCKOPT` / `OP_SETSOCKOPT` | socket option surface |

## Authority

The manifest grants IPC and memory only:
`CAPSULE_REQUIRED_CAPS = 0x00018`. It has no hardware, driver, MMIO, IRQ, DMA,
PIO, filesystem, admin, debug, or raw packet authority.

## Privacy and persistence

The socket table is process-scoped runtime state. It should track only the
caller, handle, family, kind, endpoint, and transport state needed to dispatch
operations. It does not persist socket history, payloads, DNS history, or peer
metadata across exit.

## Runtime lifecycle

The capsule owns per-pid handle tables, maps handles to UDP/TCP/Nym transport
state, dispatches operations to transport capsules, and releases handles on
close or caller teardown.

## Failure model

No handle, no transport, table full, bad family/kind, not bound, not listening,
not connected, empty RX, refused, and timeout return protocol errors. No kernel
socket fallback exists.

## Current implemented surface

- Socket family and kind types are present.
- A per-pid socket table is present.
- Protocol constants cover socket, bind, listen, accept, connect, send,
  receive, close, getsockopt, and setsockopt.
- The capsule boundary keeps POSIX-shaped API compatibility in userland
  instead of reintroducing Linux-shaped kernel syscalls.

## Wire format

Requests use the `NSKT` protocol magic. Socket requests carry family and kind.
Bind/connect/listen/accept/send/receive/close requests carry caller-owned
socket handles and transport-specific address fields. Replies carry status,
socket handles, option values, or payload bytes.

## State ownership

The capsule owns per-pid socket tables, socket handles, option state, accept
queues, and transport dispatch state. UDP, TCP, DNS, and Nym capsules own
protocol state. The kernel owns no socket table.

## Operating rules

- Scope socket handles to caller identity.
- Route all transport work to `net.udp`, `net.tcp`, `net.dns`, or `net.nym`.
- Return explicit errors for bad handle, table full, and wrong socket state.
- Do not introduce Linux-shaped socket syscalls.

## Release target

The finished sockets capsule owns per-caller socket handles, bind/connect/listen
state, accept queues, transport dispatch to UDP/TCP/Nym, DNS-assisted connect
where policy allows it, mixnet cover-tick options, and close cleanup. It gives
applications a familiar API shape while keeping kernel syscalls native Mk-only.

## Release evidence

Release evidence is UDP socket validation, TCP connect/accept validation, Nym mixnet
session validation, close cleanup, per-pid isolation test, transport failure mapping,
and static proof that the kernel has no socket syscall surface.

## Release checklist

- UDP socket validation passes.
- TCP connect/accept validation passes.
- Nym mixnet socket connect/send/recv/cover option validation passes.
- Close cleanup releases handle state.
- Per-pid isolation is tested.
- Static gate confirms kernel syscall surface stays Mk-only.

## Explicit non-goals today

No kernel socket syscalls, raw NIC access, firewall, DNS resolver internals,
TLS, persistent connection database, packet capture, or filesystem-backed fd
table lives here.

## Verification

- Static gate: `bash nonos-ci/run-static-checks.sh`
- Build gate, once `src/main.rs` lands: `make -B nonos-mk-net-sockets`
- Runtime proof: create UDP and TCP sockets, bind/connect, send/receive, close,
  and prove that all transport work routes through network capsules.
