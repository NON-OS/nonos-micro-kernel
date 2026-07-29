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

use core::sync::atomic::{AtomicUsize, Ordering};

use super::constants::RX_BUFFER_SIZE;

pub struct RxBuffer {
    buffer: [u8; RX_BUFFER_SIZE],
    read_pos: AtomicUsize,
    write_pos: AtomicUsize,
}

impl RxBuffer {
    pub const fn new() -> Self {
        Self {
            buffer: [0; RX_BUFFER_SIZE],
            read_pos: AtomicUsize::new(0),
            write_pos: AtomicUsize::new(0),
        }
    }

    pub fn push(&mut self, byte: u8) -> bool {
        let write = self.write_pos.load(Ordering::Relaxed);
        let next_write = (write + 1) % RX_BUFFER_SIZE;

        if next_write == self.read_pos.load(Ordering::Acquire) {
            return false;
        }

        self.buffer[write] = byte;
        self.write_pos.store(next_write, Ordering::Release);
        true
    }

    pub fn pop(&self) -> Option<u8> {
        let read = self.read_pos.load(Ordering::Relaxed);
        let write = self.write_pos.load(Ordering::Acquire);

        if read == write {
            return None;
        }

        let byte = self.buffer[read];
        self.read_pos.store((read + 1) % RX_BUFFER_SIZE, Ordering::Release);
        Some(byte)
    }

    pub fn is_empty(&self) -> bool {
        self.read_pos.load(Ordering::Acquire) == self.write_pos.load(Ordering::Acquire)
    }

    pub fn is_full(&self) -> bool {
        let write = self.write_pos.load(Ordering::Acquire);
        let next_write = (write + 1) % RX_BUFFER_SIZE;
        next_write == self.read_pos.load(Ordering::Acquire)
    }

    pub fn available(&self) -> usize {
        let read = self.read_pos.load(Ordering::Acquire);
        let write = self.write_pos.load(Ordering::Acquire);

        if write >= read {
            write - read
        } else {
            RX_BUFFER_SIZE - read + write
        }
    }

    pub fn capacity(&self) -> usize {
        RX_BUFFER_SIZE - 1
    }

    pub fn clear(&mut self) {
        self.read_pos.store(0, Ordering::Release);
        self.write_pos.store(0, Ordering::Release);
    }

    pub fn peek(&self) -> Option<u8> {
        let read = self.read_pos.load(Ordering::Relaxed);
        let write = self.write_pos.load(Ordering::Acquire);

        if read == write {
            None
        } else {
            Some(self.buffer[read])
        }
    }
}

impl Default for RxBuffer {
    fn default() -> Self {
        Self::new()
    }
}
