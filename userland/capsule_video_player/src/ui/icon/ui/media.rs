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

use crate::ui::sprite::{cache, Glyph};
use nonos_app_skeleton::paint::PaintBuffer;

pub fn cc(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    cache::draw(fb, x, y, s, argb, Glyph::Cc);
}

pub fn pip(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    cache::draw(fb, x, y, s, argb, Glyph::Pip);
}

pub fn fullscreen(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    cache::draw(fb, x, y, s, argb, Glyph::Fullscreen);
}

pub fn clock(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, argb: u32) {
    cache::draw(fb, x, y, s, argb, Glyph::Clock);
}
