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

impl Symbol {
    pub fn binding(&self) -> u8 { self.st_info >> 4 }
    pub fn sym_type(&self) -> u8 { self.st_info & 0x0F }
    pub fn is_local(&self) -> bool { self.binding() == sym_bind::STB_LOCAL }
    pub fn is_global(&self) -> bool { self.binding() == sym_bind::STB_GLOBAL }
    pub fn is_weak(&self) -> bool { self.binding() == sym_bind::STB_WEAK }
    pub fn is_function(&self) -> bool { self.sym_type() == sym_type::STT_FUNC }
    pub fn is_object(&self) -> bool { self.sym_type() == sym_type::STT_OBJECT }
    pub fn is_undefined(&self) -> bool { self.st_shndx == 0 }
}
