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

use core::ptr::{read_volatile, write_volatile};

use super::{address, Plic, PlicResult};

impl Plic {
    pub fn set_priority(&self, irq: u32, priority: u8) -> PlicResult<()> {
        address::valid_irq(irq)?;
        unsafe {
            write_volatile(address::priority(self.base(), irq) as *mut u32, priority as u32);
        }
        Ok(())
    }

    pub fn get_priority(&self, irq: u32) -> PlicResult<u8> {
        address::valid_irq(irq)?;
        let value = unsafe { read_volatile(address::priority(self.base(), irq) as *const u32) };
        Ok(value as u8)
    }
}
