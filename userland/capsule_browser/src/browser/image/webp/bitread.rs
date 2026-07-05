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

// LSB-first bit reader over the VP8L bitstream. Bits are consumed from the
// least significant end of each byte, low bytes first, matching the format.
pub(super) struct BitReader<'a> {
    data: &'a [u8],
    pos: usize,
    bit: u32,
    // True once a read ran past the end; every later read yields zero so a
    // truncated stream degrades instead of panicking.
    pub eos: bool,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BitReader { data, pos: 0, bit: 0, eos: false }
    }

    // Read up to 24 bits as an unsigned integer, LSB first.
    pub fn read(&mut self, count: u32) -> u32 {
        let mut out = 0u32;
        for i in 0..count {
            let byte = match self.data.get(self.pos) {
                Some(b) => *b,
                None => {
                    self.eos = true;
                    0
                }
            };
            let b = (byte as u32 >> self.bit) & 1;
            out |= b << i;
            self.bit += 1;
            if self.bit == 8 {
                self.bit = 0;
                self.pos += 1;
            }
        }
        out
    }

    pub fn read_bit(&mut self) -> u32 {
        self.read(1)
    }
}
