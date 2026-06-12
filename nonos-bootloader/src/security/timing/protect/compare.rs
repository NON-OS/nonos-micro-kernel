// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::jitter::add_random_delay;

#[inline(never)]
pub fn constant_time_select(condition: bool, a: u64, b: u64) -> u64 {
    let mask = if condition { u64::MAX } else { 0 };
    (a & mask) | (b & !mask)
}

#[inline(never)]
pub fn constant_time_is_zero(value: u64) -> bool {
    let v = value | value.wrapping_neg();
    (v >> 63) == 0
}

#[inline(never)]
pub fn constant_time_eq_u8(a: u8, b: u8) -> bool {
    constant_time_is_zero((a ^ b) as u64)
}

#[inline(never)]
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        add_random_delay();
        return false;
    }
    let mut diff = 0u8;
    for i in 0..a.len() {
        diff |= a[i] ^ b[i];
    }
    add_random_delay();
    diff == 0
}
