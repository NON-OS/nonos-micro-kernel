// NONOS Operating System (AGPL-3.0-or-later)
//! Known-behaviour proof for the firmware-download DDMA channel. A modeled
//! device clears the channel-owned bit as soon as a transfer is written (the
//! card completing the copy) and records every register write; the proofs then
//! assert the exact source/destination/control program: the length and
//! checksum-enable and owned bits on the control word, that the first chunk of
//! a section starts the checksum fresh while later chunks continue it, that a
//! channel that never releases ownership is reported failed, and that the
//! checksum status is read correctly.

use core::cell::RefCell;

use crate::fw::ddma::{
    enable_download, record_section, reset_checksum, transfer, DDMA_CHKSUM_CONT, DDMA_CHKSUM_EN,
    DDMA_CHKSUM_STS, DDMA_OWN, DMEM_CHKSUM_OK, DMEM_DW_OK, IMEM_CHKSUM_OK, IMEM_DW_OK, MCUFWDL_EN,
    OCPBASE_DMEM, REG_DDMA_CH0CTRL, REG_DDMA_CH0DA, REG_DDMA_CH0SA, REG_MCUFW_CTRL,
};
use crate::regs::Mmio;

// A device that completes each DDMA transfer immediately: it clears the owned
// bit the moment the control word is written, so a poll after a transfer sees
// the channel idle. `stuck` models a card that never finishes.
struct Card {
    ctrl: RefCell<u32>,
    fwctrl: RefCell<u16>,
    writes: RefCell<Vec<(usize, u32)>>,
    stuck: bool,
    chksum_bad: bool,
}

impl Card {
    fn new() -> Self {
        Self {
            ctrl: RefCell::new(0),
            fwctrl: RefCell::new(0),
            writes: RefCell::new(Vec::new()),
            stuck: false,
            chksum_bad: false,
        }
    }
}

impl Mmio for Card {
    fn read8(&self, off: usize) -> u8 {
        if off == REG_MCUFW_CTRL {
            *self.fwctrl.borrow() as u8
        } else {
            0
        }
    }
    fn write8(&self, off: usize, val: u8) {
        if off == REG_MCUFW_CTRL {
            let hi = *self.fwctrl.borrow() & 0xFF00;
            *self.fwctrl.borrow_mut() = hi | val as u16;
        }
    }
    fn read16(&self, off: usize) -> u16 {
        if off == REG_MCUFW_CTRL {
            *self.fwctrl.borrow()
        } else {
            0
        }
    }
    fn write16(&self, off: usize, val: u16) {
        if off == REG_MCUFW_CTRL {
            *self.fwctrl.borrow_mut() = val;
        }
    }
    fn read32(&self, off: usize) -> u32 {
        if off == REG_DDMA_CH0CTRL {
            *self.ctrl.borrow()
        } else {
            0
        }
    }
    fn write32(&self, off: usize, val: u32) {
        self.writes.borrow_mut().push((off, val));
        if off == REG_DDMA_CH0CTRL {
            // The card takes ownership then, unless stuck, completes: owned bit
            // clears, and the checksum status reflects success/failure.
            let mut done = val & !DDMA_OWN;
            if self.chksum_bad {
                done |= DDMA_CHKSUM_STS;
            } else {
                done &= !DDMA_CHKSUM_STS;
            }
            *self.ctrl.borrow_mut() = if self.stuck { val } else { done };
        }
    }
}

#[test]
fn enable_download_sets_the_bit_and_keeps_reserved() {
    let card = Card::new();
    *card.fwctrl.borrow_mut() = 0x2800; // some reserved bits in the 0x3800 mask
    enable_download(&card);
    let v = *card.fwctrl.borrow();
    assert_eq!(v & MCUFWDL_EN, MCUFWDL_EN, "download enable is set");
    assert_eq!(v & 0x2800, 0x2800, "reserved bits are preserved");
}

