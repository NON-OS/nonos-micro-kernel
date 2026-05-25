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

use super::state::Symbol;
use crate::elf::types::{sym_bind, sym_type};
use core::mem;

#[test]
fn test_symbol_size() {
    assert_eq!(mem::size_of::<Symbol>(), Symbol::SIZE);
}

#[test]
fn test_symbol_info() {
    let mut sym = Symbol::default();
    sym.st_info = (sym_bind::STB_GLOBAL << 4) | sym_type::STT_FUNC;
    assert!(sym.is_global());
    assert!(!sym.is_local());
    assert!(sym.is_function());
    assert!(!sym.is_object());
}
