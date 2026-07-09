# driver_proofs

Host-runnable proofs for driver request parsers over untrusted input. The real
driver source is included through `#[path]` and run on the host.

## AHCI block I/O bounds

A block read or write request carries an attacker-controlled logical block
address and sector count. The proof establishes that the parser never panics, a
short request body is rejected, and an accepted request stays on the disk: the
sector count is within range and `lba + count` neither overflows nor exceeds the
device capacity. A Kani harness proves the bounds over every request body and
every capacity.

This is an isolation property. A request that passed the parser could otherwise
address storage outside the device, so the bound is what keeps one client's I/O
from reaching another's.

## Run

```sh
cd userland/driver_proofs
cargo test --release
cargo kani                # all-input bounds (requires Kani)
```
