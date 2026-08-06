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

//! LSB-first bit reader over a byte slice, as DEFLATE specifies.

use super::error::InflateError;

pub(super) struct BitReader<'a> {
    pub(super) data: &'a [u8],
    pub(super) byte: usize,
    bit: u32,
}

impl<'a> BitReader<'a> {
    pub(super) fn new(data: &'a [u8]) -> BitReader<'a> {
        BitReader { data, byte: 0, bit: 0 }
    }

    /// Read one bit, LSB first, advancing the cursor.
    pub(super) fn bit(&mut self) -> Result<u32, InflateError> {
        if self.byte >= self.data.len() {
            return Err(InflateError::Truncated);
        }
        let v = (self.data[self.byte] >> self.bit) & 1;
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.byte += 1;
        }
        Ok(v as u32)
    }

    /// Read `n` bits, first bit read is the least significant.
    pub(super) fn bits(&mut self, n: u32) -> Result<u32, InflateError> {
        let mut v = 0u32;
        for i in 0..n {
            v |= self.bit()? << i;
        }
        Ok(v)
    }

    /// Drop any partial bits, advancing to the next byte boundary, where a
    /// stored block's length and the stream trailer live.
    pub(super) fn align(&mut self) {
        if self.bit != 0 {
            self.bit = 0;
            self.byte += 1;
        }
    }

    /// The rest of the input from the next byte boundary.
    pub(super) fn byte_aligned_tail(&mut self) -> &'a [u8] {
        self.align();
        &self.data[core::cmp::min(self.byte, self.data.len())..]
    }
}
