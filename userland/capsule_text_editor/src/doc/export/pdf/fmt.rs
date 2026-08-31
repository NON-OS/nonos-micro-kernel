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

pub fn push_usize(out: &mut Vec<u8>, v: usize) {
    if v == 0 {
        out.push(b'0');
        return;
    }
    let mut buf = [0u8; 20];
    let mut n = 0;
    let mut x = v;
    while x > 0 {
        buf[n] = b'0' + (x % 10) as u8;
        x /= 10;
        n += 1;
    }
    while n > 0 {
        n -= 1;
        out.push(buf[n]);
    }
}

pub fn push_f32(out: &mut Vec<u8>, v: f32) {
    let neg = v < 0.0;
    let mag = if neg { -v } else { v };
    let scaled = (mag * 100.0 + 0.5) as u64;
    if neg && scaled != 0 {
        out.push(b'-');
    }
    push_usize(out, (scaled / 100) as usize);
    let frac = (scaled % 100) as u8;
    if frac != 0 {
        out.push(b'.');
        out.push(b'0' + frac / 10);
        if frac % 10 != 0 {
            out.push(b'0' + frac % 10);
        }
    }
}

pub fn push_offset(out: &mut Vec<u8>, v: usize) {
    let mut buf = [b'0'; 10];
    let mut x = v;
    let mut i = 10;
    while i > 0 {
        i -= 1;
        buf[i] = b'0' + (x % 10) as u8;
        x /= 10;
    }
    out.extend_from_slice(&buf);
}
