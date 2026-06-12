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

use crate::memory::addr::VirtAddr;
use alloc::collections::BTreeMap;
use alloc::string::String;

pub struct RelocationContext<'a> {
    pub symbol_table: Option<VirtAddr>,
    pub string_table: Option<VirtAddr>,
    pub string_table_size: usize,
    pub got_base: Option<VirtAddr>,
    pub symbol_cache: &'a BTreeMap<String, VirtAddr>,
}

impl<'a> RelocationContext<'a> {
    pub fn empty(cache: &'a BTreeMap<String, VirtAddr>) -> Self {
        Self {
            symbol_table: None,
            string_table: None,
            string_table_size: 0,
            got_base: None,
            symbol_cache: cache,
        }
    }
}
