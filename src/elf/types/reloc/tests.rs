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

use super::state::RelaEntry;
use crate::elf::types::reloc_type;
use core::mem;

#[test]
fn test_rela_entry_size() {
    assert_eq!(mem::size_of::<RelaEntry>(), RelaEntry::SIZE);
}

#[test]
fn test_rela_entry_info() {
    let mut rela = RelaEntry::default();
    rela.r_info = RelaEntry::make_info(42, reloc_type::R_X86_64_64);
    assert_eq!(rela.symbol_index(), 42);
    assert_eq!(rela.reloc_type(), reloc_type::R_X86_64_64);
    assert_eq!(rela.type_name(), "R_X86_64_64");
}
