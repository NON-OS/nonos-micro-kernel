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

use super::entry::Entry;
use crate::calc::fixed::Fixed;

pub const CAP: usize = 32;

pub struct Ring {
    items: [Entry; CAP],
    len: usize,
}

impl Ring {
    pub const fn new() -> Self {
        Ring { items: [Entry::empty(); CAP], len: 0 }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn get(&self, index: usize) -> Option<&Entry> {
        if index < self.len {
            Some(&self.items[index])
        } else {
            None
        }
    }
    pub fn push(&mut self, expr: &[u8], value: Fixed) {
        let mut i = self.len.min(CAP - 1);
        while i > 0 {
            self.items[i] = self.items[i - 1];
            i -= 1;
        }
        self.items[0] = Entry::new(expr, value);
        if self.len < CAP {
            self.len += 1;
        }
    }
}
