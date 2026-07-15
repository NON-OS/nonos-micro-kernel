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

// A bounded line builder for census rows. Fixed storage keeps the census off
// the heap and out of any fallible allocation path during early boot.
pub(super) struct LineBuf {
    data: [u8; 100],
    len: usize,
}

impl LineBuf {
    pub(super) fn new() -> Self {
        Self { data: [0u8; 100], len: 0 }
    }

    pub(super) fn put(&mut self, text: &str) {
        for &b in text.as_bytes() {
            if self.len < self.data.len() {
                self.data[self.len] = b;
                self.len += 1;
            }
        }
    }

    pub(super) fn hex(&mut self, value: u64, digits: usize) {
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        let mut i = digits;
        while i > 0 {
            i -= 1;
            let nibble = (value >> (i * 4)) & 0xf;
            if self.len < self.data.len() {
                self.data[self.len] = DIGITS[nibble as usize];
                self.len += 1;
            }
        }
    }

    pub(super) fn dec(&mut self, value: u32) {
        let mut scratch = [0u8; 10];
        let mut n = value;
        let mut i = scratch.len();
        loop {
            i -= 1;
            scratch[i] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        for &b in &scratch[i..] {
            if self.len < self.data.len() {
                self.data[self.len] = b;
                self.len += 1;
            }
        }
    }

    pub(super) fn as_str(&self) -> &str {
        core::str::from_utf8(&self.data[..self.len]).unwrap_or("")
    }
}
