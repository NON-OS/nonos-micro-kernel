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

//! Hero glow disc button base and album-art gradient tile.

use super::canvas::Sprite;
use super::{fx, prim};

pub fn glow_disc(px: u32, fill_rgb: u32, glow_rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let c = (px / 2) as i32;
    let r = (px * 30 / 100) as i32;
    let edge = (px * 50 / 100) as i32;
    fx::radial(&mut s, c, c, r, edge, glow_rgb, 200);
    prim::disc(&mut s, c, c, r, fill_rgb);
    s
}

pub fn gradient_art(px: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    fx::gradient(&mut s, 0x7C5CFF, 0x0E0E16);
    s
}
