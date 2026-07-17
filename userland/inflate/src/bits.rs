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

pub struct Bits<'a> {
    d: &'a [u8],
    byte: usize,
    bit: u32,
}

impl<'a> Bits<'a> {
    pub fn new(d: &'a [u8]) -> Self {
        Bits { d, byte: 0, bit: 0 }
    }

    pub fn bit(&mut self) -> Option<u32> {
        let b = *self.d.get(self.byte)?;
        let v = (b >> self.bit) & 1;
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.byte += 1;
        }
        Some(v as u32)
    }

    pub fn bits(&mut self, n: u32) -> Option<u32> {
        let mut v = 0u32;
        for i in 0..n {
            v |= self.bit()? << i;
        }
        Some(v)
    }

    pub fn align(&mut self) {
        if self.bit != 0 {
            self.bit = 0;
            self.byte += 1;
        }
    }

    pub fn take(&mut self) -> Option<u8> {
        let b = *self.d.get(self.byte)?;
        self.byte += 1;
        Some(b)
    }
}
