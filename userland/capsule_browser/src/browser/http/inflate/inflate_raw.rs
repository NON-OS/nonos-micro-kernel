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
use super::dynamic::dynamic;
use super::fixed::fixed;
use super::stored::stored;
use super::tables::MAX_OUT;

pub fn inflate(src: &[u8]) -> Option<Vec<u8>> {
    let mut b = Bits::new(src);
    let mut out: Vec<u8> = Vec::new();
    loop {
        let last = b.bit()?;
        match b.bits(2)? {
            0 => stored(&mut b, &mut out)?,
            1 => fixed(&mut b, &mut out)?,
            2 => dynamic(&mut b, &mut out)?,
            _ => return None,
        }
        if out.len() > MAX_OUT {
            return None;
        }
        if last == 1 {
            return Some(out);
        }
    }
}
