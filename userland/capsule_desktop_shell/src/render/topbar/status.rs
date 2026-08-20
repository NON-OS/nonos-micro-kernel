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

//! The right-hand status cluster: battery, network and the date-time stamp,
//! all from live readings, set straight on the bar with no tile behind them.

use super::battery_glyph::battery_glyph;
use super::metrics::{batt_glyph_w, dot, gap, net_glyph_w, right_margin, search_glyph_w, FG};
use super::net_glyph::net_glyph;
use super::search_box::{search_box, total};
use super::search_glyph::search_glyph;
use super::notify_dot::notify_dot;
use crate::render::layout::menubar_rect;
use crate::render::palette;
use crate::render::text_aa::text_aa_bytes;
use crate::render::ui_font::{scale, top_y_centered, STATUS_PX};
use crate::state::indicators::clock_stamp::{stamp, STAMP_LEN};
use crate::state::indicators::{battery, net};
use crate::state::Context;

pub(super) fn status(ctx: &Context) {
    let bar = menubar_rect(ctx.width);

    let online = net::online();
    let pct = battery::percent();
    let mut bbuf = [0u8; 4];
    let blen = battery::label(&mut bbuf);
    let btext = &bbuf[..blen];
    let mut sbuf = [b'-'; STAMP_LEN];
    let stamped = stamp(&mut sbuf, ctx.clock_24h);
    let when: &[u8] = if stamped { &sbuf } else { b"--:--" };

    let has_notify = ctx.last_notify_level.is_some();
    let total = total(ctx, btext, when);
    if bar.width <= total + right_margin() {
        return;
    }

    let mut x = bar.x + bar.width - right_margin() - total;
    let glyph_y = bar.y + (bar.height - 12 * scale()) / 2;
    let text_y = top_y_centered(bar.y, bar.height, STATUS_PX);
    let dot_y = bar.y + (bar.height - dot()) / 2;

    if has_notify {
        notify_dot(ctx, x, dot_y);
        x += dot() + gap();
    }
    battery_glyph(ctx, x, glyph_y, pct);
    x += batt_glyph_w() + 6 * scale();
    x = text_aa_bytes(ctx, x, text_y, btext, FG, STATUS_PX) + gap();
    net_glyph(ctx, x, glyph_y, online);
    x += net_glyph_w() + gap();
    if let Some((sx, sy, _)) = search_box(ctx, btext, when) {
        search_glyph(ctx, sx, sy);
    }
    x += search_glyph_w() + gap();
    text_aa_bytes(ctx, x, text_y, when, palette::TEXT, STATUS_PX);
}
