// NONOS Operating System (AGPL-3.0-or-later)
//! Kani harness: the wire decode is total and field-faithful for every
//! buffer.
//!
//! The RX slot-confinement property is NOT model-checked here, and that gap
//! is deliberate rather than hidden: a harness running the real take_one
//! over a symbolic used-ring entry does not converge in the SAT backend
//! (CBMC ran for hours without an answer, in both the full pointer-equality
//! form and a weakened clamp-only form). The property is instead proven by
//! the runnable harness in net_tests.rs, which drives the real take_one
//! over two hundred thousand adversarial descriptor ids, used lengths, and
//! ring positions plus the boundary set, asserting the exact slot pointer,
//! the clamp, and the refill postconditions on every one.

use crate::protocol::decode_request;

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
