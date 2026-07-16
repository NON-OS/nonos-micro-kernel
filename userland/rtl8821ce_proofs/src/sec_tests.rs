// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proofs for the hardware security CAM writer: a CCMP key is
//! written into its eight cells with the valid bit landing last, the key bytes
//! split four per cell, the peer address across cells 0 and 1, and the key
//! index and type in cell 0; a clear zeroes cell 0. A modeled device records the
//! write/command register pairs.

use core::cell::RefCell;

use crate::regs::Mmio;
use crate::sec::{clear_cam, write_cam, Key, CAM_AES};

const SEC_CMD: usize = 0x0670;
const SEC_WRITE: usize = 0x0674;

struct Card {
    writes: RefCell<Vec<(usize, u32)>>,
}

impl Card {
    fn new() -> Self {
        Self { writes: RefCell::new(Vec::new()) }
    }
    // The content written just before a given command value.
    fn content_for_cmd(&self, cmd: u32) -> Option<u32> {
        let w = self.writes.borrow();
        for i in 1..w.len() {
            if w[i] == (SEC_CMD, cmd) && w[i - 1].0 == SEC_WRITE {
                return Some(w[i - 1].1);
            }
        }
        None
    }
}

impl Mmio for Card {
    fn read8(&self, _o: usize) -> u8 {
        0
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

const WRITE_ENABLE: u32 = 1 << 16;
const POLLING: u32 = 1 << 31;

fn cmd(entry: u32) -> u32 {
    WRITE_ENABLE | POLLING | entry
}

#[test]
fn a_ccmp_pairwise_key_fills_its_eight_cells() {
    let card = Card::new();
    let key = Key {
        key: [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD,
            0xEE, 0xFF,
        ],
        mac: [0xA0, 0xB1, 0xC2, 0xD3, 0xE4, 0xF5],
        key_idx: 0,
        pairwise: true,
    };
    // Key index 2 -> CAM base 2 << 3 = 16.
    write_cam(&card, 2, CAM_AES, &key);
    let base = 16;

    // Cell 0: key index, type (AES=4) at bit 2, valid at bit 15, MAC[0..2] high.
    let c0 = card.content_for_cmd(cmd(base)).expect("cell 0 written");
    assert_eq!(c0 & 0x3, 0, "key index 0");
    assert_eq!((c0 >> 2) & 0x7, CAM_AES as u32, "AES/CCMP key type");
    assert_eq!((c0 >> 6) & 0x1, 0, "pairwise: not a group key");
    assert_eq!((c0 >> 15) & 0x1, 1, "valid bit set");
    assert_eq!((c0 >> 16) & 0xFF, 0xA0, "MAC byte 0");
    assert_eq!((c0 >> 24) & 0xFF, 0xB1, "MAC byte 1");
    // Cell 1: the rest of the MAC.
    assert_eq!(card.content_for_cmd(cmd(base + 1)), Some(0xF5E4_D3C2), "MAC bytes 2..6");
    // Cells 2..5: the 16-byte key, four bytes each, little-endian.
    assert_eq!(card.content_for_cmd(cmd(base + 2)), Some(0x3322_1100), "key bytes 0..4");
    assert_eq!(card.content_for_cmd(cmd(base + 5)), Some(0xFFEE_DDCC), "key bytes 12..16");
    // Cells 6 and 7 are unused.
    assert_eq!(card.content_for_cmd(cmd(base + 7)), Some(0), "cell 7 unused");
}

#[test]
fn the_valid_bit_is_written_last() {
    let card = Card::new();
    let key = Key { key: [0; 16], mac: [0; 6], key_idx: 1, pairwise: true };
    write_cam(&card, 0, CAM_AES, &key);
    // The command writes count from cell 7 down to cell 0, so cell 0 (with the
    // valid bit) is the final command.
    let cmds: Vec<u32> =
        card.writes.borrow().iter().filter(|(o, _)| *o == SEC_CMD).map(|(_, v)| *v).collect();
    assert_eq!(*cmds.last().unwrap(), cmd(0), "cell 0 armed last");
    assert_eq!(cmds.len(), 8, "all eight cells written");
}

#[test]
fn a_group_key_sets_the_group_bit() {
    let card = Card::new();
    let key = Key { key: [0; 16], mac: [0xFF; 6], key_idx: 2, pairwise: false };
    write_cam(&card, 5, CAM_AES, &key);
    let c0 = card.content_for_cmd(cmd(5 << 3)).unwrap();
    assert_eq!((c0 >> 6) & 0x1, 1, "group key bit set");
    assert_eq!(c0 & 0x3, 2, "key index 2");
}

#[test]
fn clearing_a_key_zeroes_cell_zero() {
    let card = Card::new();
    clear_cam(&card, 3);
    let w = card.writes.borrow();
    assert_eq!(w[0], (SEC_WRITE, 0), "zero written");
    assert_eq!(w[1], (SEC_CMD, cmd(3 << 3)), "command targets cell 0 of entry 3");
}
