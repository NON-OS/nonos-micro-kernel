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

fn ch(p: u32, s: u32) -> i32 {
    ((p >> s) & 0xff) as i32
}

fn avg2(a: u32, b: u32) -> u32 {
    let mut o = 0u32;
    for s in [0, 8, 16, 24] {
        o |= (((ch(a, s) + ch(b, s)) >> 1) as u32) << s;
    }
    o
}

pub(super) fn average2(a: u32, b: u32) -> u32 {
    avg2(a, b)
}

// Clamp a + b - c per channel to a byte, the full add-subtract predictor.
pub(super) fn clamp_add_sub_full(a: u32, b: u32, c: u32) -> u32 {
    let mut o = 0u32;
    for s in [0, 8, 16, 24] {
        let v = (ch(a, s) + ch(b, s) - ch(c, s)).clamp(0, 255) as u32;
        o |= v << s;
    }
    o
}

// Clamp a + (a - b) / 2 per channel, the half add-subtract predictor.
pub(super) fn clamp_add_sub_half(a: u32, b: u32) -> u32 {
    let mut o = 0u32;
    for s in [0, 8, 16, 24] {
        let (x, y) = (ch(a, s), ch(b, s));
        o |= ((x + (x - y) / 2).clamp(0, 255) as u32) << s;
    }
    o
}

// Select L or T by which keeps the gradient L + T - TL closer, per the spec.
pub(super) fn select(l: u32, t: u32, tl: u32) -> u32 {
    let mut pa = 0i32;
    let mut pb = 0i32;
    for s in [0, 8, 16, 24] {
        let p = ch(l, s) + ch(t, s) - ch(tl, s);
        pa += (p - ch(t, s)).abs();
        pb += (p - ch(l, s)).abs();
    }
    if pa <= pb {
        l
    } else {
        t
    }
}
