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

#![allow(dead_code)]

use alloc::vec;
use alloc::vec::Vec;

pub const MAX_OUT: usize = 4 * 1024 * 1024;

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

pub struct Huff {
    counts: [u16; 16],
    symbols: Vec<u16>,
}

pub fn build(lengths: &[u8]) -> Huff {
    let mut counts = [0u16; 16];
    for &l in lengths {
        counts[(l & 15) as usize] += 1;
    }
    counts[0] = 0;
    let mut offsets = [0u16; 16];
    for i in 1..16 {
        offsets[i] = offsets[i - 1] + counts[i - 1];
    }
    let mut symbols = vec![0u16; lengths.len()];
    for (sym, &l) in lengths.iter().enumerate() {
        if l != 0 {
            symbols[offsets[l as usize] as usize] = sym as u16;
            offsets[l as usize] += 1;
        }
    }
    Huff { counts, symbols }
}

pub fn decode(b: &mut Bits, h: &Huff) -> Option<u16> {
    let mut code = 0i32;
    let mut first = 0i32;
    let mut index = 0i32;
    for len in 1..16 {
        code |= b.bit()? as i32;
        let count = h.counts[len] as i32;
        if code - first < count {
            return h.symbols.get((index + (code - first)) as usize).copied();
        }
        index += count;
        first += count;
        first <<= 1;
        code <<= 1;
    }
    None
}

pub const LBASE: [u16; 29] = [3,4,5,6,7,8,9,10,11,13,15,17,19,23,27,31,35,43,51,59,67,83,99,115,131,163,195,227,258];
pub const LEXT: [u8; 29] = [0,0,0,0,0,0,0,0,1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5,0];
pub const DBASE: [u16; 30] = [1,2,3,4,5,7,9,13,17,25,33,49,65,97,129,193,257,385,513,769,1025,1537,2049,3073,4097,6145,8193,12289,16385,24577];
pub const DEXT: [u8; 30] = [0,0,0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,10,10,11,11,12,12,13,13];
