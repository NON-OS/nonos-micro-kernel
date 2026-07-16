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

//! Load the baseband, AGC, RF and PHY-MAC tables into the chip. Each table is
//! walked by the condition parser; a matching entry is a register write, except
//! for a small set of reserved addresses the vendor uses as inline settle
//! delays. The MAC table writes bytes, the BB and AGC tables write 32-bit
//! registers, and the RF table writes the path-A register through the serial
//! (SIPI) interface. The write widths, the delay addresses and the SIPI word
//! layout follow rtw88 `rtw_phy_cfg_mac/agc/bb/rf` and
//! `rtw_phy_write_rf_reg_sipi`; the SIPI encoding is checked in `rtl8821ce_proofs`.

use super::cond::{apply, PhyCond};
use super::rf;
use super::tables::{RTL8821C_AGC, RTL8821C_BB, RTL8821C_MAC, RTL8821C_RF_A};
use crate::regs::Mmio;

// A BB-table address in 0xF9..=0xFE is a settle delay, not a register.
fn is_bb_delay(addr: u32) -> bool {
    (0xF9..=0xFE).contains(&addr)
}
// An RF-table address of 0xFE or 0xFFE is a settle delay.
fn is_rf_delay(addr: u32) -> bool {
    addr == 0xFE || addr == 0xFFE
}

// A short busy-wait; the exact duration only affects settle time, not the
// register program the tables produce.
fn settle() {
    for _ in 0..4096 {
        core::hint::spin_loop();
    }
}

/// Load the PHY-side MAC table (byte writes).
pub fn load_mac<M: Mmio>(mmio: &M, drv: &PhyCond) {
    apply(RTL8821C_MAC, drv, |addr, data| mmio.write8(addr as usize, data as u8));
}

/// Load the baseband table (32-bit writes, with inline settle delays).
pub fn load_bb<M: Mmio>(mmio: &M, drv: &PhyCond) {
    apply(RTL8821C_BB, drv, |addr, data| {
        if is_bb_delay(addr) {
            settle();
        } else {
            mmio.write32(addr as usize, data);
        }
    });
}

/// Load the AGC table (32-bit writes).
pub fn load_agc<M: Mmio>(mmio: &M, drv: &PhyCond) {
    apply(RTL8821C_AGC, drv, |addr, data| mmio.write32(addr as usize, data));
}

/// Load the RF path-A table through the SIPI interface (with settle delays).
pub fn load_rf_a<M: Mmio>(mmio: &M, drv: &PhyCond) {
    apply(RTL8821C_RF_A, drv, |addr, data| {
        if is_rf_delay(addr) {
            settle();
        } else {
            rf::write_a(mmio, addr as u8, data);
            settle();
        }
    });
}

/// Load every PHY table in the rtw88 order: MAC, baseband, AGC, then RF.
pub fn load_all<M: Mmio>(mmio: &M, drv: &PhyCond) {
    load_mac(mmio, drv);
    load_bb(mmio, drv);
    load_agc(mmio, drv);
    load_rf_a(mmio, drv);
}
