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

use core::ptr::read_volatile;

use super::io::RegIo;
use super::pio_read::pio_read;
use super::regs_type::Regs;

impl Regs {
    pub unsafe fn r8(self, offset: usize) -> u8 {
        match self.io {
            RegIo::Mmio(base) => read_volatile(base.add(offset)),
            RegIo::Pio(grant) => pio_read(grant, offset, 1) as u8,
        }
    }
}
