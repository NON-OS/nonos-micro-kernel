// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Paint the Launchpad: a dark full-screen wash, a title, and every app tile.

use super::grid::count;
use super::tile;
use crate::render::fill::fill_rect;
use crate::render::text::draw_overlay_text;
use crate::state::Context;

const OVERLAY_BG: u32 = 0xF2_0A0E_15;
const TITLE_FG: u32 = 0xFFF2_F6FC;
const GLYPH_ADV: u32 = 8;
const TITLE: &[u8] = b"Applications";

pub fn paint_launchpad(ctx: &Context) {
    fill_rect(
        ctx.backing_va,
        ctx.stride,
        ctx.width,
        ctx.height,
        0,
        0,
        ctx.width,
        ctx.height,
        OVERLAY_BG,
    );
    let title_w = TITLE.len() as u32 * GLYPH_ADV;
    let title_x = ctx.width.saturating_sub(title_w) / 2;
    draw_overlay_text(ctx, title_x, 56, TITLE, TITLE_FG);
    for i in 0..count(ctx.installed_apps.len(), ctx.pkg_files.len()) {
        tile::paint(ctx, i);
    }
}
