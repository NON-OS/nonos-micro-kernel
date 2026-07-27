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

use alloc::vec;
use alloc::vec::Vec;

pub const FEED_SAMPLES: usize = 0x4000;

pub struct PcmRing {
    buf: Vec<i16>,
    head: usize,
    len: usize,
}

impl PcmRing {
    pub fn new() -> Self {
        Self { buf: vec![0i16; FEED_SAMPLES], head: 0, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }

    pub fn push(&mut self, s: &[i16]) -> usize {
        let free = FEED_SAMPLES - self.len;
        let n = core::cmp::min(free, s.len());
        let mut write = (self.head + self.len) % FEED_SAMPLES;
        for i in 0..n {
            self.buf[write] = s[i];
            write = (write + 1) % FEED_SAMPLES;
        }
        self.len += n;
        n
    }

    pub fn pop_period(&mut self, out: &mut [i16]) -> usize {
        let n = core::cmp::min(self.len, out.len());
        for i in 0..n {
            out[i] = self.buf[self.head];
            self.head = (self.head + 1) % FEED_SAMPLES;
        }
        self.len -= n;
        for i in n..out.len() {
            out[i] = 0;
        }
        n
    }
}
