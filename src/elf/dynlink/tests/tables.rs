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

use super::super::DynLinkInfo;
use crate::memory::addr::VirtAddr;

#[test]
fn test_has_symbols() {
    let mut info = DynLinkInfo::new();
    assert!(!info.has_symbols());
    info.symbol_table = Some(VirtAddr::new(0x1000));
    assert!(info.has_symbols());
}

#[test]
fn test_has_strings() {
    let mut info = DynLinkInfo::new();
    assert!(!info.has_strings());
    info.string_table = Some(VirtAddr::new(0x1000));
    assert!(!info.has_strings());
    info.string_table_size = 1024;
    assert!(info.has_strings());
}

#[test]
fn test_has_init_fini() {
    let mut info = DynLinkInfo::new();
    assert!(!info.has_init());
    assert!(!info.has_fini());
    info.init_function = Some(VirtAddr::new(0x1000));
    info.fini_function = Some(VirtAddr::new(0x2000));
    assert!(info.has_init());
    assert!(info.has_fini());
}

#[test]
fn test_string_table_end() {
    let mut info = DynLinkInfo::new();
    assert!(info.string_table_end().is_none());
    info.string_table = Some(VirtAddr::new(0x1000));
    info.string_table_size = 0x200;
    assert_eq!(info.string_table_end(), Some(VirtAddr::new(0x1200)));
}
