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

//! Supersampled coverage plus disc and ring fill primitives.

use super::canvas::Sprite;

pub const SS: i32 = 4;

pub fn coverage<F: Fn(i32, i32) -> bool>(x: u32, y: u32, inside: F) -> u8 {
    let mut c = 0u32;
    for sy in 0..SS {
        for sx in 0..SS {
            if inside(x as i32 * SS + sx, y as i32 * SS + sy) {
                c += 1;
            }
        }
    }
    (c * 255 / (SS * SS) as u32) as u8
}

pub fn disc(s: &mut Sprite, cx: i32, cy: i32, r: i32, rgb: u32) {
    let ccx = cx * SS + SS / 2;
    let ccy = cy * SS + SS / 2;
    let rr = (r * SS) * (r * SS);
    for y in 0..s.h {
        for x in 0..s.w {
            let a = coverage(x, y, |px, py| {
                let (dx, dy) = (px - ccx, py - ccy);
                dx * dx + dy * dy <= rr
            });
            if a > 0 {
                s.set(x, y, rgb, a);
            }
        }
    }
}

pub fn ring(s: &mut Sprite, cx: i32, cy: i32, r_out: i32, th: i32, rgb: u32) {
    let ccx = cx * SS + SS / 2;
    let ccy = cy * SS + SS / 2;
    let ro2 = (r_out * SS) * (r_out * SS);
    let ri = ((r_out - th) * SS).max(0);
    let ri2 = ri * ri;
    for y in 0..s.h {
        for x in 0..s.w {
            let a = coverage(x, y, |px, py| {
                let (dx, dy) = (px - ccx, py - ccy);
                let d = dx * dx + dy * dy;
                d <= ro2 && d >= ri2
            });
            if a > 0 {
                s.set(x, y, rgb, a);
            }
        }
    }
}
