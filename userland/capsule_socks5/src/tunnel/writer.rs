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

/// A bounds-checked cursor for rendering an address into a caller buffer
/// without allocating. `dec` and `hex` write without leading zeros.
pub struct Writer<'a> {
    out: &'a mut [u8],
    pos: usize,
}

impl<'a> Writer<'a> {
    pub fn new(out: &'a mut [u8]) -> Self {
        Self { out, pos: 0 }
    }

    pub fn len(&self) -> usize {
        self.pos
    }

    pub fn byte(&mut self, b: u8) -> Option<()> {
        *self.out.get_mut(self.pos)? = b;
        self.pos += 1;
        Some(())
    }

    pub fn bytes(&mut self, bs: &[u8]) -> Option<()> {
        self.out.get_mut(self.pos..self.pos + bs.len())?.copy_from_slice(bs);
        self.pos += bs.len();
        Some(())
    }

    pub fn dec(&mut self, v: u16) -> Option<()> {
        let mut buf = [0u8; 5];
        let mut i = buf.len();
        let mut n = v;
        loop {
            i -= 1;
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        self.bytes(&buf[i..])
    }

    pub fn hex(&mut self, v: u16) -> Option<()> {
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        let mut buf = [0u8; 4];
        let mut i = buf.len();
        let mut n = v;
        loop {
            i -= 1;
            buf[i] = DIGITS[(n & 0xF) as usize];
            n >>= 4;
            if n == 0 {
                break;
            }
        }
        self.bytes(&buf[i..])
    }
}
