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

extern crate alloc;

use nonos_app_skeleton::PaintBuffer;

use super::chrome_card::Plate;
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::measure_text::truncate_to_width;
use super::sidebar_metrics::{line_box, rail_text_x, rail_w, ICON_S, RAIL_PAD, RAIL_X};
use super::sidebar_usage::{fill_pct, occupancy};
use super::state::State;
use super::theme::{CY, INK, INK3, PANEL, RAISE, R_CARD, TINT_BOT, TINT_TOP};

// Card band: the plate itself, then the gap holding it clear of the window foot.
pub const CARD_H: u32 = 88;
pub const CARD_FOOT: u32 = 16;

const TITLE_PX: f32 = 18.0;
const SUB_PX: f32 = 13.0;
const BAR_H: u32 = 6;
const CHEVRON: u32 = 14;
const DRIVE: &str = "NØNOS Drive";

/// The drive card pinned to the foot of the rail. Every figure on it comes from
/// the cached `vfs::usage` read taken on refresh, never from a call made inside
/// paint.
pub fn paint_storage(state: &State, fb: &mut PaintBuffer, y: u32) {
    let w = rail_w();
    Plate::new(PANEL).radius(R_CARD).tint(TINT_TOP, TINT_BOT).draw(fb, RAIL_X, y, w, CARD_H);
    let head = y + RAIL_PAD;
    draw(fb, Icon::Cube, RAIL_X + RAIL_PAD, head, ICON_S, CY);
    let ty = head + ICON_S.saturating_sub(line_box(TITLE_PX)) / 2;
    let _ = fb.text_ttf(rail_text_x() as i32, ty as i32, DRIVE, INK, TITLE_PX);
    let cx = RAIL_X + w.saturating_sub(RAIL_PAD + CHEVRON);
    draw(fb, Icon::Chevron, cx, head + ICON_S.saturating_sub(CHEVRON) / 2, CHEVRON, INK3);
    let inner = w.saturating_sub(RAIL_PAD * 2);
    let bar_y = head + ICON_S + RAIL_PAD;
    bar(fb, RAIL_X + RAIL_PAD, bar_y, inner, fill_pct(state));
    let line = occupancy(state);
    let cut = truncate_to_width(fb, line.as_str(), SUB_PX, inner);
    let _ = fb.text_ttf((RAIL_X + RAIL_PAD) as i32, (bar_y + BAR_H + 6) as i32, cut, INK3, SUB_PX);
}

// Track and fill are both opaque and land inside the plate that was just drawn,
// so a rounded fill is safe here; the plate underneath is what does the blending.
fn bar(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, pct: u32) {
    fb.fill_round(x, y, w, BAR_H, BAR_H / 2, RAISE);
    let filled = w * pct.min(100) / 100;
    if filled >= BAR_H {
        fb.fill_round(x, y, filled, BAR_H, BAR_H / 2, CY);
    }
}
