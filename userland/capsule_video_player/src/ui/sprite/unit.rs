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

//! Shared 100-unit grid helper and the box-stroke used by framed glyphs.

use super::canvas::Sprite;
use super::stroke;

pub const W: u32 = 10;

pub fn blank(px: u32) -> (Sprite, impl Fn(u32) -> i32) {
    (Sprite::blank(px), move |p: u32| (px * p / 100) as i32)
}

pub fn frame<F: Fn(u32) -> i32>(s: &mut Sprite, m: &F, r: [u32; 4], t: i32, rgb: u32) {
    let (x0, y0, x1, y1) = (m(r[0]), m(r[1]), m(r[2]), m(r[3]));
    stroke::line(s, (x0, y0), (x1, y0), t, rgb);
    stroke::line(s, (x1, y0), (x1, y1), t, rgb);
    stroke::line(s, (x1, y1), (x0, y1), t, rgb);
    stroke::line(s, (x0, y1), (x0, y0), t, rgb);
}

pub fn path<F: Fn(u32) -> i32>(s: &mut Sprite, m: &F, p: &[(u32, u32)], t: i32, rgb: u32) {
    for w in p.windows(2) {
        stroke::line(s, (m(w[0].0), m(w[0].1)), (m(w[1].0), m(w[1].1)), t, rgb);
    }
}
