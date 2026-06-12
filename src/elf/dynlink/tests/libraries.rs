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
fn test_dyn_link_info_new() {
    let info = DynLinkInfo::new();
    assert!(info.needed_libraries.is_empty());
    assert!(info.symbol_table.is_none());
    assert!(info.string_table.is_none());
    assert_eq!(info.string_table_size, 0);
    assert!(info.is_empty());
}

#[test]
fn test_dyn_link_info_default() {
    assert!(DynLinkInfo::default().is_empty());
}

#[test]
fn test_needs_libraries() {
    let mut info = DynLinkInfo::new();
    assert!(!info.needs_libraries());
    info.add_needed("libc.so.6".into());
    assert!(info.needs_libraries());
    assert_eq!(info.library_count(), 1);
}

#[test]
fn test_needs_library() {
    let mut info = DynLinkInfo::new();
    info.add_needed("libc.so.6".into());
    info.add_needed("libm.so.6".into());
    assert!(info.needs_library("libc.so.6"));
    assert!(info.needs_library("libm.so.6"));
    assert!(!info.needs_library("libpthread.so.0"));
}

#[test]
fn test_is_empty() {
    let mut info = DynLinkInfo::new();
    assert!(info.is_empty());
    info.add_needed("libc.so.6".into());
    assert!(!info.is_empty());
    let mut other = DynLinkInfo::new();
    other.symbol_table = Some(VirtAddr::new(0x1000));
    assert!(!other.is_empty());
}
