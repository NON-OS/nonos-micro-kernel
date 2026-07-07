// NONOS Operating System (AGPL-3.0-or-later)
//! Kani harnesses: the descriptor-ring state machines are total and bounded
//! for every device-written field.

use crate::constants::queue::{RX_DESC_COUNT, RX_STATUS_DD, TX_DESC_COUNT};
use crate::constants::MAX_ETHERNET_FRAME;
use crate::protocol::decode_request;
use crate::queue::layout::{RxDesc, TxDesc};
use crate::queue::{RxRing, TxRing};

// For every device-written status, errors, and length, and every reachable
// head: consume never touches memory outside the ring, only a completed
// descriptor advances the head, the yielded slot is the head slot, and a
// nonzero yielded length is bounded by the Ethernet maximum, which fits the
// slot buffer.
#[kani::proof]
fn consume_is_total_and_bounded() {
    let mut ring = [RxDesc::default(); RX_DESC_COUNT];
    let head: u16 = kani::any();
    kani::assume(head < RX_DESC_COUNT as u16);
    let status: u8 = kani::any();
    let errors: u8 = kani::any();
    let length: u16 = kani::any();
    ring[head as usize] = RxDesc { length, status, errors, ..RxDesc::default() };

    let mut rx = RxRing::new(ring.as_mut_ptr() as u64, 0, 0);
    rx.head = head;

    match rx.consume() {
        None => {
            assert!(status & RX_STATUS_DD == 0);
            assert!(rx.head == head);
        }
        Some((idx, len)) => {
            assert!(status & RX_STATUS_DD != 0);
            assert!(idx == head);
            assert!(rx.head < RX_DESC_COUNT as u16);
            assert!(len == 0 || (len as usize) <= MAX_ETHERNET_FRAME);
        }
    }
}

// For every length and every reachable tail: post fills exactly the tail
// slot, clears its completion bit, and keeps the tail inside the ring.
#[kani::proof]
fn post_is_total_and_stays_in_the_ring() {
    let mut ring = [TxDesc::default(); TX_DESC_COUNT];
    let tail: u16 = kani::any();
    kani::assume(tail < TX_DESC_COUNT as u16);
    let len: u16 = kani::any();

    let mut tx = TxRing::new(ring.as_mut_ptr() as u64, 0, 0);
    tx.tail = tail;
    let idx = tx.post(len);
    assert!(idx == tail);
    assert!(tx.tail < TX_DESC_COUNT as u16);
    assert!(ring[idx as usize].length == len);
    assert!(ring[idx as usize].status == 0);
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
