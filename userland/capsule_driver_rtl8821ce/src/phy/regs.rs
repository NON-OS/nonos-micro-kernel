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

//! Registers and bits for baseband/RF power-on and the RF serial (SIPI) write.
//! Values from rtw88 `reg.h`, `rtw8821c.h` and `rtw8821c_phy_set_param` /
//! `rtw_phy_write_rf_reg_sipi` in `phy.c`.

/// `REG_SYS_FUNC_EN` (byte 0): baseband and PCIe analog function enables.
pub const REG_SYS_FUNC_EN: usize = 0x0002;
/// `BIT_FEN_BB_RSTB` (`BIT(0)`): baseband reset.
pub const FEN_BB_RSTB: u8 = 1 << 0;
/// `BIT_FEN_BB_GLB_RST` (`BIT(1)`): baseband global reset.
pub const FEN_BB_GLB_RST: u8 = 1 << 1;
/// `BIT_FEN_PCIEA` (`BIT(6)`): the PCIe analog domain enable.
pub const FEN_PCIEA: u8 = 1 << 6;

/// `REG_RF_CTRL`: the RF enable/reset control byte.
pub const REG_RF_CTRL: usize = 0x001F;
/// `REG_WLRF1`; byte 3 mirrors the RF enable/reset control.
pub const REG_WLRF1: usize = 0x00EC;
/// `BIT_RF_EN | BIT_RF_RSTB | BIT_RF_SDM_RSTB`: enable RF and release its resets.
pub const RF_EN_RSTB: u8 = (1 << 0) | (1 << 1) | (1 << 2);

/// The path-A RF SIPI write register (`chip->rf_sipi_addr[RF_PATH_A]`).
pub const REG_RF_SIPI_A: usize = 0x0C90;
/// `RFREG_MASK`: the 20-bit RF register value field.
pub const RFREG_MASK: u32 = 0x000F_FFFF;
