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

extern crate alloc;

use crate::elf::loader::image::DynamicInfo;
use alloc::vec::Vec;

pub(in crate::elf::loader::core::parse_dynamic) struct DynamicParseState {
    pub dynamic_info: DynamicInfo,
    pub needed_offsets: Vec<u64>,
}

impl DynamicParseState {
    pub(in crate::elf::loader::core::parse_dynamic) fn new() -> Self {
        Self { dynamic_info: DynamicInfo::new(), needed_offsets: Vec::new() }
    }
}
