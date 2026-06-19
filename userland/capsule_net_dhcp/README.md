# capsule_net_dhcp

## Role

`capsule_net_dhcp` is the DHCPv4 client capsule. It obtains, renews, reports,
and releases IPv4 leases by talking to `net.udp` and applying accepted lease
configuration to `net.ip`.

```text
boot/network manager
    |
    | lease request IPC
    v
net.dhcp.client -- DHCP state machine --> net.udp --> net.ip
    |
    `-- accepted lease config --> net.ip
```

## Microkernel contract

The capsule is IPC-only:

- `MkIpcRecv` receives requests on `service:4440:net.dhcp.client`.
- `MkIpcSend` replies through `reply:4441:endpoint.4294967360`.
- Its wire magic is `NDHC`.
- Its endpoint name is `net.dhcp.client`.
- Its kernel mirror target is `src/network/dhcp_capsule`.

The kernel does not run DHCP, mutate interface configuration, or parse BOOTP
messages.

## Interface contract

| Operation | Meaning |
|---|---|
| `OP_HEALTHCHECK` | server liveness |
| `OP_LEASE_REQUEST` | start or restart acquisition |
| `OP_LEASE_STATUS` | return current lease state |
| `OP_LEASE_RELEASE` | release the active lease |
| `OP_LEASE_RENEW` | renew before expiry |

## Authority

The manifest grants IPC and memory only:
`CAPSULE_REQUIRED_CAPS = 0x00018`. It has no hardware, driver, MMIO, IRQ, DMA,
PIO, filesystem, admin, debug, or raw network-device authority.

## Privacy and persistence

Lease state is runtime network configuration. The capsule should keep address,
mask, gateway, DNS, lease time, server identifier, and renewal timers in
memory only. It does not persist lease history or client identifiers to disk.

## Runtime lifecycle

The capsule starts unleased, sends DHCP over `net.udp`, advances the client
state machine, installs accepted lease configuration into `net.ip`, renews
before expiry, and releases on request or teardown.

## Failure model

No link, timeout, NAK, malformed option, busy state, and UDP/IP failure return
protocol errors. A rejected lease must not modify `net.ip` configuration.

## Current implemented surface

- BOOTP/DHCP message representation is present.
- DHCP parse/build helpers are present.
- DHCP constants and options are present.
- A client state machine is present.
- Protocol constants cover health, request, status, release, and renew.

## Wire format

Requests use the `NDHC` protocol magic. Lease requests carry interface/client
identity fields once wired. Status replies carry lease address, mask, gateway,
DNS, lease time, server id, and current state.

## State ownership

The capsule owns the DHCP client state machine, active lease, renewal timers,
server identifier, and pending transaction id. `net.ip` owns installed
interface config.

## Operating rules

- Do not install a rejected or incomplete lease.
- Treat NAK and timeout as explicit states.
- Keep lease history volatile.
- Use `net.udp` for DHCP transport; never send raw frames.

## Release target

The finished DHCP capsule drives DISCOVER/OFFER/REQUEST/ACK, NAK, renew,
rebind, release, timeout, and lease-status paths through `net.udp`, then
installs accepted IPv4 configuration through `net.ip`. It keeps leases in
runtime memory and produces deterministic errors for no-link and timeout.

## Release evidence

Release evidence is DISCOVER/OFFER/REQUEST/ACK validation, NAK path, renew path,
release path, timeout path, and proof that accepted leases install through
`net.ip`.

## Release checklist

- DISCOVER/OFFER/REQUEST/ACK validation passes.
- NAK, timeout, renew, and release paths are tested.
- Accepted lease installs into `net.ip`.
- Rejected lease leaves prior config unchanged.
- Static gate confirms no kernel DHCP parser.

## Explicit non-goals today

No DHCP server, IPv6 SLAAC, DHCPv6, persistent lease database, network manager
UI, raw NIC access, or kernel interface mutation lives here. The UDP client,
IP configuration client, server loop, and renewal timer need promotion before
runtime use.

## Verification

- Static gate: `bash nonos-ci/run-static-checks.sh`
- Build gate, once `src/main.rs` lands: `make -B nonos-mk-net-dhcp`
- Runtime proof: DISCOVER, OFFER, REQUEST, ACK, installed IP config, renew,
  and release against a QEMU/user-network DHCP server.
