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

//! Writing numbers without an allocator.

/// Ten to the power of `n`, saturating rather than wrapping.
pub fn pow10(n: u8) -> u128 {
    let mut v = 1u128;
    let mut i = 0;
    while i < n {
        v = v.saturating_mul(10);
        i += 1;
    }
    v
}

/// Write `v` as decimal digits, most significant first.
pub fn u128_str(mut v: u128, out: &mut [u8]) -> usize {
    if v == 0 {
        out[0] = b'0';
        return 1;
    }
    let mut d = [0u8; 40];
    let mut n = 0;
    while v > 0 && n < d.len() {
        d[n] = b'0' + (v % 10) as u8;
        v /= 10;
        n += 1;
    }
    let n = n.min(out.len());
    for i in 0..n {
        out[i] = d[n - 1 - i];
    }
    n
}

/// Append `src` to `out`, truncating rather than panicking.
pub fn copy(src: &[u8], out: &mut [u8]) -> usize {
    let n = src.len().min(out.len());
    out[..n].copy_from_slice(&src[..n]);
    n
}
