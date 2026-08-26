// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Paint the Launchpad: a dark full-screen wash, a title, the search pill, the
//! tiles of the current page, and the page dots.

use super::grid::title_y;
use super::view::page_slice;
use super::{dots, search, tile};
use crate::render::fill::fill_rect;
use crate::render::measure_aa::measure_aa_bytes;
use crate::render::text_aa::text_aa_bytes;
use crate::render::ui_font::TITLE_PX;
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
    text_aa_bytes(ctx, title_x, title_y(), TITLE, TITLE_FG, TITLE_PX);
    search::paint(ctx);
    for (i, t) in page_slice(ctx).iter().enumerate() {
        tile::paint(ctx, i, *t);
    }
    dots::paint(ctx);
}
