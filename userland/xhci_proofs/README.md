# xhci_proofs

Host-runnable proofs for the xHCI driver's TRB layer. The real driver source
is included through `#[path]` and run on the host.

## TRB field algebra

Everything the driver tells the controller, and everything the controller
answers, crosses the rings as 16-byte TRBs. The device writes event TRBs, so
the extraction of completion code, slot id, type, cycle, and pointer must
read exactly the specification fields; a wrong shift silently addresses the
wrong device slot or misreads a transfer result. The proofs establish that
every getter reads its spec bits, that every setter writes exactly its field
and leaves every other bit untouched, and that getters invert setters, for
every TRB value. Kani proves the algebra over all inputs.

## Control-transfer encodings

The setup, data, and status stage builders are checked against the xHCI
specification section 6.4.1 and the USB GET_DESCRIPTOR layout: the setup
stage carries `bmRequestType 0x80`, `bRequest 0x06`, the descriptor type and
index in `wValue`, the length in `wLength`, an 8-byte transfer length,
immediate data, and the IN transfer type; the data stage carries the buffer
address split across the low words, the 17-bit length, and the IN direction;
the status stage interrupts on completion. The cycle bit, the ring ownership
handshake, follows the caller's argument in every stage. A Kani harness
proves the data stage faithful for every address, length, and cycle.

## Wire header

Header decoding is total, rejects short or mistagged buffers, reads every
field from its wire offset in little-endian order, and an encoded response
header decodes back to the request's fields. A Kani harness proves totality
and field faithfulness.

## Run

```sh
cd userland/xhci_proofs
cargo test --release
cargo kani                # all-input TRB algebra (requires Kani)
```
