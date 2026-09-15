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

//! Section 04's identity-to-hue function. The wheel is deliberately not sampled
//! whole: full-wheel hues produced muddy greens that fought the cyan chrome, so
//! ids land in a cyan band or a violet band and a wall of covers reads as one
//! collection.

const CYAN_LO: u32 = 170;
const CYAN_SPAN: u32 = 45;
const VIOLET_LO: u32 = 265;
const VIOLET_SPAN: u32 = 50;

pub fn hash(id: &str) -> u32 {
    let mut h: u32 = 0x811C_9DC5;
    for b in id.bytes() {
        h ^= b as u32;
        h = h.wrapping_mul(0x0100_0193);
    }
    h
}

pub fn hue(id: &str) -> u32 {
    let h = hash(id);
    if h & 1 == 0 {
        CYAN_LO + (h >> 8) % CYAN_SPAN
    } else {
        VIOLET_LO + (h >> 8) % VIOLET_SPAN
    }
}

pub fn motif(id: &str) -> u32 {
    (hash(id) >> 3) % 6
}

fn chan(p: u32, q: u32, mut t: i32) -> u32 {
    if t < 0 {
        t += 1000;
    }
    if t > 1000 {
        t -= 1000;
    }
    let t = t as u32;
    let v = if t < 167 {
        p + (q - p) * 6 * t / 1000
    } else if t < 500 {
        q
    } else if t < 667 {
        p + (q - p) * (667 - t) * 6 / 1000
    } else {
        p
    };
    (v * 255 / 1000).min(255)
}

pub fn hsl(h: u32, s: u32, l: u32) -> u32 {
    let (s, l) = (s.min(100) * 10, l.min(100) * 10);
    if s == 0 {
        let v = l * 255 / 1000;
        return (v << 16) | (v << 8) | v;
    }
    let q = if l < 500 { l * (1000 + s) / 1000 } else { l + s - l * s / 1000 };
    let p = 2 * l - q;
    let t = (h % 360) as i32 * 1000 / 360;
    let r = chan(p, q, t + 333);
    let g = chan(p, q, t);
    let b = chan(p, q, t - 333);
    (r << 16) | (g << 8) | b
}

pub fn stroke_rgb(h: u32) -> u32 {
    hsl(h, 92, 66)
}

pub fn ground_top(h: u32) -> u32 {
    hsl(h, 60, 16)
}

pub fn ground_bottom(h: u32) -> u32 {
    hsl(h.wrapping_sub(26), 70, 5)
}
