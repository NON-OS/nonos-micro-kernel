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

pub const SAMPLES: usize = 32;

// A fixed-capacity ring of one-second samples. `head` is the next write slot;
// `len` saturates at SAMPLES, so a fresh ring reports only the points it holds
// rather than a run of zeroes that would read as real idle time.
#[derive(Clone, Copy)]
pub struct Ring {
    cpu: [u8; SAMPLES],
    mem_kb: [u32; SAMPLES],
    head: usize,
    len: usize,
}

impl Ring {
    pub fn new() -> Self {
        Ring { cpu: [0; SAMPLES], mem_kb: [0; SAMPLES], head: 0, len: 0 }
    }

    pub fn push(&mut self, cpu: u8, mem_kb: u64) {
        self.cpu[self.head] = cpu;
        self.mem_kb[self.head] = mem_kb.min(u32::MAX as u64) as u32;
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

    // Oldest-first index into the ring, so a painter can walk 0..len() left to
    // right without knowing where the head sits.
    fn slot(&self, i: usize) -> usize {
        (self.head + SAMPLES - self.len + i) % SAMPLES
    }

    pub fn cpu_at(&self, i: usize) -> u8 {
        if i >= self.len {
            0
        } else {
            self.cpu[self.slot(i)]
        }
    }

    pub fn mem_at(&self, i: usize) -> u32 {
        if i >= self.len {
            0
        } else {
            self.mem_kb[self.slot(i)]
        }
    }

    pub fn oldest_mem(&self) -> u32 {
        self.mem_at(0)
    }

    pub fn peak_cpu(&self) -> u8 {
        (0..self.len).map(|i| self.cpu_at(i)).max().unwrap_or(0)
    }
}
