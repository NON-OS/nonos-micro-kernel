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

use super::base_addr;
use super::load_parts::LoadedParts;
use crate::elf::loader::ElfImage;
use crate::elf::types::ElfHeader;
use crate::memory::addr::VirtAddr;

pub(super) fn from_parts(header: &ElfHeader, base_addr: VirtAddr, parts: LoadedParts) -> ElfImage {
    let total_size = parts.segments.iter().map(|segment| segment.size).sum();
    ElfImage {
        base_addr,
        entry_point: base_addr::entry_point(header, base_addr),
        size: total_size,
        memory_size: total_size,
        segments: parts.segments,
        dynamic_info: parts.dynamic_info,
        dynlink_info: None,
        tls_info: parts.tls_info,
        interpreter: parts.interpreter,
    }
}
