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

//! A ring of one per-second figure, one entry per refresh, the shape of a
//! kernel rate over the last minute. Same window as the cpu ring.

use super::samples::SAMPLES;

#[derive(Clone, Copy)]
pub struct RateRing {
    v: [u32; SAMPLES],
    head: usize,
    len: usize,
}

impl RateRing {
    pub const fn new() -> Self {
        RateRing { v: [0; SAMPLES], head: 0, len: 0 }
    }

    pub fn push(&mut self, value: u32) {
        self.v[self.head] = value;
        self.head = (self.head + 1) % SAMPLES;
        if self.len < SAMPLES {
            self.len += 1;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The i-th oldest sample; zero past the end.
    pub fn at(&self, i: usize) -> u32 {
        if i >= self.len {
            0
        } else {
            self.v[(self.head + SAMPLES - self.len + i) % SAMPLES]
        }
    }

    pub fn peak(&self) -> u32 {
        (0..self.len).map(|i| self.at(i)).max().unwrap_or(0)
    }

    pub fn latest(&self) -> u32 {
        if self.len == 0 {
            0
        } else {
            self.at(self.len - 1)
        }
    }
}
