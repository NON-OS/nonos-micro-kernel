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

use alloc::vec::Vec;

use super::super::apply::process_relocations;
use crate::elf::loader::ElfImage;
use crate::memory::addr::VirtAddr;

#[test]
fn test_process_empty_relocations() {
    let image = ElfImage {
        base_addr: VirtAddr::new(0x400000),
        entry_point: VirtAddr::new(0x401000),
        size: 4096,
        memory_size: 4096,
        segments: Vec::new(),
        dynamic_info: None,
        dynlink_info: None,
        tls_info: None,
        interpreter: None,
    };
    assert!(process_relocations(&image, &[]).is_ok());
}