#[test]
fn first_chunk_programs_src_dst_and_a_fresh_checksum() {
    let card = Card::new();
    assert!(transfer(&card, 0x1111_2222, 0x8003_0000, 0x400, true));
    let w = card.writes.borrow();
    assert!(w.contains(&(REG_DDMA_CH0SA, 0x1111_2222)), "source programmed");
    assert!(w.contains(&(REG_DDMA_CH0DA, 0x8003_0000)), "destination programmed");
    let ctrl = w.iter().find(|(o, _)| *o == REG_DDMA_CH0CTRL).unwrap().1;
    assert_eq!(ctrl & 0x3FFFF, 0x400, "length in the low 18 bits");
    assert_ne!(ctrl & DDMA_CHKSUM_EN, 0, "checksum enabled");
    assert_ne!(ctrl & DDMA_OWN, 0, "ownership taken to start the copy");
    assert_eq!(ctrl & DDMA_CHKSUM_CONT, 0, "the first chunk does not continue a checksum");
}

#[test]
fn later_chunks_continue_the_checksum() {
    let card = Card::new();
    assert!(transfer(&card, 0x10, 0x8003_0000, 0x400, true));
    assert!(transfer(&card, 0x20, 0x8003_0400, 0x400, false));
    let w = card.writes.borrow();
    let ctrls: Vec<u32> =
        w.iter().filter(|(o, _)| *o == REG_DDMA_CH0CTRL).map(|(_, v)| *v).collect();
    assert_eq!(ctrls[0] & DDMA_CHKSUM_CONT, 0, "first chunk starts fresh");
    assert_ne!(ctrls[1] & DDMA_CHKSUM_CONT, 0, "second chunk continues the checksum");
}

#[test]
fn a_stuck_channel_is_reported_failed() {
    let mut card = Card::new();
    card.stuck = true;
    *card.ctrl.borrow_mut() = DDMA_OWN; // owned and never released
    assert!(!transfer(&card, 0x10, 0x8003_0000, 0x400, true), "a card that never finishes fails");
}

#[test]
fn a_good_imem_section_marks_download_and_checksum_ok() {
    let card = Card::new();
    reset_checksum(&card);
    assert!(transfer(&card, 0x10, 0x0003_0000, 0x400, true));
    // An IMEM section sits below OCPBASE_DMEM.
    let dst = OCPBASE_DMEM - 0x1000;
    assert!(record_section(&card, dst), "a clean transfer reports a good checksum");
    let v = *card.fwctrl.borrow();
    assert_eq!(v & IMEM_DW_OK, IMEM_DW_OK, "imem download-ok is set");
    assert_eq!(v & IMEM_CHKSUM_OK, IMEM_CHKSUM_OK, "imem checksum-ok is set");
    assert_eq!(v & (DMEM_DW_OK | DMEM_CHKSUM_OK), 0, "dmem bits untouched for an imem section");
}

#[test]
fn a_good_dmem_section_marks_the_dmem_bits() {
    let card = Card::new();
    reset_checksum(&card);
    assert!(transfer(&card, 0x10, 0x8020_0000, 0x400, true));
    assert!(record_section(&card, OCPBASE_DMEM), "a clean dmem transfer validates");
    let v = *card.fwctrl.borrow();
    assert_eq!(v & DMEM_DW_OK, DMEM_DW_OK, "dmem download-ok is set");
    assert_eq!(v & DMEM_CHKSUM_OK, DMEM_CHKSUM_OK, "dmem checksum-ok is set");
    assert_eq!(v & (IMEM_DW_OK | IMEM_CHKSUM_OK), 0, "imem bits untouched for a dmem section");
}

#[test]
fn a_bad_checksum_marks_download_ok_but_not_checksum_ok() {
    let mut bad = Card::new();
    bad.chksum_bad = true;
    assert!(transfer(&bad, 0x10, 0x0003_0000, 0x400, true));
    assert!(!record_section(&bad, OCPBASE_DMEM - 0x1000), "a bad checksum is reported");
    let v = *bad.fwctrl.borrow();
    assert_eq!(v & IMEM_DW_OK, IMEM_DW_OK, "download-ok still set on a checksum failure");
    assert_eq!(v & IMEM_CHKSUM_OK, 0, "checksum-ok stays clear on a failure");
}
