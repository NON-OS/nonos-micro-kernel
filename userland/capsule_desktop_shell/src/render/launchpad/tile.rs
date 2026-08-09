// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Draw one Launchpad tile: a desktop app uses its own artwork, an installed
//! tool uses its shipped glyph (or a generated tile when it has none), a
//! capsule-store app uses the generated tile since it ships no artwork we
//! embed, and all three carry a centred label underneath.

use super::gen_icon;
use super::grid::{cell_origin, CELL_W, TILE};
use super::hit::{target, Target};
use super::tool_icons;
use crate::render::draw_app_icon;
use crate::render::text::draw_overlay_text;
use crate::state::{Context, LAUNCHER_APPS, TOOL_APPS};

const LABEL_FG: u32 = 0xFFE4_EEF8;
const GLYPH_ADV: u32 = 8;
const MAX_LABEL: usize = 14;

pub(super) fn paint(ctx: &Context, index: usize) {
    let (cx, cy) = cell_origin(ctx.width, index);
    let icon_x = cx + (CELL_W - TILE) / 2;
    let label: &[u8] = match target(index, ctx.installed_apps.len(), ctx.pkg_files.len()) {
        Target::App(a) => {
            let app = &LAUNCHER_APPS[a];
            draw_app_icon(ctx, icon_x, cy, app.icon, TILE);
            app.label
        }
        Target::Tool(t) => {
            let tool = &TOOL_APPS[t];
            tool_icons::draw(ctx, icon_x, cy, TILE, tool.label);
            tool.label
        }
        Target::Installed(i) => {
            let Some(name) = ctx.installed_apps.get(i) else { return };
            gen_icon::draw(ctx, icon_x, cy, TILE, name);
            name.as_slice()
        }
        Target::Package(i) => {
            let Some(name) = ctx.pkg_files.get(i) else { return };
            let stem = name.strip_suffix(".nonos").unwrap_or(name.as_str());
            gen_icon::draw(ctx, icon_x, cy, TILE, stem.as_bytes());
            stem.as_bytes()
        }
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
