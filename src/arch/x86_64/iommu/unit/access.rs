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

//! Register access to one remapping unit. The registers live in a 4 KiB MMIO
//! page mapped from the DRHD base.

/// A mapped remapping unit. Holding one means the register page is mapped.
#[derive(Debug, Clone, Copy)]
pub struct RemapUnit {
    base_va: u64,
    base_pa: u64,
}

/// Register window of a unit, per the spec.
pub const UNIT_WINDOW: usize = 4096;

impl RemapUnit {
    /// # Safety
    /// `base_va` must be a live mapping of `UNIT_WINDOW` uncached bytes over
    /// the unit's `base_pa`, valid for as long as this value is used.
    pub const unsafe fn from_mapped(base_va: u64, base_pa: u64) -> Self {
        Self { base_va, base_pa }
    }

    pub const fn base_pa(&self) -> u64 {
        self.base_pa
    }

    pub fn read32(&self, offset: usize) -> u32 {
        debug_assert!(offset + 4 <= UNIT_WINDOW);
        // SAFETY: offset is inside the mapped register window this value
        // promises, and the registers are uncached device memory.
        unsafe { core::ptr::read_volatile((self.base_va as usize + offset) as *const u32) }
    }

    pub fn read64(&self, offset: usize) -> u64 {
        debug_assert!(offset + 8 <= UNIT_WINDOW);
        // SAFETY: as read32.
        unsafe { core::ptr::read_volatile((self.base_va as usize + offset) as *const u64) }
    }

    /// # Safety
    /// Writing a remapping register changes how devices reach memory. The
    /// caller owns the sequencing the spec requires around the register.
    pub unsafe fn write32(&self, offset: usize, value: u32) {
        debug_assert!(offset + 4 <= UNIT_WINDOW);
        // SAFETY: offset is inside the mapped window; the caller owns meaning.
        unsafe { core::ptr::write_volatile((self.base_va as usize + offset) as *mut u32, value) }
    }

    /// # Safety
    /// As `write32`.
    pub unsafe fn write64(&self, offset: usize, value: u64) {
        debug_assert!(offset + 8 <= UNIT_WINDOW);
        // SAFETY: offset is inside the mapped window; the caller owns meaning.
        unsafe { core::ptr::write_volatile((self.base_va as usize + offset) as *mut u64, value) }
    }
}
