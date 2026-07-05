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

use alloc::vec;
use alloc::vec::Vec;

use super::bitread::BitReader;
use super::huffman::Huffman;

// Decode per-symbol code lengths using the code-length-code tree. Codes 0..15
// are literal lengths; 16 repeats the previous length, 17 and 18 repeat zero,
// each with extra bits. An optional max-symbol bound stops the run early.
pub(super) fn read_code_lengths(
    br: &mut BitReader,
    cl_code: &Huffman,
    alphabet: usize,
) -> Option<Vec<u8>> {
    let mut max_symbol = alphabet;
    if br.read_bit() == 1 {
        let nbits = 2 + 2 * br.read(3) as usize;
        max_symbol = 2 + br.read(nbits as u32) as usize;
    }
    let mut lengths = vec![0u8; alphabet];
    let mut prev = 8u8;
    let mut i = 0usize;
    let mut remaining = max_symbol;
    while i < alphabet && remaining > 0 {
        remaining -= 1;
        let sym = cl_code.read(br);
        if br.eos {
            return None;
        }
        match sym {
            0..=15 => {
                lengths[i] = sym as u8;
                if sym != 0 {
                    prev = sym as u8;
                }
                i += 1;
            }
            16 => {
                let n = 3 + br.read(2) as usize;
                for _ in 0..n {
                    if i >= alphabet {
                        break;
                    }
                    lengths[i] = prev;
                    i += 1;
                }
            }
            17 => i += (3 + br.read(3) as usize).min(alphabet - i),
            18 => i += (11 + br.read(7) as usize).min(alphabet - i),
            _ => return None,
        }
    }
    Some(lengths)
}
