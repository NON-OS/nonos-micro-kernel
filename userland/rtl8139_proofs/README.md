# rtl8139_proofs

Host-runnable proofs for the RTL8139 driver's RX ring primitives. The real
driver source is included through `#[path]` and run on the host.

## The ring walk cannot leave the ring

The RTL8139 receives into a 32 KiB byte ring the device fills with
per-packet records: a status word, a raw length, then the frame. The driver
walks the ring by offset arithmetic, and the device chooses the lengths that
drive that arithmetic. Every access the walk makes goes through three
primitives, and the proofs establish their isolation property over a host
ring: a byte read lands at the wrapped offset for every offset up to
`usize::MAX`; a header read assembles its u16 little-endian and wraps at the
ring seam, taking its high byte from the start of the ring rather than past
its end; and the frame copy fills exactly the caller's buffer from wrapped
positions, stopping at the buffer's end even for a hostile length, with
guard bytes proven untouched. Kani harnesses prove totality and confinement
over every offset and length within their bounds, with every dereference
checked by the model checker.

Bounds, stated plainly: the byte-read harness covers every offset up to
`usize::MAX` unbounded; the u16 and copy harnesses assume the walk's own
offset invariant (`start < 4 * RX_BUF_DATA_BYTES`, maintained by `advance`
wrapping modulo the ring size); the copy harness uses an 8-byte output with
`#[kani::unwind(10)]`, which exceeds its 8 loop iterations, and arbitrary
lengths. Larger outputs are covered by the runnable tests with 64-byte and
96-byte buffers.

The gate above the primitives (`read_frame`) rejects bad status words, short
raw lengths, and frames larger than the Ethernet maximum or the caller's
buffer before any copy. It reaches the hardware through PIO and IRQ
syscalls, so it is not host-runnable; a constant-relation test pins the gate
bound inside the ring size.

## Wire header

Header decoding is total, rejects short or mistagged buffers, reads every
field from its wire offset in little-endian order, and an encoded response
header decodes back to the request's fields. A Kani harness proves totality
and field faithfulness.

## Run

```sh
cd userland/rtl8139_proofs
cargo test --release
cargo kani                # all-input ring confinement (requires Kani)
```
