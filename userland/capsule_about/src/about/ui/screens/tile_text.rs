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

use crate::about::data::uptime::split_dhms;
use crate::about::format::u64_decimal;

// Fixed-capacity append. Every value a tile shows is assembled from decimal runs
// and separators, and no allocator exists here, so the caller owns the buffer and
// this only ever writes inside it.
pub fn push(dst: &mut [u8], at: usize, src: &[u8]) -> usize {
    let mut i = at;
    for byte in src {
        if i < dst.len() {
            dst[i] = *byte;
            i += 1;
        }
    }
    i
}

// Days are dropped while there are none, so a freshly booted image reads "0h 04m"
// rather than padding a field that carries no information yet.
pub fn uptime(ms: u64, dst: &mut [u8; 24]) -> &[u8] {
    let (d, h, m, _) = split_dhms(ms);
    let mut buf = [0u8; 20];
    let mut n = 0usize;
    if d > 0 {
        n = push(dst, n, u64_decimal(d, &mut buf));
        n = push(dst, n, b"d ");
    }
    n = push(dst, n, u64_decimal(h, &mut buf));
    n = push(dst, n, b"h ");
    n = push(dst, n, u64_decimal(m, &mut buf));
    n = push(dst, n, b"m");
    &dst[..n]
}

pub fn dims(w: u32, h: u32, dst: &mut [u8; 24]) -> &[u8] {
    let mut buf = [0u8; 20];
    let mut n = push(dst, 0, u64_decimal(w as u64, &mut buf));
    n = push(dst, n, b"x");
    n = push(dst, n, u64_decimal(h as u64, &mut buf));
    &dst[..n]
}

pub fn ratio(num: u64, den: u64, dst: &mut [u8; 24]) -> &[u8] {
    let mut buf = [0u8; 20];
    let mut n = push(dst, 0, u64_decimal(num, &mut buf));
    n = push(dst, n, b"/");
    n = push(dst, n, u64_decimal(den, &mut buf));
    &dst[..n]
}
