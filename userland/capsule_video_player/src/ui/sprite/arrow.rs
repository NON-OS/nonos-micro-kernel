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

//! Back arrow and the three navigation chevrons.

use super::canvas::Sprite;
use super::stroke::line;
use super::unit::{blank, path, W};

pub fn back(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    line(&mut s, (m(18), m(50)), (m(84), m(50)), t, rgb);
    path(&mut s, &m, &[(44, 26), (18, 50), (44, 74)], t, rgb);
    s
}

pub fn chevron_down(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    path(&mut s, &m, &[(22, 36), (50, 66), (78, 36)], m(W), rgb);
    s
}

pub fn chevron_left(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    path(&mut s, &m, &[(64, 22), (36, 50), (64, 78)], m(W), rgb);
    s
}

pub fn chevron_right(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    path(&mut s, &m, &[(36, 22), (64, 50), (36, 78)], m(W), rgb);
    s
}
