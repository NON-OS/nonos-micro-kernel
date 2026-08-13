// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Paint the Launchpad: a dark full-screen wash, a title, and every app tile.

use super::grid::{count, TOP_PAD};
use super::tile;
use crate::render::fill::fill_rect;
use crate::render::measure_aa::measure_aa_bytes;
use crate::render::text_aa::text_aa_bytes;
use crate::render::ui_font::{top_y_centered, TITLE_PX};
use crate::state::Context;

const OVERLAY_BG: u32 = 0xF2_0A0E_15;
const TITLE_FG: u32 = 0xFFF2_F6FC;
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
    let title_x = ctx.width.saturating_sub(measure_aa_bytes(TITLE, TITLE_PX)) / 2;
    let title_y = top_y_centered(0, TOP_PAD, TITLE_PX);
    text_aa_bytes(ctx, title_x, title_y, TITLE, TITLE_FG, TITLE_PX);
    for i in 0..count(ctx.installed_apps.len(), ctx.pkg_files.len()) {
        tile::paint(ctx, i);
    }
}
