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

//! Hero glow ring button base and album-art gradient tile.

use super::canvas::Sprite;
use super::{fx, prim};

pub fn glow_ring(px: u32, ring_rgb: u32, glow_rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let c = (px / 2) as i32;
    let r = (px * 30 / 100) as i32;
    let edge = (px * 50 / 100) as i32;
    fx::radial(&mut s, c, c, r, edge, glow_rgb, 115);
    fx::radial(&mut s, c, c, 0, r, glow_rgb, 72);
    prim::ring(&mut s, c, c, r, (px * 3 / 100).max(2) as i32, ring_rgb);
    s
}

pub fn gradient_art(px: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    fx::gradient(&mut s, 0x4A3F7A, 0x241D3D);
    s
}

pub fn gradient_logo(px: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    fx::gradient(&mut s, 0x8B7FF0, 0x6A5FC4);
    s
}

pub fn knob(px: u32, rgb: u32) -> Sprite {
    let mut s = Sprite::blank(px);
    let c = (px / 2) as i32;
    prim::disc(&mut s, c, c, c - 1, rgb);
    s
}
