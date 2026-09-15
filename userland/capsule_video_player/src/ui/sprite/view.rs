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

//! Content-view marks: search, list and info.

use super::canvas::Sprite;
use super::prim::{disc, ring};
use super::stroke::line;
use super::unit::{blank, W};

pub fn search(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    ring(&mut s, m(42), m(42), m(30), t, rgb);
    line(&mut s, (m(64), m(64)), (m(84), m(84)), t, rgb);
    s
}

pub fn list(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    for y in [22u32, 50, 78] {
        disc(&mut s, m(16), m(y), m(7), rgb);
        line(&mut s, (m(36), m(y)), (m(86), m(y)), t, rgb);
    }
    s
}

pub fn info(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    ring(&mut s, m(50), m(50), m(38), t, rgb);
    disc(&mut s, m(50), m(30), m(6), rgb);
    line(&mut s, (m(50), m(46)), (m(50), m(72)), t, rgb);
    s
}
