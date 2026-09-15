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

//! Small editing marks: plus, close, check and the overflow dots.

use super::canvas::Sprite;
use super::prim::disc;
use super::stroke::line;
use super::unit::{blank, path, W};

pub fn plus(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    line(&mut s, (m(50), m(18)), (m(50), m(82)), t, rgb);
    line(&mut s, (m(18), m(50)), (m(82), m(50)), t, rgb);
    s
}

pub fn close(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    line(&mut s, (m(24), m(24)), (m(76), m(76)), t, rgb);
    line(&mut s, (m(76), m(24)), (m(24), m(76)), t, rgb);
    s
}

pub fn check(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    path(&mut s, &m, &[(20, 52), (40, 74), (82, 26)], m(W), rgb);
    s
}

pub fn dots(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let r = m(8);
    for y in [20u32, 50, 80] {
        disc(&mut s, m(50), m(y), r, rgb);
    }
    s
}
