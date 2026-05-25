// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::state::{DEFAULT_PIE_BASE, DEFAULT_STATIC_BASE};
use crate::elf::aslr::AslrManager;
use crate::elf::types::ElfHeader;
use crate::memory::addr::VirtAddr;

pub(super) fn load_base(header: &ElfHeader, aslr: &mut AslrManager) -> VirtAddr {
    if header.is_pie() {
        VirtAddr::new(aslr.randomize_base(DEFAULT_PIE_BASE))
    } else {
        VirtAddr::new(DEFAULT_STATIC_BASE)
    }
}

pub(super) fn entry_point(header: &ElfHeader, base_addr: VirtAddr) -> VirtAddr {
    if header.is_pie() {
        base_addr + header.e_entry
    } else {
        VirtAddr::new(header.e_entry)
    }
}
