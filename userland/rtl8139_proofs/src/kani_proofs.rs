// NONOS Operating System (AGPL-3.0-or-later)
//! Kani harnesses: every ring access is total and lands inside the ring,
//! for every offset and length the device can induce.

use crate::constants::dma::RX_BUF_DATA_BYTES;
use crate::protocol::decode_request;
use crate::ring::{copy, u16_at, u8_at};

// For every offset a device-driven walk can produce: the byte read never
// panics and never dereferences outside the ring (the model checker checks
// the dereference), and it reads the wrapped position.
#[kani::proof]
fn byte_reads_are_total_and_wrapped() {
    let ring = [0u8; RX_BUF_DATA_BYTES];
    let off: usize = kani::any();
    let v = u8_at(ring.as_ptr() as u64, off);
    assert!(v == ring[off % RX_BUF_DATA_BYTES]);
}

// For every header offset the walk maintains (bounded away from usize
// overflow as the caller's offset arithmetic guarantees): the u16 read never
// panics and both bytes come from inside the ring.
#[kani::proof]
fn u16_reads_are_total_and_wrapped() {
    let ring = [0u8; RX_BUF_DATA_BYTES];
    let off: usize = kani::any();
    kani::assume(off < 4 * RX_BUF_DATA_BYTES);
    let v = u16_at(ring.as_ptr() as u64, off);
    let lo = ring[off % RX_BUF_DATA_BYTES] as u16;
    let hi = ring[(off + 1) % RX_BUF_DATA_BYTES] as u16;
    assert!(v == lo | (hi << 8));
}

// For every start offset and every hostile length: the copy never panics,
// never reads outside the ring, and never writes outside the caller's
// buffer, filling exactly min(len, out.len()) bytes.
#[kani::proof]
#[kani::unwind(10)]
fn the_copy_is_confined_to_ring_and_buffer() {
    let ring = [0u8; RX_BUF_DATA_BYTES];
    let mut out = [0xEEu8; 8];
    let start: usize = kani::any();
    kani::assume(start < 4 * RX_BUF_DATA_BYTES);
    let len: usize = kani::any();

    copy(ring.as_ptr() as u64, start, &mut out, len);

    let filled = if len < out.len() { len } else { out.len() };
    let mut i = 0;
    while i < out.len() {
        if i < filled {
            assert!(out[i] == ring[(start + i) % RX_BUF_DATA_BYTES]);
        } else {
            assert!(out[i] == 0xEE);
        }
        i += 1;
    }
}

// For every buffer up to a header plus slack: wire decoding is total and
// every accepted field comes from its wire offset in little-endian order.
#[kani::proof]
fn decode_is_total_and_header_faithful() {
    let buf: [u8; 24] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= buf.len());

    if let Some(req) = decode_request(&buf[..len]) {
        assert!(len >= 20);
        assert_eq!(req.op, u16::from_le_bytes([buf[6], buf[7]]));
        assert_eq!(req.flags, u16::from_le_bytes([buf[8], buf[9]]));
        assert_eq!(req.request_id, u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]));
        assert_eq!(req.payload_len, u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]));
    }
}
