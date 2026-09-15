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

//! A bounded FIFO that says when it was overrun rather than growing.

use std::collections::VecDeque;

pub struct Fifo<T> {
    depth: usize,
    items: VecDeque<T>,
}

impl<T> Fifo<T> {
    pub fn new(depth: u32) -> Self {
        Self { depth: depth as usize, items: VecDeque::new() }
    }

    /// False when the entry would not fit; the entry is then lost, which is
    /// what the core does with the byte that overflows it.
    pub fn push(&mut self, item: T) -> bool {
        if self.items.len() >= self.depth {
            return false;
        }
        self.items.push_back(item);
        true
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn len(&self) -> u32 {
        self.items.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.items.len() >= self.depth
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}
