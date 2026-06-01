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

use alloc::string::String;

use crate::elf::types::{sym_bind, sym_type};
use crate::memory::addr::VirtAddr;

#[derive(Debug, Clone)]
pub struct ResolvedSymbol {
    pub name: String,
    pub address: VirtAddr,
    pub size: u64,
    pub binding: u8,
    pub sym_type: u8,
    pub library_id: usize,
}

impl ResolvedSymbol {
    pub fn is_function(&self) -> bool { self.sym_type == sym_type::STT_FUNC }
    pub fn is_object(&self) -> bool { self.sym_type == sym_type::STT_OBJECT }
    pub fn is_global(&self) -> bool { self.binding == sym_bind::STB_GLOBAL }
    pub fn is_weak(&self) -> bool { self.binding == sym_bind::STB_WEAK }
}
