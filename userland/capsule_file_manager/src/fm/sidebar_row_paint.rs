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

use super::chrome_pill::{pill_ink, pill_r, PillState};
use super::icon_draw::draw;
use super::measure_text::truncate_to_width;
use super::sidebar_icon::nav_icon;
use super::sidebar_metrics::{line_box, rail_text_x, rail_w, ICON_S, RAIL_PAD, RAIL_X};
use super::sidebar_model::SideRow;
use super::sidebar_rows::row_active;
use super::state::State;
use super::theme::{HAIR, INK2, INK3, R_CARD};

const LABEL_PX: f32 = 18.0;
const TITLE_PX: f32 = 13.0;
const PILL_INSET: u32 = 4;

/// One navigation row. The active row is the full-width pill from the shared
/// chrome, so its tint, hairline and glow are the same ones every other selected
/// control on screen wears.
pub fn nav_row(state: &State, fb: &mut PaintBuffer, row: &SideRow) {
    let hit = match row.hit.as_ref() {
        Some(hit) => hit,
        None => return,
    };
    let active = row_active(state, hit);
    let h = row.h.saturating_sub(PILL_INSET);
    if active {
        pill_r(fb, RAIL_X, row.y, rail_w(), h, R_CARD, PillState::Active);
    }
    let ink = if active { pill_ink(PillState::Active) } else { INK2 };
    let tint = if active { pill_ink(PillState::Active) } else { INK3 };
    let iy = row.y + h.saturating_sub(ICON_S) / 2;
    draw(fb, nav_icon(hit), RAIL_X + RAIL_PAD, iy, ICON_S, tint);
    let tx = rail_text_x();
    let avail = (RAIL_X + rail_w()).saturating_sub(tx + RAIL_PAD);
    let label = truncate_to_width(fb, row.label.as_str(), LABEL_PX, avail);
    let ty = row.y + h.saturating_sub(line_box(LABEL_PX)) / 2;
    let _ = fb.text_ttf(tx as i32, ty as i32, label, ink, LABEL_PX);
}

/// A group title, set in the tertiary ink at the readable floor; anything
/// smaller is clamped back up to it by the font layer anyway.
pub fn section_label(fb: &mut PaintBuffer, label: &str, y: u32, h: u32) {
    let ty = y + h.saturating_sub(line_box(TITLE_PX)) / 2;
    let x = RAIL_X + RAIL_PAD;
    let _ = fb.text_ttf(x as i32, ty as i32, label, INK3, TITLE_PX);
}

/// The hairline between two groups, centred in the band the rule owns. `HAIR`
/// carries alpha, so it must be blended over the rail rather than written flat.
pub fn rule(fb: &mut PaintBuffer, y: u32, h: u32) {
    let x = RAIL_X + RAIL_PAD;
    let w = rail_w().saturating_sub(RAIL_PAD * 2);
    fb.blend_rect(x, y + h / 2, w, 1, HAIR);
}
