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

pub const SPARK_SAMPLES: usize = 48;

/// A fixed window of CPU percentages, newest last. `head` is the write cursor,
/// so while the window is still filling it equals the count and the
/// chronological read below collapses to the identity.
#[derive(Clone, Copy)]
pub struct SparkRing {
    data: [u8; SPARK_SAMPLES],
    head: usize,
    len: usize,
}

impl SparkRing {
    pub const fn new() -> Self {
        SparkRing { data: [0; SPARK_SAMPLES], head: 0, len: 0 }
    }

    pub fn push(&mut self, v: u8) {
        self.data[self.head] = v.min(100);
        self.head = (self.head + 1) % SPARK_SAMPLES;
        if self.len < SPARK_SAMPLES {
            self.len += 1;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn slice(&self) -> &[u8] {
        &self.data[..self.len()]
    }

    pub fn start(&self) -> usize {
        if self.len() == 0 {
            0
        } else {
            self.head % self.len()
        }
    }
}
