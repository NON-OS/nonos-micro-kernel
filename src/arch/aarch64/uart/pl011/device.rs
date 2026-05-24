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

pub struct Pl011 {
    base: u64,
}

impl Pl011 {
    pub fn new(base: u64) -> Self {
        Self { base }
    }

    pub(crate) fn read_reg(&self, offset: u64) -> u32 {
        unsafe { read_volatile((self.base + offset) as *const u32) }
    }

    pub(crate) fn write_reg(&self, offset: u64, value: u32) {
        unsafe { write_volatile((self.base + offset) as *mut u32, value) }
    }
}
