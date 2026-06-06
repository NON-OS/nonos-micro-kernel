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
use super::super::io::RegIo;
use super::types::Regs;
use crate::regs::pio;
use core::ptr::write_volatile;

impl Regs {
    #[inline]
    pub unsafe fn w8(self, offset: usize, value: u8) {
        match self.io {
            RegIo::Mmio(base) => write_volatile(base.add(offset), value),
            RegIo::Pio(grant) => pio::write(grant, offset, 1, value as u32),
        }
    }
}
