// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Turn the receiver on. Loading the register tables and tuning the synthesizer
//! is not enough: the baseband keeps its receive path in reset across the table
//! load, the antenna is not routed into the WiFi low-noise amplifier, and the
//! 2.4GHz receive path and its digital filter are not selected. Without these the
//! radio powers up and hears nothing. This is rtw88 `rtw8821c_phy_set_param`'s
//! RX-path reset toggle, `rtw8821c_switch_rf_set` (the RFE antenna switch),
//! `rtw8821c_set_channel_bb` and `rtw8821c_set_channel_rxdfir` for a 2.4GHz
//! channel, reimplemented over the `Mmio` trait. Checked against a modeled device
//! in `rtl8821ce_proofs`.

use super::channel::{set_rf, Bw};
use crate::regs::Mmio;

// Baseband / analog-front-end register offsets (rtw88 reg.h).
const REG_RXPSEL: usize = 0x0808;
const REG_CCK_CHECK: usize = 0x0454;
const REG_ENTXCCK: usize = 0x0A80;
const REG_ENRXCCA: usize = 0x0A84;
const REG_RXCCAMSK: usize = 0x0814;
const REG_TXSCALE_A: usize = 0x0C1C;
const REG_CLKTRK: usize = 0x0860;
const REG_ADCCLK: usize = 0x08AC;
const REG_ADC160: usize = 0x08C4;
const REG_CCK0_FAREPORT: usize = 0x0A2C;
const REG_RFECTL: usize = 0x0CB8;
const REG_DMEM_CTRL: usize = 0x1080;
const REG_SYS_CTRL: usize = 0x0000;
const REG_ACBB0: usize = 0x0948;
const REG_ACBBRXFIR: usize = 0x094C;
const REG_TXDFIR: usize = 0x0C20;
const REG_CHFIR: usize = 0x08F0;

// The receive-path reset bits, held around the table load.
const RX_PSEL_RST: u32 = (1 << 28) | (1 << 29);

/// Clear the receive-path reset before the baseband tables are loaded.
pub fn pre_tables<M: Mmio>(mmio: &M) {
    clr(mmio, REG_RXPSEL, RX_PSEL_RST);
}

/// Release the receive-path reset after the tables are loaded, and clear the CCK
/// report bits, so the receive front end leaves reset.
pub fn post_tables<M: Mmio>(mmio: &M) {
    set(mmio, REG_RXPSEL, RX_PSEL_RST);
    clr(mmio, REG_CCK0_FAREPORT, (1 << 18) | (1 << 22));
}

/// Select and enable the 2.4GHz receive path, route the antenna into the WiFi
/// low-noise amplifier, tune to `channel`, and set the receive digital filter for
/// `bw`. `rfe_btg` chooses the Bluetooth-shared antenna path over the WiFi-only
/// one, decided from the board's RF front-end option. This is the whole
/// tables-loaded-to-receiving step for one 2.4GHz channel.
pub fn set_channel<M: Mmio>(mmio: &M, channel: u8, bw: Bw, rfe_btg: bool) {
    set_channel_bb(mmio);
    switch_rf(mmio, rfe_btg);
    set_rf(mmio, channel, bw);
    set_rxdfir(mmio, bw);
    // Give transmitted frames real power; without this they leave the antenna at
    // essentially zero and no access point can hear them.
    super::txpower::set_tx_power(mmio);
}

