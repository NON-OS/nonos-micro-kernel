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

use super::constants::{GICD_ICPENDR, GICD_ISPENDR};
use super::device::GicDistributor;

impl GicDistributor {
    pub fn set_pending(&self, irq: u32) {
        let reg = GICD_ISPENDR + ((irq / 32) * 4) as u64;
        self.write_reg(reg, 1u32 << (irq % 32));
    }

    pub fn clear_pending(&self, irq: u32) {
        let reg = GICD_ICPENDR + ((irq / 32) * 4) as u64;
        self.write_reg(reg, 1u32 << (irq % 32));
    }
}
