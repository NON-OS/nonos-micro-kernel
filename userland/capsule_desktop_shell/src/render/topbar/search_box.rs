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

//! Where the magnifier sits. The painter and the hit test both read the box
//! from here, so a click always lands on the glyph that was drawn.

use crate::render::layout::menubar_rect;
use crate::render::measure_aa::measure_aa_bytes;
use crate::render::ui_font::{scale, STATUS_PX};
use crate::state::Context;

use super::metrics::{batt_glyph_w, dot, gap, net_glyph_w, right_margin, search_glyph_w};

pub(super) fn total(ctx: &Context, btext: &[u8], when: &[u8]) -> u32 {
    let dot_w = if ctx.last_notify_level.is_some() { dot() + gap() } else { 0 };
    dot_w
        + batt_glyph_w()
        + 6 * scale()
        + measure_aa_bytes(btext, STATUS_PX)
        + gap()
        + net_glyph_w()
        + gap()
        + search_glyph_w()
        + gap()
        + measure_aa_bytes(when, STATUS_PX)
}

pub(super) fn search_box(ctx: &Context, btext: &[u8], when: &[u8]) -> Option<(u32, u32, u32)> {
    let bar = menubar_rect(ctx.width);
    let span = total(ctx, btext, when);
    if bar.width <= span + right_margin() {
        return None;
    }
    let dot_w = if ctx.last_notify_level.is_some() { dot() + gap() } else { 0 };
    let x = bar.x + bar.width - right_margin() - span
        + dot_w
        + batt_glyph_w()
        + 6 * scale()
        + measure_aa_bytes(btext, STATUS_PX)
        + gap()
        + net_glyph_w()
        + gap();
    let y = bar.y + (bar.height - 10 * scale()) / 2;
    Some((x, y, search_glyph_w()))
}
