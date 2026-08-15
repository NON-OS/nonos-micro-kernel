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
    batt_glyph_w, dot, gap, net_glyph_w, pad_x, right_margin, tile_h, FG, TILE_BG, TILE_BORDER,
};
use super::net_glyph::net_glyph;
use super::notify_dot::notify_dot;
use crate::render::fill::fill_rect;
use crate::render::layout::menubar_rect;
use crate::render::measure_aa::measure_aa_bytes;
use crate::render::text_aa::text_aa_bytes;
use crate::render::ui_font::{scale, top_y_centered, UI_PX};
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
    let dot_w = if has_notify { dot() + gap() } else { 0 };
    let batt_w = batt_glyph_w() + 4 * scale() + measure_aa_bytes(btext, UI_PX);
    let inner = dot_w
        + batt_w
        + gap()
        + net_glyph_w()
        + gap()
        + measure_aa_bytes(time, UI_PX)
        + gap()
        + measure_aa_bytes(date, UI_PX);
    let total = inner + pad_x() * 2;
    if bar.width <= total + right_margin() {
        return;
    }

    let x0 = bar.x + bar.width - right_margin() - total;
    let tile_y = bar.y + (bar.height - tile_h()) / 2;
    tile(ctx, x0, tile_y, total);

    let glyph_y = tile_y + (tile_h() - 11 * scale()) / 2;
    let text_y = top_y_centered(tile_y, tile_h(), UI_PX);
    let dot_y = tile_y + (tile_h() - dot()) / 2;
    let mut x = x0 + pad_x();

    if has_notify {
        notify_dot(ctx, x, dot_y);
        x += dot() + gap();
    }
    battery_glyph(ctx, x, glyph_y, pct);
    x += batt_glyph_w() + 4 * scale();
    x = text_aa_bytes(ctx, x, text_y, btext, FG, UI_PX) + gap();
    net_glyph(ctx, x, glyph_y, online);
    x += net_glyph_w() + gap();
    x = text_aa_bytes(ctx, x, text_y, time, FG, UI_PX) + gap();
    text_aa_bytes(ctx, x, text_y, date, FG, UI_PX);
}

// A rounded, bordered tile behind the cluster, matching the dock's entries.
fn tile(ctx: &Context, x: u32, y: u32, w: u32) {
    let (va, st, vw, vh) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    let h = tile_h();
    fill_rect(va, st, vw, vh, x, y, w, h, TILE_BG);
    let bg = super::metrics::BAR_BG;
    let edge = scale();
    let corner = 2 * edge;
    for &(cx, cy) in
        &[(x, y), (x + w - corner, y), (x, y + h - corner), (x + w - corner, y + h - corner)]
    {
        fill_rect(va, st, vw, vh, cx, cy, corner, corner, bg);
    }
    fill_rect(va, st, vw, vh, x + corner, y, w - 2 * corner, edge, TILE_BORDER);
    fill_rect(va, st, vw, vh, x + corner, y + h - edge, w - 2 * corner, edge, TILE_BORDER);
    fill_rect(va, st, vw, vh, x, y + corner, edge, h - 2 * corner, TILE_BORDER);
    fill_rect(va, st, vw, vh, x + w - edge, y + corner, edge, h - 2 * corner, TILE_BORDER);
}