// Select the 2.4GHz receive path and enable CCK reception. Band-level, so the
// scan retunes only the synthesizer per channel and leaves this in place.
fn set_channel_bb<M: Mmio>(mmio: &M) {
    mask(mmio, REG_RXPSEL, 1 << 28, 1); // select the CCK receive path (2.4GHz)
    mask(mmio, REG_CCK_CHECK, 1 << 7, 0); // CCK reception enabled for 2.4GHz
    mask(mmio, REG_ENTXCCK, 1 << 18, 0);
    mask(mmio, REG_RXCCAMSK, 0x0000_FC00, 15);
    mask(mmio, REG_TXSCALE_A, 0xF00, 0);
    mask(mmio, REG_CLKTRK, 0x1FFE_0000, 0x96A);
    // ADC clocking for a 2.4GHz 20MHz channel.
    let adcclk = (mmio.read32(REG_ADCCLK) & 0xFFCF_FC00) | 0x1001_0000;
    mmio.write32(REG_ADCCLK, adcclk);
    mask(mmio, REG_ADC160, 1 << 30, 1); // 160MHz ADC path
}

// Route the antenna into the receive low-noise amplifier. WiFi-only (WLG) unless
// the board shares the antenna with Bluetooth (BTG).
fn switch_rf<M: Mmio>(mmio: &M, rfe_btg: bool) {
    set(mmio, REG_DMEM_CTRL, 1 << 16);
    set(mmio, REG_SYS_CTRL, 1 << 26);
    let mut reg = mmio.read32(REG_RFECTL);
    if rfe_btg {
        reg |= 1 << 16;
        reg &= !((1 << 18) | (1 << 20) | (1 << 22) | (1 << 21) | (1 << 23));
        mask(mmio, REG_ENRXCCA, 0x00FF_0000, 0x0E);
        mask(mmio, REG_ENTXCCK, 0x0000_FFFF, 0xFC84);
    } else {
        reg |= (1 << 20) | (1 << 22) | (1 << 21);
        reg &= !((1 << 16) | (1 << 18) | (1 << 23));
        mask(mmio, REG_ENRXCCA, 0x00FF_0000, 0x12);
        mask(mmio, REG_ENTXCCK, 0x0000_FFFF, 0x7532);
    }
    mmio.write32(REG_RFECTL, reg);
}

// Set the receive digital filter for a 20MHz channel.
fn set_rxdfir<M: Mmio>(mmio: &M, bw: Bw) {
    let _ = bw; // only 20MHz is programmed today; the wide filters follow later.
    mask(mmio, REG_ACBB0, (1 << 29) | (1 << 28), 0x2);
    mask(mmio, REG_ACBBRXFIR, (1 << 29) | (1 << 28), 0x2);
    mask(mmio, REG_TXDFIR, 1 << 31, 0x1);
    mask(mmio, REG_CHFIR, 1 << 31, 0x0);
}

// Read-modify-write the bits under `msk` to `val`, aligned to the mask's low bit.
fn mask<M: Mmio>(mmio: &M, off: usize, msk: u32, val: u32) {
    let shift = msk.trailing_zeros();
    let cur = mmio.read32(off);
    mmio.write32(off, (cur & !msk) | ((val << shift) & msk));
}

fn set<M: Mmio>(mmio: &M, off: usize, bits: u32) {
    let cur = mmio.read32(off);
    mmio.write32(off, cur | bits);
}

fn clr<M: Mmio>(mmio: &M, off: usize, bits: u32) {
    let cur = mmio.read32(off);
    mmio.write32(off, cur & !bits);
}

/// Whether the board routes the antenna through the Bluetooth-shared path. rtw88
/// reads this from the RF front-end option in the efuse; these are the options
/// that select the shared path, everything else is WiFi-only.
pub fn rfe_is_btg(rfe_option: u8) -> bool {
    matches!(rfe_option & 0x1F, 2 | 4 | 7 | 0x0A | 0x0C | 0x0F)
}

/// Whether this is a front-end the 8821c actually ships with. rtw88 keeps a table
/// per option and refuses one it has no entry for. 0x1F in particular is what an
/// erased or unreadable efuse yields, and driving the RF from it programs the
/// chip for hardware that is not on this board.
pub fn rfe_is_supported(rfe_option: u8) -> bool {
    matches!(rfe_option & 0x1F, 0..=7 | 0x0A | 0x0C | 0x0F)
}
