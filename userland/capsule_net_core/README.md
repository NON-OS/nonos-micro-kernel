# capsule_net_core

## Role

`capsule_net_core` owns the selected NIC client and the in-memory network
interface state used by the higher network services.

```text
DHCP / DNS / sockets
    |
    v
net.core -- interface poll / link state --> driver.virtio_net0 / e1000 / rtl*
```

## Microkernel contract

The capsule is a signed IPC service:

- `MkIpcRecv` receives requests on `service:4480:net.core`.
- `MkIpcSend` replies through `reply:4481:endpoint.net.core.reply`.
- Its endpoint name is `net.core`.
- Its kernel mirror target is `src/userspace/capsule_net_core`.

The kernel does not own network protocol state or parse packets for this
capsule.

## Interface contract

| Operation | Meaning |
|---|---|
| health | service liveness |
| dhcp status | report configured address and lease state |

## Authority

The manifest grants driver discovery, IPC, and memory:
`CAPSULE_REQUIRED_CAPS = 0x00039`. It has no MMIO, IRQ, DMA, PIO, filesystem,
admin, debug, or direct raw hardware authority.

## Privacy and persistence

The capsule keeps runtime link, MAC, address, socket, and DHCP state in memory.
It does not persist packet contents, leases, peers, or traffic history.

## Runtime lifecycle

At startup it discovers one NIC driver endpoint, obtains the MAC address, builds
the interface state, registers network services, and enters the server loop.

## Failure model

Missing NIC, MAC query failure, interface setup failure, malformed request, and
empty poll states return deterministic protocol errors.

## Current implemented surface

- NIC endpoint discovery covers virtio-net, e1000, rtl8169, and rtl8139.
- Interface construction, polling, DHCP state, UDP port state, and service
  registration are present.
- Server handlers expose health and DHCP status.

## Wire format

Requests use the net-core service endpoint. Replies carry a status word followed
by operation-specific bytes for health and DHCP status.

## State ownership

The capsule owns selected NIC endpoint state, interface state, DHCP status, and
UDP port bookkeeping. NIC driver capsules own hardware rings and broker grants.

## Operating rules

- Select one NIC endpoint.
- Keep packet and address state transient.
- Do not parse hardware registers in this capsule.
- Keep firewall, capture, and persistent policy out of net-core.

## Release target

The finished capsule drives the shared network interface state for DHCP, DNS,
UDP, TCP, and sockets through a selected NIC driver without kernel packet policy.

## Release evidence

Release evidence is DHCP address acquisition, DNS query over the stack, TCP
connection lifecycle, packet TX/RX counters, and static proof that the kernel
does not own network protocol state.

## Release checklist

- NIC endpoint discovery works for each enabled NIC class.
- DHCP status is observable through IPC.
- DNS and TCP clients use this capsule path.
- Packet buffers remain bounded.
- Static gate confirms no kernel protocol ownership.

## Explicit non-goals today

No NIC register access, MMIO, IRQ ownership, DMA ownership, packet capture
store, firewall policy, TLS, persistent leases, or user identity policy lives
here.

## Verification

- Static gate: `bash nonos-ci/run-static-checks.sh`
- Build gate: `make -B nonos-mk-net-core`
- Runtime proof: boot with a NIC driver, acquire DHCP state, and complete DNS
  and TCP traffic through the net-core service path.
