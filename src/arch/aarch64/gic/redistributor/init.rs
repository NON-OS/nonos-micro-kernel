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
    GICR_ICENABLER0, GICR_IGROUPR0, GICR_IGRPMODR0, GICR_IPRIORITYR, GICR_ISENABLER0,
};
use super::device::GicRedistributor;

impl GicRedistributor {
    pub fn init(&self) {
        self.wake();
        self.write_reg(GICR_IGROUPR0, 0xFFFF_FFFF);
        self.write_reg(GICR_IGRPMODR0, 0);
        for irq in (0..32u32).step_by(4) {
            self.write_reg(GICR_IPRIORITYR + irq as u64, 0xA0A0_A0A0);
        }
        self.write_reg(GICR_ICENABLER0, 0xFFFF_0000);
        self.write_reg(GICR_ISENABLER0, 0x0000_FFFF);
    }
}
