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

//! Link power management. ASPM on this chip is two separate mechanisms: the host
//! sets the PCIe link control bit, and the chip carries its own software enable.
//! Clearing the chip's holds the link out of L1 regardless of what the host
//! asked for, which matters because an L1 link gates the internal clocks and a
//! gated block answers register reads with zeros while the always-on registers
//! keep answering normally. Held off for the whole of bring-up, the way rtw88
//! holds it off whenever it has real work to do. Values are rtw88 facts
//! (`pci.c`, `pci.h`), reimplemented not copied.

mod dbi;

use crate::regs::Mmio;

/// The chip's PCIe link configuration byte, in its own register file.
const RTK_PCIE_LINK_CFG: u16 = 0x0719;
/// The chip's own L1 (ASPM) software enable.
const BIT_L1_SW_EN: u8 = 1 << 3;
/// The chip's own CLKREQ software enable. CLKREQ lets the platform stop the
/// reference clock, which gates the same internal blocks L1 does.
const BIT_CLKREQ_SW_EN: u8 = 1 << 4;

/// Hold the link out of its low-power states for the duration of bring-up.
/// Returns false if the chip's PCIe register file did not answer, which is worth
/// reporting: it means the link state could not be established either way.
pub fn hold_link_awake<M: Mmio>(mmio: &M) -> bool {
    let Some(value) = dbi::read8(mmio, RTK_PCIE_LINK_CFG) else {
        return false;
    };
    dbi::write8(mmio, RTK_PCIE_LINK_CFG, value & !(BIT_L1_SW_EN | BIT_CLKREQ_SW_EN));
    true
}
