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

//! Navigation glyphs: home, grid, gear, chevron, download, heart.

use super::canvas::Sprite;
use super::{prim, shape, stroke};

fn blank(px: u32) -> (Sprite, impl Fn(u32) -> i32) {
    (Sprite::blank(px), move |p: u32| (px * p / 100) as i32)
}

pub fn home(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    shape::tri(&mut s, [(m(12), m(48)), (m(50), m(14)), (m(88), m(48))], rgb);
    shape::rrect(&mut s, m(24), m(46), m(52), m(40), m(6), rgb);
    s
}

pub fn grid(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    for (cx, cy) in [(24u32, 24u32), (24, 62), (62, 24), (62, 62)] {
        shape::rrect(&mut s, m(cx), m(cy), m(26), m(26), m(6), rgb);
    }
    s
}

pub fn gear(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    prim::ring(&mut s, m(50), m(50), m(34), m(11), rgb);
    prim::ring(&mut s, m(50), m(50), m(14), m(7), rgb);
    for (a, b) in [(50u32, 8u32), (50, 92), (8, 50), (92, 50)] {
        stroke::line(&mut s, (m(50), m(50)), (m(a), m(b)), m(11), rgb);
    }
    s
}

pub fn chevron(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    stroke::line(&mut s, (m(38), m(22)), (m(64), m(50)), m(10), rgb);
    stroke::line(&mut s, (m(64), m(50)), (m(38), m(78)), m(10), rgb);
    s
}

pub fn download(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    stroke::line(&mut s, (m(50), m(14)), (m(50), m(60)), m(10), rgb);
    stroke::line(&mut s, (m(30), m(42)), (m(50), m(62)), m(10), rgb);
    stroke::line(&mut s, (m(70), m(42)), (m(50), m(62)), m(10), rgb);
    stroke::line(&mut s, (m(20), m(82)), (m(80), m(82)), m(10), rgb);
    s
}

pub fn heart(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    prim::disc(&mut s, m(34), m(38), m(19), rgb);
    prim::disc(&mut s, m(66), m(38), m(19), rgb);
    shape::tri(&mut s, [(m(15), m(45)), (m(85), m(45)), (m(50), m(86))], rgb);
    s
}

pub fn compass(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    prim::ring(&mut s, m(50), m(50), m(40), m(9), rgb);
    shape::tri(&mut s, [(m(66), m(34)), (m(56), m(56)), (m(34), m(66))], rgb);
    prim::disc(&mut s, m(50), m(50), m(7), rgb);
    s
}

pub fn radio(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    prim::disc(&mut s, m(50), m(50), m(11), rgb);
    prim::ring(&mut s, m(50), m(50), m(26), m(8), rgb);
    prim::ring(&mut s, m(50), m(50), m(44), m(8), rgb);
    s
}

pub fn bell(px: u32, rgb: u32) -> Sprite {
    let (mut s, m) = blank(px);
    prim::disc(&mut s, m(50), m(44), m(26), rgb);
    shape::rrect(&mut s, m(24), m(44), m(52), m(26), m(6), rgb);
    shape::rrect(&mut s, m(16), m(66), m(68), m(10), m(5), rgb);
    shape::rrect(&mut s, m(42), m(78), m(16), m(12), m(6), rgb);
    s
}
