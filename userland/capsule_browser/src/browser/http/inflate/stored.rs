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
use super::tables::MAX_OUT;

pub fn stored(b: &mut Bits, out: &mut Vec<u8>) -> Option<()> {
    b.align();
    let lo = b.take()? as usize;
    let hi = b.take()? as usize;
    let len = lo | (hi << 8);
    let nlo = b.take()? as usize;
    let nhi = b.take()? as usize;
    let nlen = nlo | (nhi << 8);
    if len ^ nlen != 0xffff || out.len().checked_add(len)? > MAX_OUT {
        return None;
    }
    for _ in 0..len {
        out.push(b.take()?);
    }
    Some(())
}
