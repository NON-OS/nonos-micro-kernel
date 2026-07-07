// NONOS Operating System (AGPL-3.0-or-later)
use crate::constants::queue::{
    RX_BUFFER_LEN, RX_DESC_COUNT, RX_STATUS_DD, RX_STATUS_EOP, TX_CMD_EOP, TX_CMD_IFCS,
    TX_BUFFER_LEN, TX_CMD_RS, TX_DESC_COUNT,
};
use crate::constants::MAX_ETHERNET_FRAME;
use crate::protocol::{decode_request, encode_response_header, Request, HDR_LEN};
use crate::queue::layout::{RxDesc, TxDesc};
use crate::queue::{RxRing, TxRing};

extern crate alloc;
use alloc::vec::Vec;

// The device writes the RX descriptor fields: status, errors, and length are
// hostile values. The proofs build a real RxRing over a host array of the
// real repr(C) descriptors and run the real consume. The gate must pass a
// frame length to the copy path only when the descriptor is a completed,
// error-free end-of-packet whose length fits an Ethernet frame, which in
// turn fits the per-slot buffer, so the handler's copy can never leave the
// slot.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn rx_ring(ring: &mut [RxDesc; RX_DESC_COUNT]) -> RxRing {
    RxRing::new(ring.as_mut_ptr() as u64, 0, 0)
}

#[test]
fn consume_passes_only_complete_bounded_frames() {
    let mut ring = [RxDesc::default(); RX_DESC_COUNT];
    for seed in 1..200_000u64 {
        let mut s = seed;
        let head = (xorshift(&mut s) % RX_DESC_COUNT as u64) as u16;
        let status = (xorshift(&mut s) & 0xff) as u8;
        let errors = (xorshift(&mut s) & 0xff) as u8;
        let length = (xorshift(&mut s) & 0xffff) as u16;

        ring[head as usize] =
            RxDesc { length, status, errors, ..RxDesc::default() };
        let mut rx = rx_ring(&mut ring);
        rx.head = head;

        match rx.consume() {
            None => {
                assert_eq!(status & RX_STATUS_DD, 0, "only an incomplete descriptor is skipped");
                assert_eq!(rx.head, head, "an incomplete descriptor must not advance the ring");
            }
            Some((idx, len)) => {
                assert_ne!(status & RX_STATUS_DD, 0);
                assert_eq!(idx, head, "the yielded slot is the one at the head");
                assert_eq!(rx.head, (head + 1) % RX_DESC_COUNT as u16);
                assert_eq!(ring[idx as usize].status, 0, "the descriptor must be cleared");
                assert_eq!(ring[idx as usize].errors, 0);
                if status & RX_STATUS_EOP == 0 || errors != 0 || length == 0 {
                    assert_eq!(len, 0, "a partial or errored frame must carry no length");
                } else if length as usize > MAX_ETHERNET_FRAME {
                    assert_eq!(len, 0, "an oversized frame must carry no length");
                } else {
                    assert_eq!(len, length);
                    assert!((1..=MAX_ETHERNET_FRAME).contains(&(len as usize)));
                }
            }
        }
    }
}

#[test]
#[allow(clippy::assertions_on_constants)] // guarding constant relations is the point
fn an_accepted_length_always_fits_the_slot_buffer() {
    // consume bounds len by MAX_ETHERNET_FRAME; the handler copies len bytes
    // from the slot at buffer_va(idx). This relation is what keeps that copy
    // inside the slot.
    assert!(MAX_ETHERNET_FRAME <= RX_BUFFER_LEN);
}

#[test]
fn slot_addresses_are_laid_out_by_buffer_len() {
    let rx = RxRing::new(0, 0x10_0000, 0x20_0000);
    for idx in 0..RX_DESC_COUNT as u16 {
        assert_eq!(rx.buffer_va(idx), 0x10_0000 + idx as u64 * RX_BUFFER_LEN as u64);
        assert_eq!(rx.buffer_phys(idx), 0x20_0000 + idx as u64 * RX_BUFFER_LEN as u64);
    }
}

#[test]
fn post_programs_exactly_the_tail_descriptor_and_wraps() {
    let mut ring = [TxDesc::default(); TX_DESC_COUNT];
    let mut tx = TxRing::new(ring.as_mut_ptr() as u64, 0, 0x40_0000);
    for round in 0..(2 * TX_DESC_COUNT as u16) {
        let want_idx = round % TX_DESC_COUNT as u16;
        let len = 64 + round;
        let idx = tx.post(len);
        assert_eq!(idx, want_idx, "post must fill the slot at the tail");
        assert_eq!(tx.tail, (want_idx + 1) % TX_DESC_COUNT as u16);
        let d = ring[idx as usize];
        assert_eq!(d.length, len);
        assert_eq!(d.cmd, TX_CMD_EOP | TX_CMD_IFCS | TX_CMD_RS);
        assert_eq!(d.status, 0, "the DD bit must be cleared when posting");
        assert_eq!(d.buffer_addr, 0x40_0000 + idx as u64 * TX_BUFFER_LEN as u64);
        assert!(!tx.done(idx), "a freshly posted descriptor is not done");
        ring[idx as usize].status = 1;
        assert!(tx.done(idx), "a DD-set descriptor is done");
        ring[idx as usize].status = 0;
    }
}

// The wire header.

#[test]
fn decode_never_panics_and_reads_fields_from_their_offsets() {
    const MAGIC: u32 = 0x4E45_3130; // the wire tag from protocol/header.rs
    for seed in 1..100_000u64 {
        let mut s = seed;
        let blen = (xorshift(&mut s) % 40) as usize;
        let mut buf: Vec<u8> = (0..blen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        if buf.len() >= 6 && seed % 2 == 0 {
            buf[0..4].copy_from_slice(&MAGIC.to_le_bytes());
            buf[4..6].copy_from_slice(&1u16.to_le_bytes());
        }
        if let Some(req) = decode_request(&buf) {
            assert!(buf.len() >= HDR_LEN);
            assert_eq!(req.op, u16::from_le_bytes([buf[6], buf[7]]));
            assert_eq!(req.flags, u16::from_le_bytes([buf[8], buf[9]]));
            assert_eq!(req.request_id, u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]));
            assert_eq!(req.payload_len, u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]));
        }
    }
}

#[test]
fn encoded_response_headers_decode_back_to_the_request_fields() {
    for seed in 1..20_000u64 {
        let mut s = seed;
        let request = Request {
            op: (xorshift(&mut s) & 0xffff) as u16,
            flags: (xorshift(&mut s) & 0xffff) as u16,
            request_id: (xorshift(&mut s) & 0xffff_ffff) as u32,
            payload_len: 0,
        };
        let payload_len = (xorshift(&mut s) & 0xffff_ffff) as u32;
        let mut out = [0u8; HDR_LEN];
        encode_response_header(&mut out, &request, payload_len);
        let back = decode_request(&out).expect("a response header carries the wire tag");
        assert_eq!(back.op, request.op);
        assert_eq!(back.flags, request.flags);
        assert_eq!(back.request_id, request.request_id);
        assert_eq!(back.payload_len, payload_len);
    }
}
