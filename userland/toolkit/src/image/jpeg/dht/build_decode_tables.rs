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
use super::huffman_table::HuffmanTable;

pub fn build_decode_tables(t: &mut HuffmanTable) {
    let mut huffsize = [0u8; 257];
    let mut huffcode = [0u32; 257];
    let mut k = 0usize;
    let mut i = 1usize;
    while i <= 16 {
        let mut j = 1u8;
        while j <= t.bits[i] {
            huffsize[k] = i as u8;
            k += 1;
            j += 1;
        }
        i += 1;
    }
    huffsize[k] = 0;
    let mut code: u32 = 0;
    let mut si: u8 = huffsize[0];
    let mut k2 = 0usize;
    while huffsize[k2] != 0 {
        while huffsize[k2] == si {
            huffcode[k2] = code;
            code += 1;
            k2 += 1;
        }
        if huffsize[k2] == 0 {
            break;
        }
        while huffsize[k2] != si {
            code <<= 1;
            si += 1;
        }
    }
    let mut j = 0usize;
    let mut l = 1usize;
    while l <= 16 {
        if t.bits[l] == 0 {
            t.maxcode[l] = -1;
        } else {
            t.valptr[l] = j as i32;
            t.mincode[l] = huffcode[j] as i32;
            j += t.bits[l] as usize;
            t.maxcode[l] = huffcode[j - 1] as i32;
        }
        l += 1;
    }
    t.maxcode[17] = 0xFFFFF;
    t.total = k;
}
