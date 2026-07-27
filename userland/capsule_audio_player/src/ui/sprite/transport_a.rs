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

//! Solid transport glyphs: play, pause, stop, prev, next.

use super::canvas::Sprite;
use super::shape;

pub fn play(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    shape::tri(&mut s, [(m(32), m(22)), (m(32), m(78)), (m(76), m(50))], rgb);
    s
}

pub fn pause(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    shape::rrect(&mut s, m(30), m(24), m(14), m(52), m(4), rgb);
    shape::rrect(&mut s, m(56), m(24), m(14), m(52), m(4), rgb);
    s
}

pub fn stop(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    shape::rrect(&mut s, m(26), m(26), m(48), m(48), m(6), rgb);
    s
}

pub fn prev(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    shape::rrect(&mut s, m(24), m(26), m(10), m(48), m(3), rgb);
    shape::tri(&mut s, [(m(74), m(26)), (m(74), m(74)), (m(40), m(50))], rgb);
    s
}

pub fn next(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let m = |p: u32| (px * p / 100) as i32;
    shape::rrect(&mut s, m(66), m(26), m(10), m(48), m(3), rgb);
    shape::tri(&mut s, [(m(26), m(26)), (m(26), m(74)), (m(60), m(50))], rgb);
    s
}
