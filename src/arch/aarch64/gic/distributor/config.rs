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

use super::constants::GICD_ICFGR;
use super::device::GicDistributor;

impl GicDistributor {
    pub fn set_config(&self, irq: u32, edge: bool) {
        let reg = GICD_ICFGR + ((irq / 16) * 4) as u64;
        let shift = (irq % 16) * 2;
        let mut val = self.read_reg(reg);
        if edge {
            val |= 2 << shift;
        } else {
            val &= !(2 << shift);
        }
        self.write_reg(reg, val);
    }
}
