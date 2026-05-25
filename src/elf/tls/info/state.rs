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

use super::constants::{DEFAULT_TLS_ALIGNMENT, TCB_SIZE};
use crate::memory::addr::VirtAddr;

#[derive(Debug, Clone, Copy)]
pub struct TlsInfo {
    pub template_addr: VirtAddr,
    pub template_size: usize,
    pub memory_size: usize,
    pub alignment: usize,
}

impl TlsInfo {
    pub fn new(template_addr: VirtAddr, template_size: usize, memory_size: usize, alignment: usize) -> Self {
        Self { template_addr, template_size, memory_size, alignment: alignment.max(1) }
    }

    pub fn bss_size(&self) -> usize {
        self.memory_size.saturating_sub(self.template_size)
    }

    pub fn has_bss(&self) -> bool {
        self.memory_size > self.template_size
    }

    pub fn effective_alignment(&self) -> usize {
        self.alignment.max(DEFAULT_TLS_ALIGNMENT)
    }

    pub fn allocation_size(&self) -> usize {
        let align = self.effective_alignment();
        (self.memory_size + align - 1) & !(align - 1)
    }

    pub fn total_size_with_tcb(&self) -> usize {
        self.allocation_size() + TCB_SIZE
    }

    pub fn is_empty(&self) -> bool {
        self.memory_size == 0
    }

    pub fn template_end(&self) -> VirtAddr {
        self.template_addr + self.template_size as u64
    }
}

impl Default for TlsInfo {
    fn default() -> Self {
        Self { template_addr: VirtAddr::new(0), template_size: 0, memory_size: 0, alignment: DEFAULT_TLS_ALIGNMENT }
    }
}
