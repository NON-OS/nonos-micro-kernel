// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the host-command queue. The header/sequence encoding and the TFD
//! ring math are pure, so they are checked exactly. The doorbell is driven
//! against a modeled device that records the write, so the exact register and
//! value are validated without hardware.

use core::cell::RefCell;

use crate::constants::{HBUS_TARG_WRPTR, TFD_QUEUE_SIZE};
use crate::hcmd::doorbell;
use crate::hcmd::header::{cmd_header, make_sequence, seq_index, seq_queue, CMD_HEADER_LEN};
use crate::hcmd::ring::{advance, is_full, used};
use crate::regs::Mmio;

struct Rec {
    writes: RefCell<Vec<(usize, u32)>>,
}

impl Mmio for Rec {
    fn read32(&self, _off: usize) -> u32 {
        0
    }
    fn write32(&self, off: usize, val: u32) {
        self.writes.borrow_mut().push((off, val));
    }
}

#[test]
fn cmd_header_encodes_cmd_group_and_sequence() {
    let seq = make_sequence(4, 7);
    let h = cmd_header(0x9c, 0x00, seq);
    assert_eq!(h.len(), CMD_HEADER_LEN);
    assert_eq!(h[0], 0x9c, "command id");
    assert_eq!(h[1], 0x00, "legacy group");
    assert_eq!(u16::from_le_bytes([h[2], h[3]]), seq, "sequence is little-endian");
    assert_eq!(seq_queue(seq), 4);
    assert_eq!(seq_index(seq), 7);
}

#[test]
fn sequence_roundtrips_queue_and_index() {
    for q in 0..40u8 {
        for i in [0u8, 1, 7, 128, 255] {
            let s = make_sequence(q, i);
            assert_eq!(seq_queue(s), q & 0x1F, "queue is 5 bits");
            assert_eq!(seq_index(s), i);
        }
    }
}

#[test]
fn ring_advance_wraps_at_queue_size() {
    assert_eq!(advance(0), 1);
    assert_eq!(advance(TFD_QUEUE_SIZE - 1), 0, "the last index wraps to zero");
    let mut p = 0usize;
    for _ in 0..TFD_QUEUE_SIZE {
        p = advance(p);
    }
    assert_eq!(p, 0, "a full lap returns to the start");
}

#[test]
fn ring_is_full_when_a_write_would_catch_read() {
    assert!(!is_full(0, 0), "an empty ring is not full");
    assert!(is_full(TFD_QUEUE_SIZE - 1, 0), "one slot before the reader is full");
    assert_eq!(used(5, 2), 3);
    assert_eq!(used(1, TFD_QUEUE_SIZE - 1), 2, "used wraps across the ring");
    assert_eq!(used(3, 3), 0, "read == write means empty");
}

#[test]
fn doorbell_writes_queue_and_index_to_the_write_pointer() {
    let rec = Rec { writes: RefCell::new(Vec::new()) };
    doorbell::ring(&rec, 4, 7);
    let w = rec.writes.borrow();
    assert_eq!(w.len(), 1, "exactly one register write");
    assert_eq!(w[0].0, HBUS_TARG_WRPTR, "the write-pointer register");
    assert_eq!(w[0].1, (4 << 8) | 7, "queue in the high byte, index in the low byte");
}
