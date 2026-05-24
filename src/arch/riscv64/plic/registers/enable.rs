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
    pub fn enable(&self, hart: usize, irq: u32) -> PlicResult<()> {
        address::valid_irq(irq)?;
        let addr = address::enable(self.base(), hart, irq);
        let bit = irq % 32;
        unsafe {
            let value = read_volatile(addr as *const u32);
            write_volatile(addr as *mut u32, value | (1 << bit));
        }
        Ok(())
    }

    pub fn disable(&self, hart: usize, irq: u32) -> PlicResult<()> {
        address::valid_irq(irq)?;
        let addr = address::enable(self.base(), hart, irq);
        let bit = irq % 32;
        unsafe {
            let value = read_volatile(addr as *const u32);
            write_volatile(addr as *mut u32, value & !(1 << bit));
        }
        Ok(())
    }
}
