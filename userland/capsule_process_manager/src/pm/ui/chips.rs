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

use crate::pm::state::{State, FILTERS};
use crate::pm::theme::{ACCENT, ACCENT_TINT, MUTED, PILL_BG, PILL_BORDER};

use super::metrics::{BODY_PX, CHIP_GAP, CHIP_H, CHIP_PAD_X, CHIP_RADIUS};
use super::text;

#[path = "chips_geom.rs"]
mod chips_geom;

pub use chips_geom::{at, origin, width};

// The head band's filter row. Every chip lands over paint the frame already put
// down, so the fill and the border go through blending primitives; the active
// one wears the accent wash the sidebar's current row wears.
pub fn paint(fb: &mut PaintBuffer, state: &State) {
    let Some(mut cx) = origin(state.screen) else {
        return;
    };
    let y = chips_geom::top();
    let label_top = text::centred_top(y, CHIP_H, BODY_PX);
    for &filter in FILTERS.iter() {
        let w = chips_geom::chip_w(filter);
        let active = filter == state.filter;
        let (bg, ink) = if active { (ACCENT_TINT, ACCENT) } else { (PILL_BG, MUTED) };
        fb.fill_round(cx, y, w, CHIP_H, CHIP_RADIUS, bg);
        if !active {
            fb.stroke_round(cx, y, w, CHIP_H, CHIP_RADIUS, 1, PILL_BORDER);
        }
        text::left(fb, cx + CHIP_PAD_X, label_top, filter.label(), ink, BODY_PX);
        cx += w + CHIP_GAP;
    }
}
