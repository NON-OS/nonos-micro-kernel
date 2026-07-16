// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the receiver-enable path (`phy::rxpath`): the RX-path reset toggle
//! around the table load, the RFE antenna switch that routes the antenna into the
//! WiFi low-noise amplifier, the 2.4GHz receive-path select, and the receive
//! digital filter. A modeled device stores register values so read-modify-writes
//! behave, and records the last value written to each offset.

use core::cell::RefCell;

use crate::phy::channel::Bw;
use crate::phy::rxpath::{pre_tables, post_tables, rfe_is_btg, set_channel};
use crate::regs::Mmio;

struct Dev {
    store: RefCell<[u32; 0x3000]>,
}

impl Dev {
    fn new() -> Self {
        Self { store: RefCell::new([0u32; 0x3000]) }
    }
    fn get(&self, off: usize) -> u32 {
        self.store.borrow()[off]
    }
    fn seed(&self, off: usize, v: u32) {
        self.store.borrow_mut()[off] = v;
    }
}

impl Mmio for Dev {
    fn read8(&self, off: usize) -> u8 {
        self.store.borrow()[off] as u8
    }
    fn write8(&self, off: usize, v: u8) {
        self.store.borrow_mut()[off] = v as u32;
    }
    fn read16(&self, off: usize) -> u16 {
        self.store.borrow()[off] as u16
    }
    fn write16(&self, off: usize, v: u16) {
        self.store.borrow_mut()[off] = v as u32;
    }
    fn read32(&self, off: usize) -> u32 {
        self.store.borrow()[off]
    }
    fn write32(&self, off: usize, v: u32) {
        self.store.borrow_mut()[off] = v;
    }
}

const REG_RXPSEL: usize = 0x0808;
const REG_CCK_CHECK: usize = 0x0454;
const REG_RFECTL: usize = 0x0CB8;
const REG_ACBB0: usize = 0x0948;
const RX_PSEL_RST: u32 = (1 << 28) | (1 << 29);

#[test]
fn pre_tables_clears_and_post_tables_sets_the_rx_path_reset() {
    let dev = Dev::new();
    dev.seed(REG_RXPSEL, 0xFFFF_FFFF);
    pre_tables(&dev);
    assert_eq!(dev.get(REG_RXPSEL) & RX_PSEL_RST, 0, "rx-path reset cleared before tables");
    post_tables(&dev);
    assert_eq!(
        dev.get(REG_RXPSEL) & RX_PSEL_RST,
        RX_PSEL_RST,
        "rx-path reset released after tables"
    );
}

#[test]
fn set_channel_routes_the_antenna_and_selects_the_receive_path() {
    let dev = Dev::new();
    // A standard single-antenna board: WiFi-only (WLG) routing.
    set_channel(&dev, 1, Bw::W20, false);

    // The RFE control routes to the WiFi path: WL/WLG switch bits set, the
    // Bluetooth, control and WLA bits clear.
    let rfe = dev.get(REG_RFECTL);
    assert_eq!(rfe & ((1 << 20) | (1 << 22) | (1 << 21)), (1 << 20) | (1 << 22) | (1 << 21));
    assert_eq!(rfe & ((1 << 16) | (1 << 18) | (1 << 23)), 0, "bluetooth/wla path off");

    // The 2.4GHz receive path is selected and CCK reception is enabled.
    assert_eq!(dev.get(REG_RXPSEL) & (1 << 28), 1 << 28, "cck receive path selected");
    assert_eq!(dev.get(REG_CCK_CHECK) & (1 << 7), 0, "cck reception enabled");

    // The receive digital filter is programmed for 20MHz.
    assert_eq!((dev.get(REG_ACBB0) >> 28) & 0x3, 0x2, "rx digital filter set");
}

#[test]
fn set_channel_programs_a_nonzero_transmit_power() {
    let dev = Dev::new();
    set_channel(&dev, 1, Bw::W20, false);
    // Every rate register carries the fixed power index in all four bytes, so
    // frames leave the antenna with real power instead of the zero default.
    for group in 0..5usize {
        let word = dev.get(0x1D00 + group * 4);
        assert_ne!(word, 0, "rate group {group} has transmit power");
        assert_eq!(word & 0xFF, 0x28, "power index in the low byte");
        assert_eq!(word, 0x2828_2828, "the index is packed across all four rates");
    }
}

#[test]
fn a_bluetooth_shared_board_routes_the_bluetooth_path() {
    let dev = Dev::new();
    set_channel(&dev, 6, Bw::W20, true);
    let rfe = dev.get(REG_RFECTL);
    assert_eq!(rfe & (1 << 16), 1 << 16, "bluetooth-shared antenna path selected");
    assert_eq!(rfe & ((1 << 20) | (1 << 22)), 0, "wifi-only bits cleared");
}

#[test]
fn rfe_option_selects_the_antenna_path() {
    // The shared-antenna options route through Bluetooth; everything else is
    // WiFi-only. A standard single-antenna module is option 0.
    for opt in [2u8, 4, 7, 0x0A, 0x0C, 0x0F] {
        assert!(rfe_is_btg(opt), "option {opt:#x} is bluetooth-shared");
    }
    for opt in [0u8, 1, 3, 5, 6, 8] {
        assert!(!rfe_is_btg(opt), "option {opt:#x} is wifi-only");
    }
}
