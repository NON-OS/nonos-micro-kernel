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

//! Small glyphs: plus, check, close.

use super::canvas::Sprite;
use super::stroke;

pub fn plus(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    let t = m(12);
    stroke::line(&mut s, (m(20), m(50)), (m(80), m(50)), t, rgb);
    stroke::line(&mut s, (m(50), m(20)), (m(50), m(80)), t, rgb);
    s
}

pub fn check(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    let t = m(12);
    stroke::line(&mut s, (m(22), m(52)), (m(42), m(72)), t, rgb);
    stroke::line(&mut s, (m(42), m(72)), (m(78), m(28)), t, rgb);
    s
}

pub fn close(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    let t = m(12);
    stroke::line(&mut s, (m(24), m(24)), (m(76), m(76)), t, rgb);
    stroke::line(&mut s, (m(24), m(76)), (m(76), m(24)), t, rgb);
    s
}
