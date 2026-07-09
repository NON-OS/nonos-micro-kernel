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

#[inline]
pub(crate) fn ct_eq_bool(a: &[u8; 32], b: &[u8; 32]) -> u8 {
    let mut diff = 0u8;
    for i in 0..32 {
        diff |= a[i] ^ b[i];
    }
    ((diff as u16 | (diff as u16).wrapping_neg()) >> 8) as u8 ^ 1
}

#[inline]
pub(crate) fn ct_is_all_zero(data: &[u8; 32]) -> u8 {
    let mut acc = 0u8;
    for &b in data {
        acc |= b;
    }
    // 1 if every byte is zero, else 0, branch-free. `acc | acc.wrapping_neg()`
    // has its high bit set for any nonzero `acc`, so the shift yields the
    // nonzero flag; subtracting from 1 gives the all-zero flag. Returning a full
    // 0x00/0xFF mask here would underflow the callers' `1 - ct_is_all_zero(..)`.
    1 - ((acc | acc.wrapping_neg()) >> 7)
}
