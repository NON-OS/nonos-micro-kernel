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

use super::bitbuf::BitBuf;
use crate::reed_solomon::ec_codewords;
use crate::version::{blocks, Ecc};

/// The full interleaved codeword stream (data then EC) for `data` at
/// (version, ecc), padded and block-interleaved per ISO/IEC 18004 clause 8.6.
pub(crate) fn message_codewords(data: &[u8], version: u8, ecc: Ecc) -> Vec<u8> {
    let bl = blocks(version, ecc);
    let cap = bl.total_data_codewords();

    let mut bits = BitBuf::new();
    bits.push(0b0100, 4);
    let count_bits = if version <= 9 { 8 } else { 16 };
    bits.push(data.len() as u32, count_bits);
    for &b in data {
        bits.push(b as u32, 8);
    }
    let cap_bits = cap * 8;
    let term = core::cmp::min(4, cap_bits.saturating_sub(bits.len()));
    bits.push(0, term as u32);
    while bits.len() % 8 != 0 {
        bits.push(0, 1);
    }
    let mut bytes = bits.into_bytes();
    let pads = [0xECu8, 0x11u8];
    let mut i = 0;
    while bytes.len() < cap {
        bytes.push(pads[i % 2]);
        i += 1;
    }

    let mut data_blocks: Vec<&[u8]> = Vec::with_capacity(bl.total_blocks());
    let mut ec_blocks: Vec<Vec<u8>> = Vec::with_capacity(bl.total_blocks());
    let mut off = 0usize;
    for _ in 0..bl.g1_blocks {
        let b = &bytes[off..off + bl.g1_data as usize];
        off += bl.g1_data as usize;
        ec_blocks.push(ec_codewords(b, bl.ec_per_block as usize));
        data_blocks.push(b);
    }
    for _ in 0..bl.g2_blocks {
        let b = &bytes[off..off + bl.g2_data as usize];
        off += bl.g2_data as usize;
        ec_blocks.push(ec_codewords(b, bl.ec_per_block as usize));
        data_blocks.push(b);
    }

    let mut out = Vec::with_capacity(cap + bl.total_blocks() * bl.ec_per_block as usize);
    let max_data = bl.g1_data.max(bl.g2_data) as usize;
    for c in 0..max_data {
        for b in &data_blocks {
            if c < b.len() {
                out.push(b[c]);
            }
        }
    }
    for c in 0..bl.ec_per_block as usize {
        for e in &ec_blocks {
            out.push(e[c]);
        }
    }
    out
}
