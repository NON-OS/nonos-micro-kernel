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
use crate::image::types::DecodeError;

use super::build_decode_tables::build_decode_tables;
use super::constants::MAX_HT;
use super::huffman_table::HuffmanTable;

pub fn parse_dht(
    seg: &[u8],
    dc_tables: &mut [HuffmanTable; MAX_HT],
    ac_tables: &mut [HuffmanTable; MAX_HT],
) -> Result<(), DecodeError> {
    let mut p = 0usize;
    while p < seg.len() {
        let tc_th = seg[p];
        p += 1;
        let tc = (tc_th >> 4) & 0x0F;
        let th = (tc_th & 0x0F) as usize;
        if th >= MAX_HT || tc > 1 {
            return Err(DecodeError::Unsupported);
        }
        if p + 16 > seg.len() {
            return Err(DecodeError::Truncated);
        }
        let mut t = HuffmanTable::new();
        let mut count = 0usize;
        let mut i = 1usize;
        while i <= 16 {
            t.bits[i] = seg[p + i - 1];
            count += t.bits[i] as usize;
            i += 1;
        }
        p += 16;
        if count > 256 || p + count > seg.len() {
            return Err(DecodeError::Truncated);
        }
        let mut i = 0usize;
        while i < count {
            t.huffval[i] = seg[p + i];
            i += 1;
        }
        p += count;
        t.present = true;
        build_decode_tables(&mut t);
        if tc == 0 {
            dc_tables[th] = t;
        } else {
            ac_tables[th] = t;
        }
    }
    Ok(())
}
