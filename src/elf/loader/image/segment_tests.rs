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

use super::LoadedSegment;
use crate::elf::phdr_type;
use crate::memory::addr::VirtAddr;
use x86_64::structures::paging::PageTableFlags;

#[test]
fn test_loaded_segment_type_name() {
    let segment = LoadedSegment {
        vaddr: VirtAddr::new(0x1000),
        size: 4096,
        flags: PageTableFlags::PRESENT,
        segment_type: phdr_type::PT_LOAD,
    };
    assert_eq!(segment.type_name(), "LOAD");
}

#[test]
fn test_loaded_segment_permissions() {
    let segment = LoadedSegment {
        vaddr: VirtAddr::new(0x1000),
        size: 4096,
        flags: PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        segment_type: phdr_type::PT_LOAD,
    };
    assert!(segment.is_readable());
    assert!(segment.is_writable());
    assert!(segment.is_executable());
}

#[test]
fn test_loaded_segment_end_addr() {
    let segment = LoadedSegment {
        vaddr: VirtAddr::new(0x1000),
        size: 4096,
        flags: PageTableFlags::PRESENT,
        segment_type: phdr_type::PT_LOAD,
    };
    assert_eq!(segment.end_addr(), VirtAddr::new(0x2000));
}
