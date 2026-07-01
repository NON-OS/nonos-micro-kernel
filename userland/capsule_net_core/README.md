# capsule_net_core

## Role

`capsule_net_core` is the integrated network core capsule. It binds the selected
network device path to smoltcp-backed interface state and exposes compact IPC
operations for DHCP status, DNS resolution, UDP sockets, and TCP sockets.

```text
browser / apps
    |
    | MkIpc socket operations
    v
net.core -- smoltcp iface/sockets --> net device client --> selected NIC capsule
    |
    `-- bounded replies to callers
```

## Microkernel contract

The capsule is an IPC network service. The kernel routes messages and enforces
capabilities; it does not hold TCP state, DNS cache state, DHCP lease state, or
application socket buffers.

- `MkIpcRecv` receives network service requests.
- `MkIpcSend` returns bounded replies.
- The capsule uses `CAPSULE_REQUIRED_CAPS` from `Capsule.mk`.
- The service endpoint is `net.core`.

## Interface contract

The capsule accepts protocol-framed operations for service health, DHCP status,
DNS A resolution, UDP bind/send/recv/unbind, TCP connect/send/recv/state/close,
and interface polling. Requests are parsed from explicit wire headers and every
unknown operation returns an error status.

## Authority

The capsule owns network stack state only. It does not own MMIO, IRQ, DMA, PIO,
device enumeration, raw PCI config, filesystem access, debug authority, or
admin authority. Hardware access remains below the selected driver capsule and
broker grants.

## Privacy and persistence

The capsule observes IP addresses, DNS names requested by callers, socket peer
addresses, transient packet payloads, and DHCP lease data. It keeps this state
in memory only and does not write packet captures, DNS history, socket history,
or lease history to persistent storage.

## Runtime lifecycle

At start, the capsule discovers the selected network path, initializes the
smoltcp interface, polls the device, maintains DHCP and socket progress, and
services IPC requests. Callers see explicit busy, empty, no-route, and protocol
errors rather than implicit blocking.

## Failure model

No device, link down, missing lease, DNS failure, malformed request, unsupported
operation, socket exhaustion, connection refusal, timeout, receive-empty, and
device TX/RX failure are explicit protocol results. They must not panic the
capsule or leak ownership of a caller socket handle.

## Current implemented surface

- Device RX/TX client modules are present.
- smoltcp interface construction and polling are present.
- DHCP, DNS, UDP, and TCP handler modules are present.
- Request parsing and bounded response helpers are present.
- Unknown operations reply with a protocol error.

## State ownership

The capsule owns the smoltcp interface, socket tables, DHCP state, DNS resolver
state, and the selected network device client. Driver capsules own hardware
rings and broker grants. Applications own their own navigation or socket call
intent, not network-global state.

## Operating rules

- Never move TCP, DHCP, DNS, or socket state into the kernel.
- Keep all request and response buffers bounded.
- Return explicit protocol errors for malformed or unsupported operations.
- Keep packet captures and persistent network telemetry out of this capsule.

## Release evidence

Release evidence for this capsule is a signed and attested `net.core`, static
proof that it has no hardware authority, DHCP lease evidence, DNS A-resolution
evidence, TCP connect/send/recv/close evidence, UDP bind/send/recv evidence,
and a browser fetch that reaches the GUI through the NØNOS network path.

## Wire format

Requests use explicit protocol magics for DHCP, DNS, UDP, and TCP families.
Each request starts with an operation code and bounded payload length. Replies
return a status word before operation-specific data. Socket operations carry
caller-owned handles; the capsule owns the backing socket state.

## Release target

The finished capsule is a first-class production network profile:
`nonos-mk-net-core-prod` builds the signed capsule set and the kernel profile
with `microkernel-net-core`. The capsule must boot, acquire or report DHCP
state, resolve DNS, open TCP, move bytes, close sockets, and keep all hardware
access below brokered driver capsules.

## Release checklist

- Root build includes `userland/capsule_net_core/Capsule.mk`.
- `microkernel-net-core` embeds and spawns `net_core`.
- DHCP status returns explicit lease or no-lease state.
- DNS A resolution returns bounded replies.
- TCP connect/send/recv/state/close works through IPC.
- Unknown operations return protocol errors.
- Static gate confirms no kernel packet stack.

## Explicit non-goals today

No raw NIC ownership, packet capture store, persistent DNS cache, browser
history, firewall policy, Tor/Nym routing policy, TLS verification, or GUI
rendering belongs in `net.core`. Those remain in driver capsules, browser
capsule, Nym capsule, or user-facing apps.

## Verification

- Static gate: `bash nonos-ci/run-static-checks.sh`
- Capsule build: `make userland/capsule_net_core/target/x86_64-nonos-user/release/net_core`
- Production profile: `make nonos-mk-net-core-prod`
- Runtime proof: DHCP/DNS/TCP socket transaction followed by a browser fetch
  rendered in the GUI through `net.core`.
