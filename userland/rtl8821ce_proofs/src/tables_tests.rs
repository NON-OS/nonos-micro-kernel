// NONOS Operating System (AGPL-3.0-or-later)
//! Proof that the RTL8821CE card-enable power table drives the executor to
//! completion and issues the exact register program, in order. A modeled card
//! supplies the two hardware side effects the sequence waits on (the power
//! becomes ready after the switch is toggled, and the enable bit self-clears
//! when it settles), so the polls resolve; the proof asserts the sequence of
//! register writes matches the transcribed rtw88 values and that a card that
//! never becomes ready is reported as a failed bring-up.

use core::cell::RefCell;

use crate::pwr::{run_pwr_seq, CARD_ENABLE};
use crate::regs::Mmio;

const SIZE: usize = 0x1000;

struct Card {
    regs: RefCell<[u8; SIZE]>,
    write_offsets: RefCell<Vec<u16>>,
    dead: bool,
}

impl Card {
    fn new() -> Self {
        Self {
            regs: RefCell::new([0u8; SIZE]),
            write_offsets: RefCell::new(Vec::new()),
            dead: false,
        }
    }
}

impl Mmio for Card {
    fn read8(&self, off: usize) -> u8 {
        self.regs.borrow()[off]
    }
    fn write8(&self, off: usize, val: u8) {
        self.regs.borrow_mut()[off] = val;
        self.write_offsets.borrow_mut().push(off as u16);
        if self.dead {
            return;
        }
        // Hardware side effects the power sequence waits on: toggling the power
        // switch at 0x0075 makes the power-ready bit appear at 0x0006, and the
        // enable bit at 0x0005 self-clears once it settles.
        if off == 0x0075 {
            self.regs.borrow_mut()[0x0006] |= 0x02;
        }
        if off == 0x0005 && val & 0x01 != 0 {
            self.regs.borrow_mut()[0x0005] &= !0x01;
        }
    }
    fn read16(&self, _o: usize) -> u16 {
        0
    }
    fn write16(&self, _o: usize, _v: u16) {}
    fn read32(&self, _o: usize) -> u32 {
        0
    }
    fn write32(&self, _o: usize, _v: u32) {}
}

#[test]
fn card_enable_runs_the_exact_register_program() {
    let card = Card::new();
    assert!(run_pwr_seq(&card, CARD_ENABLE), "the power-on sequence completes");
    assert_eq!(
        *card.write_offsets.borrow(),
        vec![
            0x0005, 0x0075, 0x0075, 0x0006, 0x0005, 0x0005, 0x0005, 0x0020, 0x0074, 0x0022, 0x0062,
            0x0061, 0x007C,
        ],
        "the register writes match the rtw88 card-enable order"
    );
}

#[test]
fn a_card_that_never_powers_up_fails() {
    let mut card = Card::new();
    card.dead = true; // no side effects: the power-ready poll never resolves
    assert!(!run_pwr_seq(&card, CARD_ENABLE), "a dead card is reported, not hung");
}
