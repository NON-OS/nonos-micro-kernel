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

//! Power on the baseband and RF domains before the register tables are loaded:
//! enable the PCIe analog domain, pulse the baseband reset (release, assert,
//! release), then enable the RF and release its resets on both the control and
//! WLRF registers. This is the prologue of rtw88 `rtw8821c_phy_set_param` in
//! `rtw8821c.c`; the register sequence is checked against a modeled device in
//! `rtl8821ce_proofs`.

use super::regs::{
    FEN_BB_GLB_RST, FEN_BB_RSTB, FEN_PCIEA, REG_RF_CTRL, REG_SYS_FUNC_EN, REG_WLRF1, RF_EN_RSTB,
};
use crate::regs::Mmio;

const BB_RST: u8 = FEN_BB_RSTB | FEN_BB_GLB_RST;

fn settle() {
    for _ in 0..1024 {
        core::hint::spin_loop();
    }
}

/// Bring the baseband and RF domains out of reset.
pub fn power_on<M: Mmio>(mmio: &M) {
    // Enable the PCIe analog domain.
    let val = mmio.read8(REG_SYS_FUNC_EN) | FEN_PCIEA;
    mmio.write8(REG_SYS_FUNC_EN, val);
    // Pulse the baseband reset: release, assert, release.
    mmio.write8(REG_SYS_FUNC_EN, val | BB_RST);
    mmio.write8(REG_SYS_FUNC_EN, val & !BB_RST);
    mmio.write8(REG_SYS_FUNC_EN, val | BB_RST);
    // Enable RF and release its resets on both control registers.
    mmio.write8(REG_RF_CTRL, RF_EN_RSTB);
    settle();
    mmio.write8(REG_WLRF1 + 3, RF_EN_RSTB);
    settle();
}
