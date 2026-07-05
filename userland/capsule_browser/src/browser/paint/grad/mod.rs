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

// CSS gradient painting. Linear gradients parse to an angle and color stops
// and fill their box with source-over compositing. Radial and conic
// gradients are not drawn yet, so a box keeps its color rather than guessing.

mod color;
mod parse;
mod render;
mod split;
mod stops;
mod trig;

use nonos_app_skeleton::PaintBuffer;

// True when the background value is a gradient function this module owns.
pub(super) fn is_gradient(src: &str) -> bool {
    src.starts_with("linear-gradient(") || src.starts_with("radial-gradient(")
}

// Paint a gradient background into the box; returns false when the value is
// not a gradient we render, so the caller can fall back.
pub(super) fn paint_gradient(
    fb: &mut PaintBuffer,
    src: &str,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> bool {
    if let Some(g) = parse::parse_linear(src) {
        render::fill_linear(fb, &g, x, y, w, h);
        return true;
    }
    false
}
