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

use super::state::StackSetup;
use crate::elf::auxv::AuxEntry;
use crate::elf::errors::{ElfError, ElfResult};
use crate::elf::stack::layout::POINTER_SIZE;
use crate::memory::addr::VirtAddr;
use core::ptr;

impl StackSetup {
    pub(super) fn push_string(&mut self, s: &str) -> ElfResult<VirtAddr> {
        let bytes = s.as_bytes();
        let len = bytes.len() + 1;
        if len > self.available_space() {
            return Err(ElfError::MemoryAllocationFailed);
        }
        self.current = VirtAddr::new(self.current.as_u64() - len as u64);
        unsafe {
            let dst = self.current.as_mut_ptr::<u8>();
            ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
            ptr::write(dst.add(bytes.len()), 0);
        }
        Ok(self.current)
    }

    pub(super) fn push_u64(&mut self, value: u64) -> ElfResult<VirtAddr> {
        if POINTER_SIZE > self.available_space() {
            return Err(ElfError::MemoryAllocationFailed);
        }
        self.current = VirtAddr::new(self.current.as_u64() - POINTER_SIZE as u64);
        unsafe { ptr::write(self.current.as_mut_ptr::<u64>(), value) };
        Ok(self.current)
    }

    pub(super) fn push_pointer_array(&mut self, ptrs: &[VirtAddr]) -> ElfResult<VirtAddr> {
        self.push_u64(0)?;
        for ptr in ptrs.iter().rev() {
            self.push_u64(ptr.as_u64())?;
        }
        Ok(self.current)
    }

    pub(super) fn push_auxv(&mut self, auxv: &[AuxEntry]) -> ElfResult<VirtAddr> {
        for entry in auxv.iter().rev() {
            self.push_u64(entry.a_val)?;
            self.push_u64(entry.a_type)?;
        }
        Ok(self.current)
    }
}
