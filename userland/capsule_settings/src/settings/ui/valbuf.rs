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

//! A fixed on-stack string for values the panel formats at paint time. The paint
//! path runs on every frame, so it formats into this rather than allocating.

pub const CAP: usize = 72;

pub struct ValBuf {
    pub(super) bytes: [u8; CAP],
    pub(super) len: usize,
}

impl ValBuf {
    pub fn new() -> Self {
        ValBuf { bytes: [0; CAP], len: 0 }
    }

    pub fn push_str(&mut self, s: &str) {
        for b in s.as_bytes() {
            self.push(*b);
        }
    }

    pub fn push_bytes(&mut self, s: &[u8]) {
        for b in s {
            self.push(if b.is_ascii_graphic() || *b == b' ' { *b } else { b'?' });
        }
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")
    }

    pub(super) fn push(&mut self, b: u8) {
        if self.len < CAP {
            self.bytes[self.len] = b;
            self.len += 1;
        }
    }
}
