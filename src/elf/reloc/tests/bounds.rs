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

use alloc::{collections::BTreeMap, vec::Vec};

use super::super::{process_relocations_with_context, RelocationContext};
use crate::elf::errors::ElfError;
use crate::elf::loader::{ElfImage, LoadedSegment};
use crate::elf::types::{reloc_type, RelaEntry};
use crate::memory::addr::VirtAddr;
use x86_64::structures::paging::PageTableFlags;

#[test]
fn test_relocation_rejects_target_outside_loaded_segments() {
    let image = image_with(LoadedSegment {
        vaddr: VirtAddr::new(0x401000),
        size: 0x100,
        flags: PageTableFlags::PRESENT | PageTableFlags::NO_EXECUTE,
        segment_type: 1,
    });
    let rela =
        [RelaEntry { r_offset: 0x800, r_info: reloc_type::R_X86_64_RELATIVE as u64, r_addend: 0 }];
    assert!(matches!(run(&image, &rela), Err(ElfError::RelocationFailed)));
}

#[test]
fn test_irelative_rejects_resolver_outside_executable_segment() {
    let image = image_with(LoadedSegment {
        vaddr: VirtAddr::new(0x400000),
        size: 8,
        flags: PageTableFlags::PRESENT | PageTableFlags::NO_EXECUTE,
        segment_type: 1,
    });
    let rela = [RelaEntry {
        r_offset: 0,
        r_info: reloc_type::R_X86_64_IRELATIVE as u64,
        r_addend: 0x2000,
    }];
    assert!(matches!(run(&image, &rela), Err(ElfError::RelocationFailed)));
}

fn run(image: &ElfImage, rela: &[RelaEntry]) -> Result<(), ElfError> {
    process_relocations_with_context(image, rela, &RelocationContext::empty(&BTreeMap::new()))
}

fn image_with(segment: LoadedSegment) -> ElfImage {
    ElfImage {
        base_addr: VirtAddr::new(0x400000),
        entry_point: VirtAddr::new(0x401000),
        size: 0x2000,
        memory_size: 0x2000,
        segments: Vec::from([segment]),
        dynamic_info: None,
        dynlink_info: None,
        tls_info: None,
        interpreter: None,
    }
}
