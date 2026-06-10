// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::loader::*;

#[test]
fn test_rela64_parsing() {
    let rela = reloc::Rela64 { r_offset: 0x1000, r_info: (10u64 << 32) | 8, r_addend: -100 };

    assert_eq!(rela.reloc_type(), reloc::reloc_type::R_X86_64_RELATIVE);
    assert_eq!(rela.symbol_index(), 10);
}

#[test]
fn test_relocation_context() {
    let ctx = reloc::RelocationContext::new(0x100000, 0x50000);
    assert_eq!(ctx.base_addr, 0x100000);
    assert_eq!(ctx.load_bias, 0x50000);
}

#[test]
fn test_dynamic_info_defaults() {
    let info = types::DynamicInfo::default();
    assert!(!info.has_relocations());
    assert!(!info.has_symbols());
}

#[test]
fn test_dynamic_info_with_relocations() {
    let mut info = types::DynamicInfo::default();
    info.rela_addr = Some(0x1000);
    info.rela_size = 240;
    info.rela_ent = 24;

    assert!(info.has_relocations());
}
