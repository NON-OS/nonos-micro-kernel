# capsule_net_dns

## Role

`capsule_net_dns` is the DNS resolver capsule. It builds DNS queries, sends
them over `net.udp` to a configured upstream resolver, parses responses, and
keeps a bounded runtime cache.

```text
application / sockets capsule
    |
    | resolve IPC
    v
net.dns -- query/cache/response parser --> net.udp
```

## Microkernel contract

The capsule is a userland IPC service:

- `MkIpcRecv` receives requests on `service:4450:net.dns`.
- `MkIpcSend` replies through `reply:4451:endpoint.4294967370`.
- Its wire magic is `NDNS`.
- Its endpoint name is `net.dns`.
- Its kernel mirror target is `src/network/dns_capsule`.

The kernel does not parse DNS, keep resolver configuration, or cache names.

## Interface contract

| Operation | Meaning |
|---|---|
| `OP_HEALTHCHECK` | server liveness |
| `OP_RESOLVE_A` / `OP_RESOLVE_AAAA` | resolve host names through upstream DNS |
| `OP_FLUSH_CACHE` | clear runtime answer cache |
| `OP_SET_UPSTREAM` | set resolver endpoint policy |

## Authority

The manifest grants IPC and memory only:
`CAPSULE_REQUIRED_CAPS = 0x00018`. It has no direct network-device,
filesystem, driver, MMIO, IRQ, DMA, PIO, admin, or debug authority.

## Privacy and persistence

DNS queries can reveal user intent. The cache is runtime-only and should be
bounded by TTL and capacity. The capsule does not persist lookup history,
write resolver logs, or expose query history to unrelated capsules.

## Runtime lifecycle

The capsule accepts upstream configuration, builds DNS queries, sends them over
`net.udp`, parses responses, fills a bounded TTL cache, and answers lookup
requests until exit.

## Failure model

No upstream, timeout, NXDOMAIN, SERVFAIL, invalid name, malformed response, and
cache miss return deterministic errors. Query history is not persisted.

## Current implemented surface

- DNS header and record-type structures are present.
- DNS name encode/decode helpers are present.
- Query builder and response parser are present.
- A runtime answer cache is present.
- Protocol constants cover health, A lookup, AAAA lookup, cache flush, and
  upstream resolver configuration.

## Wire format

Requests use the `NDNS` protocol magic. Resolve requests carry a DNS name up to
255 bytes and a record-family operation. Replies carry status and one or more
address records. Upstream DNS packets remain RFC 1035 wire format over UDP.

## State ownership

The capsule owns upstream resolver config, runtime cache entries, TTL expiry,
and pending query state. `net.udp` owns datagram transport. The kernel owns no
resolver cache.

## Operating rules

- Bound names to DNS wire limits.
- Honor TTL when caching.
- Do not persist query history.
- Return deterministic NXDOMAIN, SERVFAIL, timeout, and invalid-name errors.

## Release target

The finished DNS capsule owns upstream configuration, A/AAAA query dispatch
over `net.udp`, response validation, TTL-bounded cache behavior, cache flush,
timeout handling, and deterministic NXDOMAIN/SERVFAIL errors. It does not
store query history or move resolver policy into the kernel.

## Release evidence

Release evidence is A/AAAA resolve validation, cache hit by TTL, cache flush,
timeout behavior, NXDOMAIN/SERVFAIL behavior, and static proof that DNS parsing
stays in userland.

## Release checklist

- A and AAAA resolve validation passes.
- TTL cache hit and flush behavior are tested.
- NXDOMAIN, SERVFAIL, timeout, and invalid-name errors are tested.
- Static gate confirms no kernel DNS parser.

## Explicit non-goals today

No authoritative server, DNSSEC validation, DoH/DoT, mDNS, persistent cache,
hosts file, network fetcher outside UDP, or kernel DNS resolver lives here.
The UDP client, server loop, timeout handling, and cache maintenance tick need
promotion before runtime use.

## Verification

- Static gate: `bash nonos-ci/run-static-checks.sh`
- Build gate, once `src/main.rs` lands: `make -B nonos-mk-net-dns`
- Runtime proof: set upstream, resolve A/AAAA through UDP, cache hit by TTL,
  flush cache, and no kernel DNS parsing.
