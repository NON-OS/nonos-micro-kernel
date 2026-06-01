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

use super::DynamicInfo;

#[test]
fn test_dynamic_info_new() {
    let info = DynamicInfo::new();
    assert!(info.needed_libraries.is_empty());
    assert!(info.symbol_table.is_none());
    assert!(info.string_table.is_none());
    assert_eq!(info.string_table_size, 0);
    assert!(!info.needs_relocation());
    assert!(!info.needs_linking());
}

#[test]
fn test_dynamic_info_rela_count() {
    let mut info = DynamicInfo::new();
    info.rela_size = 72;
    assert_eq!(info.rela_count(), 3);
}
