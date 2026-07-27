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

//! Radial glow feathering and diagonal gradient fills.

use super::canvas::Sprite;
use super::prim::SS;

pub fn radial(s: &mut Sprite, cx: i32, cy: i32, r0: i32, r1: i32, rgb: u32, amax: u8) {
    let ccx = cx * SS + SS / 2;
    let ccy = cy * SS + SS / 2;
    let (r02, r12) = ((r0 * SS) * (r0 * SS), (r1 * SS) * (r1 * SS));
    for y in 0..s.h {
        for x in 0..s.w {
            let (dx, dy) = (x as i32 * SS + SS / 2 - ccx, y as i32 * SS + SS / 2 - ccy);
            let d = (dx * dx + dy * dy) as i64;
            if d <= r02 as i64 || d > r12 as i64 {
                continue;
            }
            let t = (d - r02 as i64) * 255 / (r12 - r02).max(1) as i64;
            s.set(x, y, rgb, (amax as i64 * (255 - t) / 255) as u8);
        }
    }
}

pub fn lerp(c0: u32, c1: u32, t: u32) -> u32 {
    let ch = |sh: u32| {
        let a = (c0 >> sh) & 0xFF;
        let b = (c1 >> sh) & 0xFF;
        (a * (255 - t) + b * t) / 255
    };
    (ch(16) << 16) | (ch(8) << 8) | ch(0)
}

pub fn gradient(s: &mut Sprite, c0: u32, c1: u32) {
    let max = (s.w + s.h).max(1);
    for y in 0..s.h {
        for x in 0..s.w {
            let t = (x + y) * 255 / max;
            s.set(x, y, lerp(c0, c1, t), 255);
        }
    }
}
