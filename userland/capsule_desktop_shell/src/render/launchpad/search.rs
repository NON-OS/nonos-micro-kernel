// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The Launchpad search pill: a rounded field that filters the grid as you type.

use super::grid::{search_h, search_w, search_y};
use crate::render::layout::Rect;
use crate::render::measure_aa::{measure_aa, truncate_to_width};
use crate::render::palette;
use crate::render::panel::panel;
use crate::render::text_aa::text_aa;
use crate::render::ui_font::{scale, top_y_centered, UI_PX};
use crate::state::Context;

const PLACEHOLDER: &str = "Search applications...";

pub(super) fn rect(ctx: &Context) -> (u32, u32, u32, u32) {
    let w = search_w().min(ctx.width);
    let x = (ctx.width.saturating_sub(w)) / 2;
    (x, search_y(), w, search_h())
}

/// Whether a point falls inside the search pill, so a click there is absorbed
/// by the field rather than dismissing the overlay.
pub(crate) fn hit(ctx: &Context, px: u32, py: u32) -> bool {
    let (x, y, w, h) = rect(ctx);
    px >= x && px < x + w && py >= y && py < y + h
}

pub(super) fn paint(ctx: &Context) {
    let (x, y, w, h) = rect(ctx);
    let border = if ctx.launchpad_query.is_empty() {
        palette::LINE_SOFT
    } else {
        palette::ACCENT
    };
    panel(
        ctx,
        Rect { x, y, width: w, height: h },
        palette::R_DOCK,
        palette::TILE_FILL,
        border,
    );

    let pad = 14 * scale();
    let text_x = x + pad;
    let avail = w.saturating_sub(2 * pad);
    let top_y = top_y_centered(y, h, UI_PX);

    if ctx.launchpad_query.is_empty() {
        let s = truncate_to_width(PLACEHOLDER, UI_PX, avail);
        text_aa(ctx, text_x, top_y, s, palette::TEXT_DIM, UI_PX);
    } else {
        let s = truncate_to_width(&ctx.launchpad_query, UI_PX, avail);
        text_aa(ctx, text_x, top_y, s, palette::TEXT, UI_PX);
        let caret_x = text_x + measure_aa(s, UI_PX) + 2 * scale();
        text_aa(ctx, caret_x, top_y, "_", palette::ACCENT, UI_PX);
    }
}
