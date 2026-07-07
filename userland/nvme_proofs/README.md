# nvme_proofs

Host-runnable proofs for the NVMe driver's untrusted-input parsers. The real
driver source is included through `#[path]` and run on the host.

## Block I/O bounds

A block read or write request carries an attacker-controlled logical block
address and sector count over IPC. The proof establishes that the parser never
panics, a short request body is rejected, and an accepted request stays on the
disk: the sector count is within range and `lba + count` neither overflows nor
exceeds the device capacity. A Kani harness proves the bounds over every
request body and every capacity. This is the same isolation property the AHCI
proof establishes for the other block driver.

## Wire header decode

The IPC request header is decoded from attacker-controlled bytes. The proof
establishes that decoding is total, that a short buffer or a wrong magic or
version is rejected, that every accepted field is read from its wire offset in
little-endian order, and that an encoded response header decodes back to the
request's fields. A Kani harness proves totality and field faithfulness over
every buffer up to a full header plus slack.

## Device-controlled identify and SMART pages

The identify controller page, the identify namespace page, and the SMART log
page are written by the device over DMA, so a hostile controller chooses every
byte. The driver always parses them from fixed-size buffers: 4096 bytes for
identify, 512 for the SMART log page. Over those buffer sizes the proofs
establish that parsing never panics for any contents and that every field is
read from its NVMe spec offset in little-endian order. For the namespace page
the device also steers which of the 16 LBA-format slots is read; the proof
covers every slot and shows the reported block size is zero or a power of two
(an absurd LBA shift yields zero rather than a wrapped shift) and the
formatted-count arithmetic saturates instead of wrapping. Kani harnesses prove
totality over every page of the parsed sizes.

The parsers index fixed offsets without a length guard, so these guarantees
hold for the buffer sizes the driver actually passes, not for arbitrary short
slices. The call sites keep that precondition: both identify paths return a
4096-byte DMA slice and the SMART path a 512-byte one.

## Run

```sh
cd userland/nvme_proofs
cargo test --release
cargo kani                # all-input totality and bounds (requires Kani)
```
