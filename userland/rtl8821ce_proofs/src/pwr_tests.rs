// NONOS Operating System (AGPL-3.0-or-later)
//! Known-behaviour proofs for the power-sequence executor. A modeled register
//! file records every write and can be told to satisfy a poll; the proofs then
//! assert that a WRITE step is a read-modify-write that preserves untouched
//! bits, that POLL waits for exactly its masked value and reports a dead card
//! when it never appears, that END stops the sequence, and that a table runs its
//! steps in order.

use core::cell::RefCell;

use crate::pwr::command::PwrCmd;
use crate::pwr::run_pwr_seq;
use crate::regs::Mmio;

const SIZE: usize = 0x1000;

struct MockMmio {
    regs: RefCell<[u8; SIZE]>,
    writes: RefCell<Vec<(usize, u8)>>,
}

impl MockMmio {
    fn new() -> Self {
        Self { regs: RefCell::new([0u8; SIZE]), writes: RefCell::new(Vec::new()) }
    }
    fn preset(&self, off: usize, val: u8) {
        self.regs.borrow_mut()[off] = val;
    }
}

impl Mmio for MockMmio {
    fn read8(&self, off: usize) -> u8 {
        self.regs.borrow()[off]
    }
    fn write8(&self, off: usize, val: u8) {
        self.regs.borrow_mut()[off] = val;
        self.writes.borrow_mut().push((off, val));
    }
    fn read16(&self, off: usize) -> u16 {
        let r = self.regs.borrow();
        u16::from_le_bytes([r[off], r[off + 1]])
    }
    fn write16(&self, off: usize, val: u16) {
        let b = val.to_le_bytes();
        self.regs.borrow_mut()[off] = b[0];
        self.regs.borrow_mut()[off + 1] = b[1];
    }
    fn read32(&self, off: usize) -> u32 {
        let r = self.regs.borrow();
        u32::from_le_bytes([r[off], r[off + 1], r[off + 2], r[off + 3]])
    }
    fn write32(&self, off: usize, val: u32) {
        let b = val.to_le_bytes();
        for (i, byte) in b.iter().enumerate() {
            self.regs.borrow_mut()[off + i] = *byte;
        }
    }
}

#[test]
fn write_is_a_masked_read_modify_write() {
    let m = MockMmio::new();
    m.preset(0x02, 0xF0); // untouched high nibble
    let ok = run_pwr_seq(&m, &[PwrCmd::write(0x02, 0x03, 0x01), PwrCmd::end()]);
    assert!(ok);
    assert_eq!(m.read8(0x02), 0xF1, "only the masked bits change; the rest are kept");
}

#[test]
fn poll_waits_for_its_masked_value() {
    let m = MockMmio::new();
    m.preset(0x04, 0x80); // ready bit already set
    let ok = run_pwr_seq(&m, &[PwrCmd::poll(0x04, 0x80, 0x80), PwrCmd::end()]);
    assert!(ok, "a poll whose value is present succeeds");
}

#[test]
fn poll_reports_a_dead_card() {
    let m = MockMmio::new(); // 0x04 stays 0, never matches 0x80
    let ok = run_pwr_seq(&m, &[PwrCmd::poll(0x04, 0x80, 0x80), PwrCmd::end()]);
    assert!(!ok, "a poll that never observes its value fails instead of hanging");
}

#[test]
fn end_stops_the_sequence() {
    let m = MockMmio::new();
    let ok = run_pwr_seq(&m, &[PwrCmd::end(), PwrCmd::write(0x08, 0xFF, 0xAA)]);
    assert!(ok);
    assert_eq!(m.read8(0x08), 0x00, "nothing after END runs");
    assert!(m.writes.borrow().is_empty());
}

#[test]
fn a_table_runs_its_steps_in_order() {
    let m = MockMmio::new();
    let table = [
        PwrCmd::write(0x02, 0xFF, 0x11),
        PwrCmd::write(0x08, 0xFF, 0x22),
        PwrCmd::write(0x0004, 0xFF, 0x33),
        PwrCmd::end(),
    ];
    assert!(run_pwr_seq(&m, &table));
    assert_eq!(
        *m.writes.borrow(),
        vec![(0x02, 0x11), (0x08, 0x22), (0x0004, 0x33)],
        "writes land at the right offsets in the table's order"
    );
}
