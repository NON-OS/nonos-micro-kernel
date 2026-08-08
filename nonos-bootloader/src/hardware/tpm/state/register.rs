// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::core::TpmState;

impl TpmState {
    pub(crate) fn read_reg8(&self, offset: u32) -> u8 {
        let addr = (self.base + offset as u64) as *const u8;
        unsafe { core::ptr::read_volatile(addr) }
    }

    pub(crate) fn write_reg8(&self, offset: u32, value: u8) {
        let addr = (self.base + offset as u64) as *mut u8;
        unsafe { core::ptr::write_volatile(addr, value) }
    }

    pub(crate) fn read_reg32(&self, offset: u32) -> u32 {
        let addr = (self.base + offset as u64) as *const u32;
        unsafe { core::ptr::read_volatile(addr) }
    }

    /// The CRB control registers are all 32 bits wide and several of them will
    /// not accept a byte write, so they need this rather than `write_reg8`.
    pub(crate) fn write_reg32(&self, offset: u32, value: u32) {
        let addr = (self.base + offset as u64) as *mut u32;
        unsafe { core::ptr::write_volatile(addr, value) }
    }
}
