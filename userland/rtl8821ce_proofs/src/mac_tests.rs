// NONOS Operating System (AGPL-3.0-or-later)
//! Known-behaviour proofs for the MAC init register-operation engine. A modeled
//! register file backs reads so the set/clear read-modify-writes work and records
//! writes; the proofs assert that a plain write lands at each width, that a set
//! ORs its bits while preserving the rest, that a clear masks its bits off, and
//! that a table runs in order.

use core::cell::RefCell;

use crate::mac::op::MacOp;
use crate::mac::{run_mac_table, MAC_INIT};
use crate::regs::Mmio;

const SIZE: usize = 0x1500;

struct MockMmio {
    regs: RefCell<[u8; SIZE]>,
    writes: RefCell<Vec<(usize, u32)>>,
}

impl MockMmio {
    fn new() -> Self {
        Self { regs: RefCell::new([0u8; SIZE]), writes: RefCell::new(Vec::new()) }
    }
}

impl Mmio for MockMmio {
    fn read8(&self, off: usize) -> u8 {
        self.regs.borrow()[off]
    }
    fn write8(&self, off: usize, val: u8) {
        self.regs.borrow_mut()[off] = val;
        self.writes.borrow_mut().push((off, val as u32));
    }
    fn read16(&self, off: usize) -> u16 {
        let r = self.regs.borrow();
        u16::from_le_bytes([r[off], r[off + 1]])
    }
    fn write16(&self, off: usize, val: u16) {
        let b = val.to_le_bytes();
        self.regs.borrow_mut()[off] = b[0];
        self.regs.borrow_mut()[off + 1] = b[1];
        self.writes.borrow_mut().push((off, val as u32));
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
        self.writes.borrow_mut().push((off, val));
    }
}

#[test]
fn writes_land_at_each_width() {
    let m = MockMmio::new();
    run_mac_table(
        &m,
        &[
            MacOp::write8(0x10, 0xAB),
            MacOp::write16(0x20, 0x1234),
            MacOp::write32(0x30, 0xDEAD_BEEF),
        ],
    );
    assert_eq!(m.read8(0x10), 0xAB);
    assert_eq!(m.read16(0x20), 0x1234);
    assert_eq!(m.read32(0x30), 0xDEAD_BEEF);
}

#[test]
fn set_ors_bits_and_keeps_the_rest() {
    let m = MockMmio::new();
    m.write8(0x10, 0x0F);
    m.write32(0x30, 0x0000_00FF);
    run_mac_table(&m, &[MacOp::set8(0x10, 0x40), MacOp::set32(0x30, 0x0001_0000)]);
    assert_eq!(m.read8(0x10), 0x4F, "the set bit is added, the rest kept");
    assert_eq!(m.read32(0x30), 0x0001_00FF, "the set bit is added, the rest kept");
}

#[test]
fn clear_masks_bits_off_and_keeps_the_rest() {
    let m = MockMmio::new();
    m.write8(0x10, 0xFF);
    m.write32(0x30, 0xFFFF_FFFF);
    run_mac_table(&m, &[MacOp::clear8(0x10, 0x0F), MacOp::clear32(0x30, 0x0000_00FF)]);
    assert_eq!(m.read8(0x10), 0xF0, "only the masked bits are cleared");
    assert_eq!(m.read32(0x30), 0xFFFF_FF00, "only the masked bits are cleared");
}

#[test]
fn a_masked_field_update_is_a_clear_then_set() {
    // rtw88 does `value8 &= 0xF0; value8 |= 0xF` on REG_TRXFF_BNDY+1; that is a
    // clear of the low nibble then a set of it, preserving the high nibble.
    let m = MockMmio::new();
    m.write8(0x08, 0xA5);
    run_mac_table(&m, &[MacOp::clear8(0x08, 0x0F), MacOp::set8(0x08, 0x0F)]);
    assert_eq!(m.read8(0x08), 0xAF, "high nibble kept, low nibble set to 0xF");
}

