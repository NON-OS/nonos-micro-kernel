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
fn test_has_relocations() {
    let mut info = DynLinkInfo::new();
    assert!(!info.has_relocations());
    info.rela_table = Some(VirtAddr::new(0x1000));
    assert!(info.has_relocations());
    info.rela_table = None;
    info.plt_relocations = Some(VirtAddr::new(0x2000));
    assert!(info.has_relocations());
}

#[test]
fn test_rela_count() {
    let mut info = DynLinkInfo::new();
    info.rela_size = 72;
    assert_eq!(info.rela_count(), 3);
}

#[test]
fn test_plt_rela_count() {
    let mut info = DynLinkInfo::new();
    info.plt_rela_size = 48;
    assert_eq!(info.plt_rela_count(), 2);
}

#[test]
fn test_total_relocation_count() {
    let mut info = DynLinkInfo::new();
    info.rela_size = 72;
    info.plt_rela_size = 48;
    assert_eq!(info.total_relocation_count(), 5);
}
