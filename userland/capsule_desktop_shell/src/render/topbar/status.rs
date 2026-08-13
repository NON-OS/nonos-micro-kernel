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

//! The right-hand status cluster: a rounded tile holding the notification dot,
//! battery, network, clock and date, all from live readings.

use super::battery_glyph::battery_glyph;
use super::metrics::{
    BATT_GLYPH_W, DOT, FG, GAP, NET_GLYPH_W, PAD_X, RIGHT_MARGIN, TILE_BG, TILE_BORDER, tile_h,
};
use super::net_glyph::net_glyph;
use super::notify_dot::notify_dot;
use crate::render::fill::fill_rect;
use crate::render::layout::menubar_rect;
use crate::render::measure_aa::measure_aa_bytes;
use crate::render::text_aa::text_aa_bytes;
use crate::render::ui_font::{top_y_centered, UI_PX};
use crate::state::indicators::{battery, clock, net};
use crate::state::Context;

pub(super) fn status(ctx: &Context) {
    let bar = menubar_rect(ctx.width);

    // Live readings.
    let online = net::online();
    let pct = battery::percent();
    let mut bbuf = [0u8; 4];
    let blen = battery::label(&mut bbuf);
    let btext = &bbuf[..blen];
    let mut tbuf = [b' '; 5];
    let time: &[u8] = if clock::hhmm(&mut tbuf, ctx.clock_24h) { &tbuf } else { b"--:--" };
    let mut dbuf = [b'-'; 10];
    let date: &[u8] = if clock::ymd(&mut dbuf) { &dbuf } else { b"----------" };

    // Widths.
    let has_notify = ctx.last_notify_level.is_some();
    let dot_w = if has_notify { DOT + GAP } else { 0 };
    let batt_w = BATT_GLYPH_W + 4 + measure_aa_bytes(btext, UI_PX);
    let inner = dot_w
        + batt_w
        + GAP
        + NET_GLYPH_W
        + GAP
        + measure_aa_bytes(time, UI_PX)
        + GAP
        + measure_aa_bytes(date, UI_PX);
    let total = inner + PAD_X * 2;
    if bar.width <= total + RIGHT_MARGIN {
        return;
    }

    let x0 = bar.x + bar.width - RIGHT_MARGIN - total;
    let tile_y = bar.y + (bar.height - tile_h()) / 2;
    tile(ctx, x0, tile_y, total);

    let glyph_y = tile_y + (tile_h() - 11) / 2;
    let text_y = top_y_centered(tile_y, tile_h(), UI_PX);
    let dot_y = tile_y + (tile_h() - DOT) / 2;
    let mut x = x0 + PAD_X;

    if has_notify {
        notify_dot(ctx, x, dot_y);
        x += DOT + GAP;
    }
    battery_glyph(ctx, x, glyph_y, pct);
    x += BATT_GLYPH_W + 4;
    x = text_aa_bytes(ctx, x, text_y, btext, FG, UI_PX) + GAP;
    net_glyph(ctx, x, glyph_y, online);
    x += NET_GLYPH_W + GAP;
    x = text_aa_bytes(ctx, x, text_y, time, FG, UI_PX) + GAP;
    text_aa_bytes(ctx, x, text_y, date, FG, UI_PX);
}

// A rounded, bordered tile behind the cluster, matching the dock's entries.
fn tile(ctx: &Context, x: u32, y: u32, w: u32) {
    let (va, st, vw, vh) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    let h = tile_h();
    fill_rect(va, st, vw, vh, x, y, w, h, TILE_BG);
    let bg = super::metrics::BAR_BG;
    for &(cx, cy) in &[(x, y), (x + w - 2, y), (x, y + h - 2), (x + w - 2, y + h - 2)] {
        fill_rect(va, st, vw, vh, cx, cy, 2, 2, bg);
    }
    fill_rect(va, st, vw, vh, x + 2, y, w - 4, 1, TILE_BORDER);
    fill_rect(va, st, vw, vh, x + 2, y + h - 1, w - 4, 1, TILE_BORDER);
    fill_rect(va, st, vw, vh, x, y + 2, 1, h - 4, TILE_BORDER);
    fill_rect(va, st, vw, vh, x + w - 1, y + 2, 1, h - 4, TILE_BORDER);
}
