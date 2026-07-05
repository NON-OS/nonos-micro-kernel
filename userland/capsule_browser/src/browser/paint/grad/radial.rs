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

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use super::split::split_top;
use super::stop_list::parse_stops;
use super::stops::color_at;

// Parse radial-gradient(...) stops, ignoring the shape and position prelude,
// which we approximate as a circle centered on the box out to its corner.
pub(super) fn parse_radial(func: &str) -> Option<Vec<(u32, f32)>> {
    let inner = func.strip_prefix("radial-gradient(")?.strip_suffix(')')?;
    let mut items = split_top(inner);
    if items.is_empty() {
        return None;
    }
    // A leading prelude (circle, size, "at <pos>") holds no color, so drop it.
    if items[0].contains("at ")
        || items[0].contains("circle")
        || items[0].contains("ellipse")
        || items[0].contains("closest")
        || items[0].contains("farthest")
    {
        items.remove(0);
    }
    parse_stops(&items)
}

// Fill the box with the radial gradient, source-over, the stop position given
// by distance from the center normalized to the farthest corner.
pub(super) fn fill_radial(
    fb: &mut PaintBuffer,
    stops: &[(u32, f32)],
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) {
    if w <= 0 || h <= 0 {
        return;
    }
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let radius = (cx * cx + cy * cy).max(1.0);
    for py in 0..h {
        for px in 0..w {
            let dx = px as f32 - cx;
            let dy = py as f32 - cy;
            let t = super::sqrt::sqrt((dx * dx + dy * dy) / radius);
            super::render::put_pixel(fb, x + px, y + py, color_at(stops, t));
        }
    }
}
