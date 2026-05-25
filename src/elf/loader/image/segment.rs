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

use crate::elf::types::phdr_type;
use crate::memory::addr::VirtAddr;
use x86_64::structures::paging::PageTableFlags;

#[derive(Debug, Clone)]
pub struct LoadedSegment {
    pub vaddr: VirtAddr,
    pub size: usize,
    pub flags: PageTableFlags,
    pub segment_type: u32,
}

impl LoadedSegment {
    pub fn is_readable(&self) -> bool {
        self.flags.contains(PageTableFlags::PRESENT)
    }

    pub fn is_writable(&self) -> bool {
        self.flags.contains(PageTableFlags::WRITABLE)
    }

    pub fn is_executable(&self) -> bool {
        !self.flags.contains(PageTableFlags::NO_EXECUTE)
    }

    pub fn end_addr(&self) -> VirtAddr {
        self.vaddr + self.size as u64
    }

    pub fn type_name(&self) -> &'static str {
        match self.segment_type {
            phdr_type::PT_LOAD => "LOAD",
            phdr_type::PT_DYNAMIC => "DYNAMIC",
            phdr_type::PT_INTERP => "INTERP",
            phdr_type::PT_NOTE => "NOTE",
            phdr_type::PT_TLS => "TLS",
            phdr_type::PT_GNU_EH_FRAME => "EH_FRAME",
            phdr_type::PT_GNU_STACK => "GNU_STACK",
            phdr_type::PT_GNU_RELRO => "RELRO",
            _ => "UNKNOWN",
        }
    }
}
