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

## Bring-up: on the air under a drawn address, or not at all

The bring-up files are included the same way and pointed at a window in host
memory standing in for BAR0, with a part on a second thread completing the
reset. The station address is drawn per bring-up rather than read out of the
EEPROM, because the EEPROM address is the one identifier an amnesic machine
would otherwise announce to every network it joins. With the entropy source
switched off the bring-up fails after the reset: RAL0/RAH0 still hold the
EEPROM address, and neither the receiver nor the transmitter was enabled. A
mutation that falls back to any address without entropy kills that test.
With entropy the drawn address is locally administered unicast, RAL0/RAH0
hold it with the valid bit set, the 128 multicast table entries are cleared,
both rings are named to the part at the driver's layout, the receive ring is
primed with its buffer addresses, and both directions end up enabled. The
reset handshake is proven against a part that completes it and against one
that never does.

The order of the enable against the address write inside one bring-up is a
property of the source and is not asserted here: a window reads back final
state, and a part sampling from another thread misses a gap of a few
instructions.
