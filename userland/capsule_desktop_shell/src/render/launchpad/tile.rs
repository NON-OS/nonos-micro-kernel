// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Draw one Launchpad tile: a desktop app uses its own artwork, an installed
//! tool uses its shipped glyph (or a generated tile when it has none), a
//! capsule-store app uses the generated tile since it ships no artwork we
//! embed, and all three carry a centred label underneath.

use super::gen_icon;
use super::grid::{cell_origin, CELL_H, CELL_W, TILE};
use super::hit::{target, Target};
use super::tool_icons;
use crate::render::draw_app_icon;
use crate::render::measure_aa::{measure_aa, truncate_to_width};
use crate::render::text_aa::text_aa;
use crate::render::ui_font::{top_y_centered, valid_str, UI_PX};
use crate::state::{Context, LAUNCHER_APPS, TOOL_APPS};

const LABEL_FG: u32 = 0xFFE4_EEF8;
const LABEL_GAP: u32 = 8;

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
    paint_label(ctx, label, cx, cy + TILE + LABEL_GAP);
}

// The tile label, centred within the cell.
fn paint_label(ctx: &Context, name: &[u8], cx: u32, band_y: u32) {
    let shown = truncate_to_width(valid_str(name), UI_PX, CELL_W);
    let x = cx + CELL_W.saturating_sub(measure_aa(shown, UI_PX)) / 2;
    let band_h = CELL_H.saturating_sub(TILE + LABEL_GAP);
    text_aa(ctx, x, top_y_centered(band_y, band_h, UI_PX), shown, LABEL_FG, UI_PX);
}
