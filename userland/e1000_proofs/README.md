# e1000_proofs

Host-runnable proofs for the e1000 driver's device-facing descriptor rings.
The real driver source is included through `#[path]` and run on the host.

## RX: hostile descriptors cannot leave the ring or oversize a copy

The NIC writes the RX descriptor fields: status, errors, and length are
device-chosen. The proofs build a real `RxRing` over a host array of the real
`repr(C)` descriptors and run the real `consume`. For every device-written
field and every reachable head: an incomplete descriptor yields nothing and
does not advance the ring; a completed one yields exactly the head slot,
clears the descriptor, and advances the head in range; and a nonzero length
reaches the copy path only for an error-free end-of-packet frame bounded by
the Ethernet maximum. A constant-relation test pins that maximum inside the
per-slot buffer, so the handler's copy from `buffer_va(idx)` cannot leave the
slot.

## TX: posting stays inside the ring

`post` fills exactly the tail slot with the frame length and `EOP|IFCS|RS`,
clears the completion bit, and wraps the tail in range; `done` reports the
DD bit. Proven over every length and every reachable tail.

## Wire header

Header decoding is total, rejects short or mistagged buffers, reads every
field from its wire offset in little-endian order, and an encoded response
header decodes back to the request's fields.

Kani harnesses prove all three claims over all inputs within their bounds,
with every ring dereference checked by the model checker.

## Run

```sh
cd userland/e1000_proofs
cargo test --release
cargo kani                # all-input ring bounds (requires Kani)
```
