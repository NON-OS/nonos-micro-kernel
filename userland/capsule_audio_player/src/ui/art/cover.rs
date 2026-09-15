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

//! Deterministic album artwork. Section 04 in one call: ground, motif, scrim.

use nonos_app_skeleton::PaintBuffer;

use super::hue::{ground_bottom, ground_top, hue, motif, stroke_rgb};
use super::{ground, motif_a, motif_b};
use crate::ui::geometry::Rect;
use crate::ui::theme::alpha;

pub fn tint(id: &str) -> u32 {
    alpha(stroke_rgb(hue(id)), 0xFF)
}

pub fn cover(fb: &mut PaintBuffer, r: Rect, id: &str, rad: u32) {
    if r.w <= 0 || r.h <= 0 {
        return;
    }
    let h = hue(id);
    ground::ground(fb, r, rad, ground_top(h), ground_bottom(h));
    let c = alpha(stroke_rgb(h), 0xE6);
    match motif(id) {
        0 => motif_a::ring(fb, r, c),
        1 => motif_a::city(fb, r, c),
        2 => motif_a::horizon(fb, r, c),
        3 => motif_b::grid(fb, r, c),
        4 => motif_b::orb(fb, r, c),
        _ => motif_b::wave(fb, r, c),
    }
}

pub fn cover_scrimmed(fb: &mut PaintBuffer, r: Rect, id: &str, rad: u32) {
    cover(fb, r, id, rad);
    ground::scrim(fb, r, rad);
}
