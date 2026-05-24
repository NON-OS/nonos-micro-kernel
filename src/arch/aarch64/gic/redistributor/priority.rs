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

use core::ptr::write_volatile;

use super::constants::GICR_IPRIORITYR;
use super::device::GicRedistributor;

impl GicRedistributor {
    pub fn set_priority(&self, irq: u32, priority: u8) {
        if irq < 32 {
            let addr = (self.base + GICR_IPRIORITYR + irq as u64) as *mut u8;
            unsafe {
                write_volatile(addr, priority);
            }
        }
    }
}
