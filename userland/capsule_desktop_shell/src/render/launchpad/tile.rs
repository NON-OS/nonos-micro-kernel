// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Draw one Launchpad tile: a desktop app uses its own artwork, an installed
//! tool gets a generated tile, and both carry a centred label underneath.

use super::gen_icon;
use super::grid::{cell_origin, CELL_W, TILE};
use crate::render::draw_app_icon;
use crate::render::text::draw_overlay_text;
use crate::state::{Context, LAUNCHER_APPS, TOOL_APPS};

const LABEL_FG: u32 = 0xFFE4_EEF8;
const GLYPH_ADV: u32 = 8;
const MAX_LABEL: usize = 14;

pub(super) fn paint(ctx: &Context, index: usize) {
    let (cx, cy) = cell_origin(ctx.width, index);
    let icon_x = cx + (CELL_W - TILE) / 2;
    let apps = LAUNCHER_APPS.len();
    let label: &[u8] = if index < apps {
        let app = &LAUNCHER_APPS[index];
        draw_app_icon(ctx, icon_x, cy, app.icon, TILE);
        app.label
    } else {
        let tool = &TOOL_APPS[index - apps];
        gen_icon::draw(ctx, icon_x, cy, TILE, tool.label);
        tool.label
    };
    paint_label(ctx, label, cx, cy + TILE + 8);
}

// The tile label, centred within the cell.
fn paint_label(ctx: &Context, name: &[u8], cx: u32, y: u32) {
    let shown = if name.len() > MAX_LABEL { &name[..MAX_LABEL] } else { name };
    let text_w = shown.len() as u32 * GLYPH_ADV;
    let x = cx + CELL_W.saturating_sub(text_w) / 2;
    draw_overlay_text(ctx, x, y, shown, LABEL_FG);
}
