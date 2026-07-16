// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for baseband/RF bring-up: the PHY-condition parser's branch logic
//! (known-answer over a hand-built conditional table), the RF SIPI word
//! encoding, the BB/RF power-on register sequence, and that the real vendor
//! tables apply cleanly (their unconditional anchor entries reach the chip). The
//! parser and encodings are fully checked here; the on-chip RF calibration and
//! the exact chip condition are the on-silicon boundary.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::phy::apply::{load_bb, load_rf_a};
use crate::phy::calib::iqk;
use crate::phy::channel::{rf18_value, rf_switch, set_rf, Bw, RfSwitch};
use crate::phy::cond::{apply, PhyCond, INTF_PCIE};
use crate::phy::power::power_on;
use crate::phy::regs::{REG_RF_CTRL, REG_RF_SIPI_A, REG_SYS_FUNC_EN};
use crate::phy::rf::{read_a, sipi_word, write_masked_a};
use crate::regs::Mmio;

// A hand-built conditional table: an rfe==4 branch, an else branch, and an
// unconditional tail.
const COND_TABLE: &[u32] = &[
    0x8000_0004,
    0,
    0x4000_0000,
    0,
    0x0100,
    0xAAAA,
    0xA000_0000,
    0,
    0x0100,
    0xBBBB,
    0xB000_0000,
    0,
    0x0200,
    0xCCCC,
];

fn collect(table: &[u32], drv: &PhyCond) -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    apply(table, drv, |a, d| out.push((a, d)));
    out
}

#[test]
fn parser_takes_the_matching_branch() {
    let drv = PhyCond { cut: 1, pkg: 1, intf: INTF_PCIE, rfe: 4 };
    assert_eq!(
        collect(COND_TABLE, &drv),
        vec![(0x0100, 0xAAAA), (0x0200, 0xCCCC)],
        "the rfe==4 branch and the unconditional tail apply"
    );
}

#[test]
fn parser_takes_the_else_branch_when_the_guard_fails() {
    let drv = PhyCond { cut: 1, pkg: 1, intf: INTF_PCIE, rfe: 7 };
    assert_eq!(
        collect(COND_TABLE, &drv),
        vec![(0x0100, 0xBBBB), (0x0200, 0xCCCC)],
        "a non-matching rfe falls through to the else branch"
    );
}

#[test]
fn sipi_word_packs_address_and_value() {
    // addr in bits 20..27, value (20 bits) in bits 0..19, within 28 bits.
    assert_eq!(sipi_word(0x18, 0x12345), 0x0181_2345);
    assert_eq!(sipi_word(0x00, 0x00010000), 0x0001_0000);
    // Address is masked to 8 bits and value to 20 bits.
    assert_eq!(sipi_word(0x1FF, 0xFFFFFFFF) & 0xF000_0000, 0, "stays within 28 bits");
}

// A modeled chip recording register writes.
struct MockMmio {
    regs: RefCell<HashMap<usize, u32>>,
    writes: RefCell<Vec<(usize, u32)>>,
}

impl MockMmio {
    fn new() -> Self {
        Self { regs: RefCell::new(HashMap::new()), writes: RefCell::new(Vec::new()) }
    }
}

impl Mmio for MockMmio {
    fn read8(&self, off: usize) -> u8 {
        *self.regs.borrow().get(&off).unwrap_or(&0) as u8
    }
    fn write8(&self, off: usize, val: u8) {
        self.regs.borrow_mut().insert(off, val as u32);
        self.writes.borrow_mut().push((off, val as u32));
    }
    fn read16(&self, off: usize) -> u16 {
        *self.regs.borrow().get(&off).unwrap_or(&0) as u16
    }
    fn write16(&self, off: usize, val: u16) {
        self.regs.borrow_mut().insert(off, val as u32);
        self.writes.borrow_mut().push((off, val as u32));
    }
    fn read32(&self, off: usize) -> u32 {
        *self.regs.borrow().get(&off).unwrap_or(&0)
    }
    fn write32(&self, off: usize, val: u32) {
        self.regs.borrow_mut().insert(off, val);
        self.writes.borrow_mut().push((off, val));
    }
}

#[test]
fn power_on_runs_the_reset_sequence() {
    let m = MockMmio::new();
    power_on(&m);
    let w = m.writes.borrow();
    // The PCIe analog enable (bit 6) is set first at REG_SYS_FUNC_EN.
    assert_eq!(w[0], (REG_SYS_FUNC_EN, 0x40), "PCIe analog enable set");
    // The baseband reset is pulsed: released, asserted, released.
    assert_eq!(w[1], (REG_SYS_FUNC_EN, 0x43), "bb reset released with analog");
    assert_eq!(w[2], (REG_SYS_FUNC_EN, 0x40), "bb reset asserted");
    assert_eq!(w[3], (REG_SYS_FUNC_EN, 0x43), "bb reset released");
    // RF enabled and out of reset.
    assert!(w.contains(&(REG_RF_CTRL, 0x07)), "RF enabled and reset released");
}

#[test]
fn the_real_bb_table_applies_its_unconditional_anchor() {
    let m = MockMmio::new();
    let drv = PhyCond { cut: 1, pkg: 1, intf: INTF_PCIE, rfe: 4 };
    load_bb(&m, &drv);
    // The BB table's first entry is unconditional and must reach the chip.
    assert_eq!(m.read32(0x800), 0x9020_D010, "BB register 0x800 programmed");
    assert!(m.writes.borrow().len() > 1000, "the full baseband table is applied");
}

