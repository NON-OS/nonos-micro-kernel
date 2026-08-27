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

use nonos_app_skeleton::PaintBuffer;

use crate::snake::theme::{HALO_FAR, HALO_MID, HALO_NEAR, SCRIM};
use crate::snake::ui::rect::Rect;

const RINGS: [(u32, u32); 3] = [(1, HALO_NEAR), (3, HALO_MID), (5, HALO_FAR)];

// The compositor has no blur pass, so a halo is spelled out one ring at a
// time: three concentric strokes at falling alpha around the same shape.
pub fn bloom(fb: &mut PaintBuffer, r: Rect, radius: u32) {
    for (spread, argb) in RINGS {
        let x = r.0.saturating_sub(spread);
        let y = r.1.saturating_sub(spread);
        fb.stroke_round(x, y, r.2 + spread * 2, r.3 + spread * 2, radius + spread, 1, argb);
    }
}

// Everything under a modal is already painted, so the dim has to blend.
pub fn scrim(fb: &mut PaintBuffer) {
    fb.blend_rect(0, 0, fb.width, fb.height, SCRIM);
}

pub fn shade(fb: &mut PaintBuffer, r: Rect, radius: u32) {
    fb.shadow_round(r.0, r.1, r.2, r.3, radius, 10, SCRIM);
}
