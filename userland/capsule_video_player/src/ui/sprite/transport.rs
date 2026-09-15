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

//! Play, pause and seek marks for the transport bar.

use super::canvas::Sprite;
use super::shape::{rrect, tri};
use super::stroke::line;
use super::unit::{blank, path, W};

pub fn play(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    tri(&mut s, [(m(36), m(20)), (m(36), m(80)), (m(80), m(50))], rgb);
    s
}

pub fn pause(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let (w, h, r) = (m(W), m(56), m(5));
    rrect(&mut s, m(36), m(22), w, h, r, rgb);
    rrect(&mut s, m(58), m(22), w, h, r, rgb);
    s
}

pub fn prev(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    tri(&mut s, [(m(78), m(22)), (m(78), m(78)), (m(36), m(50))], rgb);
    line(&mut s, (m(26), m(22)), (m(26), m(78)), m(W), rgb);
    s
}

pub fn next(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    tri(&mut s, [(m(22), m(22)), (m(22), m(78)), (m(64), m(50))], rgb);
    line(&mut s, (m(74), m(22)), (m(74), m(78)), m(W), rgb);
    s
}

pub fn rewind(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    path(&mut s, &m, &[(52, 22), (28, 50), (52, 78)], t, rgb);
    path(&mut s, &m, &[(80, 22), (56, 50), (80, 78)], t, rgb);
    s
}