#[test]
fn the_real_mac_init_table_writes_the_rtw88_values() {
    // Run the actual RTL8821CE MAC init program against a zeroed device and check
    // the register values against rtw88 rtw8821c_mac_init / rtw_drv_info_cfg,
    // including the composite 16/32-bit words computed from the WLAN_* fields.
    let m = MockMmio::new();
    run_mac_table(&m, MAC_INIT);

    assert_eq!(m.read8(0x0455), 0x70, "AMPDU max time");
    assert_eq!(m.read8(0x04E5), 0xE4, "pre-txcnt low byte");
    assert_eq!(m.read8(0x04E6), 0x09, "pre-txcnt high byte (EN_PRECNT set)");
    assert_eq!(m.read32(0x04C8), 0x2020_08FF, "protection mode control");
    assert_eq!(m.read16(0x04CE), 0x0801, "BAR mode retry limits");
    assert_eq!(m.read8(0x0512), 0x19, "PIFS");
    assert_eq!(m.read8(0x051B), 0x09, "slot time");
    assert_eq!(m.read32(0x0514), 0x1010_0E0A, "SIFS composite");
    assert_eq!(m.read16(0x0502), 0x0186, "VO TXOP limit");
    assert_eq!(m.read16(0x0506), 0x03BC, "VI TXOP limit");
    assert_eq!(m.read32(0x0544), 0x001B_0005, "NAV composite");
    assert_eq!(m.read16(0x055E), 0x3030, "RX TSF composite");
    assert_eq!(m.read32(0x0540), 0x0000_6404, "TBTT composite");
    assert_eq!(m.read32(0x06A0), 0x0FFF_FFFF, "RX filter map 0");
    assert_eq!(m.read16(0x06A4), 0xFFFF, "RX filter map 2");
    // RCR is written then APP_PHYSTS (bit 28) is set: 0xE400220E | 0x10000000.
    assert_eq!(m.read32(0x0608), 0xF400_220E, "RCR with APP_PHYSTS");
    assert_eq!(m.read8(0x060C), 0x18, "RX packet size limit (12288>>9)");
    assert_eq!(m.read8(0x0639), 0x40, "ACK timeout CCK");
    assert_eq!(m.read8(0x066C), 0x02, "WMAC TRXPTCL bit 1 set");
    assert_eq!(m.read8(0x0718), 0x40, "SND PTCL disable VHTSIGB CRC set");
    assert_eq!(m.read32(0x07D8), 0xB081_0041, "WMAC option function 2");
    assert_eq!(m.read8(0x07D4), 0x98, "WMAC option normal function 1");
    assert_eq!(m.read8(0x060F), 0x04, "RX drvinfo size = PHY status size");
    assert_eq!(m.read8(0x0115), 0x0F, "TRXFF boundary low nibble fixed to 0xF");
}

#[test]
fn set_and_clear_ops_in_the_table_preserve_untouched_bits() {
    // Seed bits the init program only sets or clears, and confirm the rest are
    // kept: TIMER0_SRC_SEL keeps its high bits when TSFT_SEL is cleared, and
    // BCN_CTRL keeps its low bits when the beacon function is set.
    let m = MockMmio::new();
    m.write8(0x05B4, 0xFF); // REG_TIMER0_SRC_SEL
    m.write8(0x0550, 0x01); // REG_BCN_CTRL
    run_mac_table(&m, MAC_INIT);
    assert_eq!(m.read8(0x05B4), 0x8F, "only TSFT_SEL_TIMER0 (0x70) is cleared");
    assert_eq!(m.read8(0x0550), 0x09, "EN_BCN_FUNCTION (0x08) set, low bit kept");
}

#[test]
fn a_table_runs_in_order() {
    let m = MockMmio::new();
    run_mac_table(
        &m,
        &[MacOp::write8(0x02, 0x11), MacOp::write8(0x04, 0x22), MacOp::write8(0x06, 0x33)],
    );
    assert_eq!(
        *m.writes.borrow(),
        vec![(0x02, 0x11), (0x04, 0x22), (0x06, 0x33)],
        "operations land in table order"
    );
}
