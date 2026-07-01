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
use super::huff::{decode, Huff};
use super::tables::{DBASE, DEXT, LBASE, LEXT, MAX_OUT};

pub fn codes(b: &mut Bits, out: &mut Vec<u8>, lit: &Huff, dist: &Huff) -> Option<()> {
    loop {
        let sym = decode(b, lit)?;
        if sym == 256 {
            return Some(());
        }
        if sym < 256 {
            out.push(sym as u8);
        } else {
            copy_match(b, out, sym, dist)?;
        }
        if out.len() > MAX_OUT {
            return None;
        }
    }
}

fn copy_match(b: &mut Bits, out: &mut Vec<u8>, sym: u16, dist: &Huff) -> Option<()> {
    let s = (sym - 257) as usize;
    if s >= 29 {
        return None;
    }
    let len = LBASE[s] as usize + b.bits(LEXT[s] as u32)? as usize;
    let dsym = decode(b, dist)? as usize;
    if dsym >= 30 {
        return None;
    }
    let dist_v = DBASE[dsym] as usize + b.bits(DEXT[dsym] as u32)? as usize;
    if dist_v == 0 || dist_v > out.len() || out.len().checked_add(len)? > MAX_OUT {
        return None;
    }
    let start = out.len() - dist_v;
    for i in 0..len {
        out.push(out[start + i]);
    }
    Some(())
}
