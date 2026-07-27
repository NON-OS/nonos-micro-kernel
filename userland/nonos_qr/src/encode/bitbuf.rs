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

use alloc::vec::Vec;

/// Most-significant-bit-first bit accumulator for the data stream.
pub(super) struct BitBuf {
    bytes: Vec<u8>,
    nbits: usize,
}

impl BitBuf {
    pub(super) fn new() -> Self {
        BitBuf { bytes: Vec::new(), nbits: 0 }
    }
    pub(super) fn len(&self) -> usize {
        self.nbits
    }
    pub(super) fn push(&mut self, value: u32, bits: u32) {
        for i in (0..bits).rev() {
            let bit = ((value >> i) & 1) as u8;
            let byte = self.nbits / 8;
            if byte == self.bytes.len() {
                self.bytes.push(0);
            }
            self.bytes[byte] |= bit << (7 - (self.nbits % 8));
            self.nbits += 1;
        }
    }
    pub(super) fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}
