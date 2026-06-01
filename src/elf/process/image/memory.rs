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

use crate::elf::loader::ElfImage;
use crate::memory::addr::VirtAddr;

use super::state::ProcessImage;

pub(super) fn calculate_brk(image: &ElfImage) -> VirtAddr {
    let mut highest = image.base_addr.as_u64();
    for segment in &image.segments {
        let end = segment.vaddr.as_u64() + segment.size as u64;
        if end > highest {
            highest = end;
        }
    }
    VirtAddr::new((highest + 0xFFF) & !0xFFF)
}

impl ProcessImage {
    pub fn total_memory_size(&self) -> usize {
        let interp_size = self.interpreter.as_ref().map_or(0, |image| image.size);
        self.executable.size + interp_size + self.stack.stack_size()
    }

    pub fn extend_brk(&mut self, increment: usize) -> Option<VirtAddr> {
        let new_brk = self.brk_current.as_u64().checked_add(increment as u64)?;
        self.brk_current = VirtAddr::new(new_brk);
        Some(self.brk_current)
    }
}