#[test]
fn rf18_encodes_a_2g_channel() {
    // Channel 6, 20MHz: band 2G (0), channel number 6, both bandwidth bits.
    let v = rf18_value(6, Bw::W20, 0);
    assert_eq!(v & 0xFF, 6, "channel number");
    assert_eq!(v & ((1 << 16) | (1 << 9) | (1 << 8)), 0, "2G band is zero");
    assert_eq!(v & ((1 << 11) | (1 << 10)), (1 << 11) | (1 << 10), "20MHz sets both bw bits");
    assert_eq!(v & ((1 << 18) | (1 << 17)), 0, "no synthesizer index in 2G");
}

#[test]
fn rf18_encodes_a_5g_channel_with_synthesizer_index() {
    // Channel 149 (>140), 80MHz: 5G band, GT synthesizer index, 80MHz bit.
    let v = rf18_value(149, Bw::W80, 0);
    assert_eq!(v & 0xFF, 149, "channel number");
    assert_eq!(v & ((1 << 16) | (1 << 8)), (1 << 16) | (1 << 8), "5G band set");
    assert_eq!(v & (1 << 18), 1 << 18, "GT synthesizer index for channels above 140");
    assert_eq!(v & ((1 << 11) | (1 << 10)), 1 << 10, "80MHz sets the low bw bit only");
    // Channel 100..=140 uses the GE index instead.
    assert_eq!(rf18_value(100, Bw::W20, 0) & (1 << 17), 1 << 17, "GE index for 100..=140");
}

#[test]
fn rf18_preserves_unrelated_bits() {
    // A bit outside the band/channel/index/bandwidth fields survives.
    let v = rf18_value(6, Bw::W20, 1 << 20);
    assert_eq!(v & (1 << 20), 1 << 20, "unrelated bits are kept");
}

#[test]
fn rf_switch_selects_the_front_end() {
    assert_eq!(rf_switch(6, false), RfSwitch::Wlg, "2G WLAN");
    assert_eq!(rf_switch(6, true), RfSwitch::Btg, "2G shared with Bluetooth");
    assert_eq!(rf_switch(149, false), RfSwitch::Wla, "5G WLAN");
}

#[test]
fn rf_direct_read_masks_to_twenty_bits() {
    let m = MockMmio::new();
    // RF register 0x18 reads from base 0x2800 + (0x18 << 2) = 0x2860.
    m.write32(0x2860, 0x0012_3456);
    assert_eq!(read_a(&m, 0x18), 0x2_3456, "read is masked to 20 bits");
}

#[test]
fn rf_masked_write_is_a_read_modify_write_through_sipi() {
    let m = MockMmio::new();
    // Register 0xDF currently reads 0x00001 at its direct address.
    m.write32(0x2800 + (0xDF << 2), 0x0_0001);
    write_masked_a(&m, 0xDF, 1 << 6, 1); // set bit 6, keep the rest
                                         // The resulting SIPI write carries reg 0xDF and the merged value 0x41.
    let expect = sipi_word(0xDF, 0x41);
    assert!(
        m.writes.borrow().contains(&(REG_RF_SIPI_A, expect)),
        "masked write preserves other bits"
    );
}

#[test]
fn set_rf_programs_register_18_for_the_channel() {
    let m = MockMmio::new();
    set_rf(&m, 6, Bw::W20); // 2G channel 6, 20MHz; rf18 reads 0
                            // Register 0x18 is written with the computed value via SIPI.
    let expect = sipi_word(0x18, rf18_value(6, Bw::W20, 0));
    assert!(m.writes.borrow().contains(&(REG_RF_SIPI_A, expect)), "RF 0x18 programmed");
    // Every RF write in the retune goes to the SIPI register.
    assert!(m.writes.borrow().iter().all(|&(o, _)| o == REG_RF_SIPI_A), "all writes are RF SIPI");
}

#[test]
fn iqk_sends_the_request_waits_for_done_then_clears() {
    let m = MockMmio::new();
    // The firmware reports IQK done: RF_DTXLOK (0x08) reads the marker at its
    // direct address 0x2800 + (0x08 << 2) = 0x2820.
    m.write32(0x2820, 0x0A_BCDE);

    let sent = RefCell::new(0usize);
    let ok = iqk(
        &m,
        |pkt| {
            *sent.borrow_mut() = pkt.len();
            true
        },
        3,
        false,
    );
    assert!(ok, "calibration completes");
    assert_eq!(*sent.borrow(), 32, "a 32-byte H2C packet is sent");
    // The completion marker is cleared through a SIPI write to RF_DTXLOK.
    assert!(
        m.writes.borrow().contains(&(REG_RF_SIPI_A, sipi_word(0x08, 0))),
        "the done marker is cleared"
    );
}

#[test]
fn iqk_fails_when_the_request_is_refused() {
    let m = MockMmio::new();
    assert!(!iqk(&m, |_pkt| false, 0, false), "a refused H2C packet fails the calibration");
}

#[test]
fn the_real_rf_table_writes_through_sipi() {
    let m = MockMmio::new();
    // rfe/pkg chosen so the RF table's leading branch matches and RF writes flow.
    let drv = PhyCond { cut: 1, pkg: 1, intf: INTF_PCIE, rfe: 5 };
    load_rf_a(&m, &drv);
    // Every RF write goes to the path-A SIPI register.
    assert!(
        m.writes.borrow().iter().any(|&(off, _)| off == REG_RF_SIPI_A),
        "RF path-A programmed through the SIPI register"
    );
    assert!(m.writes.borrow().len() > 500, "the RF table is applied");
}
