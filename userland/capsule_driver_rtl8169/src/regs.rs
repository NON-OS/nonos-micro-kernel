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

use core::ptr;

/// The register window, by base address.
///
/// Every accessor is unsafe under one contract: `offset` must lie inside the
/// BAR the caller mapped at `base`, and the caller must be the driver that
/// owns the part, since reads and writes here reach the device directly.
#[derive(Clone, Copy)]
pub struct Regs {
    base: u64,
}

impl Regs {
    pub const fn new(base: u64) -> Self {
        Self { base }
    }

    /// # Safety
    ///
    /// `offset` lies inside the mapped window; see the type.
    pub unsafe fn r8(&self, offset: usize) -> u8 {
        ptr::read_volatile((self.base as usize + offset) as *const u8)
    }

    /// # Safety
    ///
    /// `offset` lies inside the mapped window; see the type.
    pub unsafe fn r16(&self, offset: usize) -> u16 {
        ptr::read_volatile((self.base as usize + offset) as *const u16)
    }

    /// # Safety
    ///
    /// `offset` lies inside the mapped window; see the type.
    pub unsafe fn r32(&self, offset: usize) -> u32 {
        ptr::read_volatile((self.base as usize + offset) as *const u32)
    }

    /// # Safety
    ///
    /// `offset` lies inside the mapped window; see the type.
    pub unsafe fn w8(&self, offset: usize, value: u8) {
        ptr::write_volatile((self.base as usize + offset) as *mut u8, value);
    }

    /// # Safety
    ///
    /// `offset` lies inside the mapped window; see the type.
    pub unsafe fn w16(&self, offset: usize, value: u16) {
        ptr::write_volatile((self.base as usize + offset) as *mut u16, value);
    }

    /// # Safety
    ///
    /// `offset` lies inside the mapped window; see the type.
    pub unsafe fn w32(&self, offset: usize, value: u32) {
        ptr::write_volatile((self.base as usize + offset) as *mut u32, value);
    }
}
