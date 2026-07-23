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

pub const QUEUE_BYTES: usize = 0x1_0000;

pub struct PcmQueue {
    buf: Vec<u8>,
    head: usize,
    len: usize,
}

impl PcmQueue {
    pub fn new() -> Self {
        Self { buf: vec![0u8; QUEUE_BYTES], head: 0, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_full_for(&self, n: usize) -> bool {
        QUEUE_BYTES - self.len < n
    }

    pub fn clear(&mut self) {
        self.head = 0;
        self.len = 0;
    }

    pub fn push(&mut self, data: &[u8]) -> usize {
        let space = QUEUE_BYTES - self.len;
        let n = core::cmp::min(space, data.len());
        let mut tail = (self.head + self.len) % QUEUE_BYTES;
        for &b in &data[..n] {
            self.buf[tail] = b;
            tail = (tail + 1) % QUEUE_BYTES;
        }
        self.len += n;
        n
    }

    pub fn pop_into(&mut self, out: &mut [u8]) -> usize {
        let n = core::cmp::min(self.len, out.len());
        for slot in out.iter_mut().take(n) {
            *slot = self.buf[self.head];
            self.head = (self.head + 1) % QUEUE_BYTES;
        }
        self.len -= n;
        for slot in out.iter_mut().skip(n) {
            *slot = 0;
        }
        n
    }
}
