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

use super::bits::Bits;
use super::codes::codes;
use super::huff::{build, decode};
use super::tables::{MAX_OUT, ORDER};

pub fn dynamic(b: &mut Bits, out: &mut Vec<u8>) -> Option<()> {
    let hlit = b.bits(5)? as usize + 257;
    let hdist = b.bits(5)? as usize + 1;
    let hclen = b.bits(4)? as usize + 4;
    if hlit > 286 || hdist > 30 {
        return None;
    }
    let mut cl = [0u8; 19];
    for i in 0..hclen {
        cl[ORDER[i]] = b.bits(3)? as u8;
    }
    let mut lengths: Vec<u8> = Vec::with_capacity(hlit + hdist);
    let clh = build(&cl);
    while lengths.len() < hlit + hdist {
        let sym = decode(b, &clh)?;
        push_lengths(sym, b, &mut lengths, hlit + hdist)?;
    }
    let lit = build(&lengths[..hlit]);
    let dist = build(&lengths[hlit..hlit + hdist]);
    codes(b, out, &lit, &dist)
}

fn push_lengths(sym: u16, b: &mut Bits, lengths: &mut Vec<u8>, limit: usize) -> Option<()> {
    match sym {
        0..=15 => lengths.push(sym as u8),
        16 => {
            let prev = *lengths.last()?;
            let n = 3 + b.bits(2)? as usize;
            if lengths.len().checked_add(n)? > limit || lengths.len().checked_add(n)? > MAX_OUT {
                return None;
            }
            for _ in 0..n {
                lengths.push(prev);
            }
        }
        17 | 18 => push_zeroes(sym, b, lengths, limit)?,
        _ => return None,
    }
    (lengths.len() <= limit).then_some(())
}

fn push_zeroes(sym: u16, b: &mut Bits, lengths: &mut Vec<u8>, limit: usize) -> Option<()> {
    let n = if sym == 17 { 3 + b.bits(3)? as usize } else { 11 + b.bits(7)? as usize };
    if lengths.len().checked_add(n)? > limit {
        return None;
    }
    for _ in 0..n {
        lengths.push(0);
    }
    Some(())
}
