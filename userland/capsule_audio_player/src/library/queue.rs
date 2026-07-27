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
use alloc::vec::Vec;

pub struct Queue {
    items: Vec<usize>,
    current: usize,
}

impl Queue {
    pub fn new() -> Self {
        Queue { items: Vec::new(), current: 0 }
    }

    pub fn enqueue(&mut self, track: usize) {
        if !self.items.contains(&track) {
            self.items.push(track);
        }
    }

    pub fn remove(&mut self, track: usize) {
        if let Some(pos) = self.items.iter().position(|&t| t == track) {
            self.items.remove(pos);
            if self.current > pos || self.current >= self.items.len() {
                self.current = self.current.saturating_sub(1);
            }
        }
    }

    pub fn contains(&self, track: usize) -> bool {
        self.items.contains(&track)
    }

    pub fn items(&self) -> &[usize] {
        &self.items
    }

    pub fn current(&self) -> Option<usize> {
        self.items.get(self.current).copied()
    }

    pub fn focus(&mut self, track: usize) {
        if let Some(pos) = self.items.iter().position(|&t| t == track) {
            self.current = pos;
        }
    }

    pub fn has_next(&self) -> bool {
        self.current + 1 < self.items.len()
    }

    pub fn advance(&mut self) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }
        self.current = (self.current + 1) % self.items.len();
        self.current()
    }

    pub fn back(&mut self) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }
        self.current = (self.current + self.items.len() - 1) % self.items.len();
        self.current()
    }

    pub fn shuffle(&mut self, seed: u64) {
        let cur = self.current();
        let mut s = seed | 1;
        let mut i = self.items.len();
        while i > 1 {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            i -= 1;
            self.items.swap(i, (s % (i as u64 + 1)) as usize);
        }
        self.refocus(cur);
    }

    pub fn restore_order(&mut self) {
        let cur = self.current();
        self.items.sort_unstable();
        self.refocus(cur);
    }

    fn refocus(&mut self, track: Option<usize>) {
        if let Some(t) = track {
            self.focus(t);
        }
    }
}
