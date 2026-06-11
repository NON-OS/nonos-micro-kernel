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

use super::constants::{
    CTLR_ARE_NS, CTLR_ARE_S, CTLR_ENABLE_G0, CTLR_ENABLE_G1NS, GICD_CTLR, GICD_ICENABLER,
    GICD_ICPENDR, GICD_IGROUPR, GICD_IPRIORITYR,
};
use super::device::GicDistributor;

impl GicDistributor {
    pub fn init(&self) {
        self.write_reg(GICD_CTLR, 0);
        let num_irqs = self.num_irqs();
        for irq in (32..num_irqs).step_by(32) {
            self.write_reg(GICD_ICENABLER + (irq / 32) as u64 * 4, 0xFFFF_FFFF);
            self.write_reg(GICD_ICPENDR + (irq / 32) as u64 * 4, 0xFFFF_FFFF);
            self.write_reg(GICD_IGROUPR + (irq / 32) as u64 * 4, 0xFFFF_FFFF);
        }
        for irq in (32..num_irqs).step_by(4) {
            self.write_reg(GICD_IPRIORITYR + irq as u64, 0xA0A0_A0A0);
        }
        self.write_reg(GICD_CTLR, CTLR_ENABLE_G0 | CTLR_ENABLE_G1NS | CTLR_ARE_S | CTLR_ARE_NS);
    }
}
