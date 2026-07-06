# virtio_net_proofs

Host-runnable proofs for the virtio-net driver's device-facing RX path and
its untrusted protocol input. The real driver source is included through
`#[path]` and run on the host.

## The RX used ring, driven by a hostile device

Every received frame arrives as a used-ring entry the device writes: a
descriptor id and a used length, both fully attacker-chosen. The proofs build
a real `RxQueue` over host memory with the real constructor, write used-ring
entries the way a malicious device would, and run the real `take_one`.

The property is isolation. For every descriptor id, used length, and ring
position: `take_one` never panics and never touches memory outside the queue
region and buffer area; a returned frame lies exactly in the payload area of
the slot the id selects, after the virtio-net header, never in another slot
or outside the buffers; a used length at or below the header yields an empty
frame; an oversized used length is clamped to the slot payload; a wild
descriptor id is reduced into the slot range; and the drained slot is handed
back through the avail ring on the next call.

Stated plainly: this property is proven by the runnable harness over two
hundred thousand adversarial entries plus the boundary set, not by the model
checker. A Kani harness for it does not converge in the SAT backend (CBMC
ran for hours in both the pointer-equality and the weakened clamp-only
form), so the crate ships without one rather than with a bound it cannot
honestly call all-input. The wire decode below is model-checked.

## The TX length gate

The TX copy is kept in bounds by constant relations: the `tx_packet` handler
rejects frames above the Ethernet maximum, `send` pads short frames to the
Ethernet minimum, and both extremes must fit behind the virtio-net header in
a TX slot. A test asserts those relations against the real constants, so
widening the MTU or shrinking the buffers fails the proof before it ships.

## Wire header

Header decoding is total, rejects short or mistagged buffers, reads every
field from its wire offset in little-endian order, and an encoded response
header decodes back to the request's fields. A Kani harness proves totality
and field faithfulness.

## Run

```sh
cd userland/virtio_net_proofs
cargo test --release
cargo kani                # all-input slot confinement (requires Kani)
```
