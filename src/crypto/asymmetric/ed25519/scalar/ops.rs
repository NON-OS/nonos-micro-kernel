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

use super::reduce::sc_reduce_mod_l;

#[inline]
pub(crate) fn sc_ge(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut borrow: i32 = 0;
    for i in 0..32 {
        let diff = (a[i] as i32) - (b[i] as i32) - borrow;
        borrow = (diff >> 8) & 1;
    }
    borrow == 0
}

pub(crate) fn sc_addmul_mod_l(r: &[u8; 32], k: &[u8; 32], a: &[u8; 32]) -> [u8; 32] {
    let mut wide = [0u64; 64];
    for i in 0..32 {
        for j in 0..32 {
            wide[i + j] += (k[i] as u64) * (a[j] as u64);
        }
    }

    for i in 0..32 {
        wide[i] += r[i] as u64;
    }

    let mut out64 = [0u8; 64];
    let mut carry = 0u64;
    for i in 0..64 {
        let v = wide[i] + carry;
        out64[i] = (v & 0xFF) as u8;
        carry = v >> 8;
    }

    sc_reduce_mod_l(&mut out64)
}
