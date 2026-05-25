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

extern crate alloc;

use crate::elf::types::Symbol;
use crate::memory::addr::VirtAddr;
use alloc::string::String;
use core::ptr;

pub(super) unsafe fn read_symbol(symtab: VirtAddr, index: usize) -> Option<Symbol> {
    let sym_ptr = (symtab.as_u64() + (index * Symbol::SIZE) as u64) as *const Symbol;
    Some(unsafe { ptr::read(sym_ptr) })
}

pub(super) unsafe fn read_name(strtab: VirtAddr, strtab_size: usize, offset: usize) -> String {
    let ptr = (strtab.as_u64() + offset as u64) as *const u8;
    let max_len = strtab_size.saturating_sub(offset).min(256);
    let mut name = String::new();
    for index in 0..max_len {
        let ch = unsafe { *ptr.add(index) };
        if ch == 0 {
            break;
        }
        name.push(ch as char);
    }
    name
}
