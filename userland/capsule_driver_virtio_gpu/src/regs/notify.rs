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
use super::io::RegIo;
use super::pio;
use super::state::Regs;
impl Regs {
    #[inline]
    pub unsafe fn notify(self, queue: u16) {
        match self.notify {
            RegIo::Mmio(base) => write_volatile((base as usize + self.notify_offset) as *mut u16, queue),
            RegIo::Pio(grant) => pio::write(grant, self.notify_offset, 2, queue as u32),
        }
    }
}
