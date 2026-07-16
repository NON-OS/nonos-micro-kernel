// NONOS Operating System (AGPL-3.0-or-later)
//! Known-behaviour proofs for the H2C mailbox: a command waits for its mailbox
//! to drain, writes the extended word before the main word into the right box
//! registers, and the cursor advances round-robin across the four mailboxes. A
//! modeled device reports mailbox occupancy and records writes.

use core::cell::RefCell;

use crate::h2c::{build_iqk, H2c};
use crate::regs::Mmio;

#[test]
fn iqk_packet_carries_the_header_and_flags() {
    let p = build_iqk(false, true, 5);
    let w0 = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    let w1 = u32::from_le_bytes([p[4], p[5], p[6], p[7]]);
    let w2 = u32::from_le_bytes([p[8], p[9], p[10], p[11]]);
    assert_eq!(w0 & 0x7F, 0x01, "packet category");
    assert_eq!((w0 >> 8) & 0xFF, 0xFF, "packet command id");
    assert_eq!((w0 >> 16) & 0xFFFF, 0x0E, "IQK sub-command id");
    assert_eq!(w1 & 0xFFFF, 9, "total length (header + 1)");
    assert_eq!((w1 >> 16) & 0xFFFF, 5, "sequence number");
    assert_eq!(w2 & 1, 0, "clear flag");
    assert_eq!((w2 >> 1) & 1, 1, "segmented-IQK flag");
    assert_eq!(p.len(), 32, "the packet is 32 bytes");
}

const REG_HMETFR: usize = 0x01CC;
const REG_HMEBOX0: usize = 0x01D0;
const REG_HMEBOX0_EX: usize = 0x01F0;

struct Card {
    // Bit mask of mailboxes the firmware still owns (busy).
    busy: u8,
    writes: RefCell<Vec<(usize, u32)>>,
}

impl Card {
    fn new(busy: u8) -> Self {
        Self { busy, writes: RefCell::new(Vec::new()) }
    }
}

impl Mmio for Card {
    fn read8(&self, off: usize) -> u8 {
        if off == REG_HMETFR {
            self.busy
        } else {
            0
        }
    }
    fn write8(&self, _o: usize, _v: u8) {}
    fn read16(&self, _o: usize) -> u16 {
        0
    }
    fn write16(&self, _o: usize, _v: u16) {}
    fn read32(&self, _o: usize) -> u32 {
        0
    }
    fn write32(&self, off: usize, val: u32) {
        self.writes.borrow_mut().push((off, val));
    }
}

#[test]
fn a_command_writes_ext_then_main_into_box_zero() {
    let card = Card::new(0); // all mailboxes free
    let mut h2c = H2c::new();
    assert!(h2c.send(&card, 0x1122_3344, 0xAABB_CCDD), "command sent");
    let w = card.writes.borrow();
    // The extended word lands first, then the main word.
    assert_eq!(w[0], (REG_HMEBOX0_EX, 0xAABB_CCDD), "extended word into box 0 ex");
    assert_eq!(w[1], (REG_HMEBOX0, 0x1122_3344), "main word into box 0");
}

#[test]
fn the_cursor_advances_round_robin() {
    let card = Card::new(0);
    let mut h2c = H2c::new();
    for _ in 0..4 {
        assert!(h2c.send(&card, 0, 0));
    }
    let w = card.writes.borrow();
    // Four commands touch boxes 0,1,2,3 in turn (main word each at +4 stride).
    let mains: Vec<usize> =
        w.iter().filter(|(o, _)| (0x1D0..0x1E0).contains(o)).map(|(o, _)| *o).collect();
    assert_eq!(mains, vec![REG_HMEBOX0, REG_HMEBOX0 + 4, REG_HMEBOX0 + 8, REG_HMEBOX0 + 12]);
}

#[test]
fn a_stuck_mailbox_fails_the_send() {
    // Mailbox 0 is permanently owned by the firmware: its bit never clears.
    let card = Card::new(0b0001);
    let mut h2c = H2c::new();
    assert!(!h2c.send(&card, 0, 0), "a mailbox that never drains fails");
    assert!(card.writes.borrow().is_empty(), "nothing is written to a busy mailbox");
}

#[test]
fn a_free_mailbox_is_used_even_when_others_are_busy() {
    // Boxes 1,2,3 busy but box 0 (the cursor's first) is free.
    let card = Card::new(0b1110);
    let mut h2c = H2c::new();
    assert!(h2c.send(&card, 0x5, 0x6), "box 0 is free and used");
}
