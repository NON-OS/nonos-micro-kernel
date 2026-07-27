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

//! Stroke transport glyphs: shuffle and repeat.

use super::canvas::Sprite;
use super::stroke;

pub fn shuffle(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    let t = m(9);
    stroke::line(&mut s, (m(22), m(26)), (m(78), m(74)), t, rgb);
    stroke::line(&mut s, (m(22), m(74)), (m(78), m(26)), t, rgb);
    s
}

pub fn repeat(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    let t = m(9);
    stroke::line(&mut s, (m(28), m(34)), (m(72), m(34)), t, rgb);
    stroke::line(&mut s, (m(72), m(34)), (m(72), m(66)), t, rgb);
    stroke::line(&mut s, (m(72), m(66)), (m(28), m(66)), t, rgb);
    stroke::line(&mut s, (m(28), m(66)), (m(28), m(34)), t, rgb);
    s
}
