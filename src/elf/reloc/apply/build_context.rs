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

extern crate alloc;

use super::super::context::RelocationContext;
use crate::elf::loader::ElfImage;
use crate::memory::addr::VirtAddr;
use alloc::collections::BTreeMap;
use alloc::string::String;

pub(super) fn context_for<'a>(
    image: &ElfImage,
    empty_cache: &'a BTreeMap<String, VirtAddr>,
) -> RelocationContext<'a> {
    if let Some(dynamic) = &image.dynamic_info {
        RelocationContext {
            symbol_table: dynamic.symbol_table,
            string_table: dynamic.string_table,
            string_table_size: dynamic.string_table_size,
            got_base: None,
            symbol_cache: empty_cache,
        }
    } else {
        RelocationContext::empty(empty_cache)
    }
}
