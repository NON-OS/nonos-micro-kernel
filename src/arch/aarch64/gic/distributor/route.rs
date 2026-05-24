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

use super::constants::{GICD_IROUTER, GICD_TYPER};
use super::device::GicDistributor;

impl GicDistributor {
    pub fn set_route(&self, irq: u32, affinity: u64) {
        self.write_reg64(GICD_IROUTER + (irq as u64) * 8, affinity);
    }

    pub fn num_irqs(&self) -> u32 {
        ((self.read_reg(GICD_TYPER) & 0x1F) + 1) * 32
    }
}
