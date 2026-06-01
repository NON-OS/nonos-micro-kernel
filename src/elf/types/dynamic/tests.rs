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

use super::state::DynamicEntry;
use crate::elf::types::dyn_tag;
use core::mem;

#[test]
fn test_dynamic_entry_size() {
    assert_eq!(mem::size_of::<DynamicEntry>(), DynamicEntry::SIZE);
}

#[test]
fn test_dynamic_entry() {
    let mut dyn_entry = DynamicEntry::default();
    assert!(dyn_entry.is_null());
    dyn_entry.d_tag = dyn_tag::DT_NEEDED;
    assert!(!dyn_entry.is_null());
    assert_eq!(dyn_entry.tag_name(), "NEEDED");
}
