# usb_msc_proofs

Host-runnable proofs for the USB mass-storage driver's untrusted-input
parsers. The real driver source is included through `#[path]` and run on the
host.

## Configuration descriptor walk

The configuration descriptor is read from the device, so a hostile peripheral
chooses every byte: record lengths, types, the declared total, and the
endpoint fields. The proof establishes that the walk never panics on any
input, terminates on every input (a zero-length record is an error, not a
loop), rejects a declared total that exceeds the buffer or a record that runs
past the total, and never yields more than `MAX_BINDINGS` bindings. An
accepted binding is structurally sound: its IN endpoint carries the direction
bit, its OUT endpoint does not, and both are present. A Kani harness proves
totality and the binding invariants over every buffer within its bound.

Bound, stated plainly: the descriptor-walk harness covers every buffer up to
64 bytes with `#[kani::unwind(40)]`, which exceeds the maximum 33 loop
iterations a 64-byte total permits, so the bound does not truncate any path
within the modeled size. Descriptors longer than 64 bytes are covered by the
150,000-case runnable fuzz up to 600 bytes, not by the model checker.

## Command status wrapper

Every bulk-only transfer ends with a 13-byte CSW written by the device. The
proof establishes that parsing is strict: exactly 13 bytes, the `USBS`
signature, and a status of at most 2, with the tag and residue read
little-endian from their offsets. Anything else is rejected without a panic.
A Kani harness proves this over every input.

## Block requests and the CBW path

A block request arrives over IPC with an attacker-controlled LBA and block
count. The proof establishes that validation accepts exactly six bytes with a
block count between 1 and `MAX_TRANSFER_BLOCKS`, so the derived transfer
length cannot overflow, and that a validated request becomes a Bulk-Only
Transport CBW whose CDB carries the LBA and count big-endian at their SBC
offsets with the right opcode. Kani harnesses prove the validation bounds and
that a written CBW is byte-for-byte faithful to its fields.

## Wire protocol

The IPC frame must be exact: the proof establishes that a request parses only
when the declared payload length equals the bytes that follow the header, the
returned payload slice is precisely that region, and short, mistagged, or
misframed buffers are rejected. A Kani harness proves totality and exact
framing over every buffer within its bound.

## Run

```sh
cd userland/usb_msc_proofs
cargo test --release
cargo kani                # all-input totality and bounds (requires Kani)
```
