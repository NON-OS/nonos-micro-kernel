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

//! Small glyphs: speaker, note, magnifier.

use super::canvas::Sprite;
use super::{prim, shape, stroke};

pub fn speaker(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    let cy = m(50);
    shape::rrect(&mut s, m(18), m(40), m(14), m(20), m(3), rgb);
    shape::tri(&mut s, [(m(18), cy), (m(42), m(22)), (m(42), m(78))], rgb);
    prim::ring(&mut s, m(50), cy, m(30), m(6), rgb);
    s
}

pub fn note(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    prim::disc(&mut s, m(38), m(72), m(13), rgb);
    stroke::line(&mut s, (m(49), m(70)), (m(49), m(24)), m(7), rgb);
    stroke::line(&mut s, (m(49), m(24)), (m(72), m(34)), m(7), rgb);
    s
}

pub fn magnifier(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    prim::ring(&mut s, m(42), m(42), m(24), m(8), rgb);
    stroke::line(&mut s, (m(58), m(58)), (m(80), m(80)), m(9), rgb);
    s
}
