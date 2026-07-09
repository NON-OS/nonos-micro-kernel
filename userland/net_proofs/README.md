# net_proofs

Host-runnable proofs for the network capsules' untrusted-input parsers. Each
parser is included from its capsule through `#[path]` and run on the host against
large adversarial input sets. A network parser is a direct attack surface, so
the properties are safety properties: no panic, no out-of-bounds access, and
termination.

## DNS

The response parser terminates and never panics across every two-byte
compression-pointer value, including a self-referential pointer. The classic
compression-pointer loop, a standard denial of service in naive DNS code, is
absent.

## ICMP and ARP

ICMP parsing never panics and never returns a payload slice outside the input.
ARP parsing never panics and rejects every truncated packet.

## TCP

The segment parser never panics and never returns a payload slice outside the
segment. The out-of-order reassembly buffer never panics on hostile streams of
overlapping, out-of-order, and sequence-wrapping segments, and joins contiguous
data in order while stopping at a gap.

## DHCP

The reply parser walks a variable-length option list. It never panics, and an
option whose length field runs past the packet is rejected rather than read out
of bounds.

## Run

```sh
cd userland/net_proofs
cargo test --release
```
