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

//! The account block pinned to the bottom of the rail, above its own divider.

use nonos_app_skeleton::PaintBuffer;

use crate::editor::widget::truncate_to_width;

use super::metrics::{footer_h, lh, rail_x, AVATAR, BODY, RAIL_PAD, RAIL_W};
use super::palette::{BRAND_B, MUTED, NAV_ACCENT, RAIL_LINE, TITLE};

pub(super) fn paint_footer(fb: &mut PaintBuffer, h: u32) {
    let top = h.saturating_sub(footer_h());
    fb.fill_rect(rail_x(), top, RAIL_W - 1, 1, RAIL_LINE);
    let x = rail_x() + RAIL_PAD;
    let cy = top + footer_h() / 2;
    fb.circle(x + AVATAR / 2, cy, AVATAR / 2, NAV_ACCENT);
    let gw = fb.measure_ttf("M", BODY).max(0) as u32;
    let gx = x + AVATAR / 2 - (gw / 2).min(AVATAR / 2);
    let gy = cy.saturating_sub(lh(BODY) / 2) as i32;
    let _ = fb.text_ttf(gx as i32, gy, "M", BRAND_B, BODY);
    let tx = (x + AVATAR + 12) as i32;
    let ty = cy.saturating_sub(lh(BODY)) as i32;
    let avail = RAIL_W.saturating_sub(RAIL_PAD * 2 + AVATAR + 12) as i32;
    let name = truncate_to_width(fb, "Mehedi Hasan", BODY, avail);
    let _ = fb.text_ttf(tx, ty, name, TITLE, BODY);
    let kind = truncate_to_width(fb, "Local Account", BODY, avail);
    let _ = fb.text_ttf(tx, ty + lh(BODY) as i32, kind, MUTED, BODY);
}
