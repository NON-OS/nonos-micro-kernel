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

//! Repeat, shuffle and the two volume states.

use super::canvas::Sprite;
use super::shape::tri;
use super::stroke::line;
use super::unit::{blank, path, W};

pub fn repeat(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    path(&mut s, &m, &[(58, 30), (14, 30), (14, 70), (86, 70), (86, 44)], t, rgb);
    tri(&mut s, [(m(56), m(18)), (m(56), m(42)), (m(80), m(30))], rgb);
    s
}

pub fn shuffle(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    path(&mut s, &m, &[(12, 28), (34, 28), (62, 72), (74, 72)], t, rgb);
    path(&mut s, &m, &[(12, 72), (34, 72), (44, 56)], t, rgb);
    path(&mut s, &m, &[(54, 42), (62, 28), (74, 28)], t, rgb);
    tri(&mut s, [(m(70), m(16)), (m(70), m(40)), (m(92), m(28))], rgb);
    tri(&mut s, [(m(70), m(60)), (m(70), m(84)), (m(92), m(72))], rgb);
    s
}

fn cone<F: Fn(u32) -> i32>(s: &mut Sprite, m: &F, t: i32, rgb: u32) {
    path(s, m, &[(18, 40), (32, 40), (50, 22), (50, 78), (32, 60), (18, 60), (18, 40)], t, rgb);
}

pub fn volume(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    cone(&mut s, &m, t, rgb);
    line(&mut s, (m(64), m(38)), (m(64), m(62)), t, rgb);
    line(&mut s, (m(80), m(28)), (m(80), m(72)), t, rgb);
    s
}

pub fn mute(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    let t = m(W);
    cone(&mut s, &m, t, rgb);
    line(&mut s, (m(68), m(40)), (m(88), m(60)), t, rgb);
    line(&mut s, (m(88), m(40)), (m(68), m(60)), t, rgb);
    s
}
