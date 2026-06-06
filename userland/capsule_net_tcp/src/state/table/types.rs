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

use alloc::vec::Vec;
use spin::Mutex;

use crate::state::Entry;

pub const TABLE_CAP: usize = 256;
pub static TABLE: Mutex<Table> = Mutex::new(Table::new());

pub struct Table {
    pub(super) entries: Vec<Entry>,
    pub(super) next_handle: u32,
    pub(super) next_iss: u32,
}

impl Table {
    pub const fn new() -> Self {
        Self { entries: Vec::new(), next_handle: 1, next_iss: 0x7000_0000 }
    }

    pub fn next_iss(&mut self) -> u32 {
        self.next_iss = self.next_iss.wrapping_add(0x10001);
        self.next_iss
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TableError {
    Full,
    NotFound,
}
