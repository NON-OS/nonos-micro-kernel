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

use super::constants::{MSIP_BASE, MTIME, MTIMECMP_BASE};

pub struct Clint {
    base: u64,
}

impl Clint {
    pub const fn new(base: u64) -> Self {
        Self { base }
    }

    pub fn mtime(&self) -> u64 {
        unsafe { read_volatile((self.base + MTIME) as *const u64) }
    }

    pub fn set_mtimecmp(&self, hart: usize, value: u64) {
        let addr = self.base + MTIMECMP_BASE + (hart as u64 * 8);
        unsafe { write_volatile(addr as *mut u64, value) }
    }

    pub fn get_mtimecmp(&self, hart: usize) -> u64 {
        let addr = self.base + MTIMECMP_BASE + (hart as u64 * 8);
        unsafe { read_volatile(addr as *const u64) }
    }

    pub fn send_ipi(&self, hart: usize) {
        let addr = self.base + MSIP_BASE + (hart as u64 * 4);
        unsafe { write_volatile(addr as *mut u32, 1) }
    }

    pub fn clear_ipi(&self, hart: usize) {
        let addr = self.base + MSIP_BASE + (hart as u64 * 4);
        unsafe { write_volatile(addr as *mut u32, 0) }
    }

    pub fn is_ipi_pending(&self, hart: usize) -> bool {
        let addr = self.base + MSIP_BASE + (hart as u64 * 4);
        unsafe { read_volatile(addr as *const u32) != 0 }
    }
}
