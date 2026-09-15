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

//! Player overlay marks: captions, elapsed clock, fullscreen and picture-in-picture.

use super::canvas::Sprite;
use super::prim::ring;
use super::stroke::line;
use super::unit::{blank, frame, path, W};

pub fn cc(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    frame(&mut s, &m, [12, 26, 88, 74], t, rgb);
    line(&mut s, (m(30), m(50)), (m(44), m(50)), t, rgb);
    line(&mut s, (m(56), m(50)), (m(70), m(50)), t, rgb);
    s
}

pub fn clock(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    ring(&mut s, m(50), m(50), m(38), t, rgb);
    path(&mut s, &m, &[(50, 26), (50, 52), (68, 62)], t, rgb);
    s
}

pub fn fullscreen(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    path(&mut s, &m, &[(14, 36), (14, 14), (36, 14)], t, rgb);
    path(&mut s, &m, &[(64, 14), (86, 14), (86, 36)], t, rgb);
    path(&mut s, &m, &[(86, 64), (86, 86), (64, 86)], t, rgb);
    path(&mut s, &m, &[(36, 86), (14, 86), (14, 64)], t, rgb);
    s
}

pub fn pip(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    frame(&mut s, &m, [10, 20, 90, 80], t, rgb);
    frame(&mut s, &m, [48, 44, 78, 68], t, rgb);
    s
}
