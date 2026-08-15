// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Draw one Launchpad tile: a desktop app uses its own artwork, an installed
//! tool uses its shipped glyph (or a generated tile when it has none), a
//! capsule-store app uses the generated tile since it ships no artwork we
//! embed, and all three carry a centred label underneath.

use super::gen_icon;
use super::grid::{cell_h, cell_origin, cell_w, tile};
use super::hit::{target, Target};
use super::tool_icons;
use crate::render::draw_app_icon;
use crate::render::measure_aa::{measure_aa, truncate_to_width};
use crate::render::text_aa::text_aa;
use crate::render::ui_font;
use crate::render::ui_font::{top_y_centered, valid_str, UI_PX};
use crate::state::{Context, LAUNCHER_APPS, TOOL_APPS};

const LABEL_FG: u32 = 0xFFE4_EEF8;
const LABEL_GAP_LOGICAL: u32 = 8;

fn label_gap() -> u32 {
    LABEL_GAP_LOGICAL * ui_font::scale()
}

pub(super) fn paint(ctx: &Context, index: usize) {
    let (cx, cy) = cell_origin(ctx.width, index);
    let icon_x = cx + (cell_w() - tile()) / 2;
    let label: &[u8] = match target(index, ctx.installed_apps.len(), ctx.pkg_files.len()) {
        Target::App(a) => {
            let app = &LAUNCHER_APPS[a];
            draw_app_icon(ctx, icon_x, cy, app.icon, tile());
            app.label
        }
        Target::Tool(t) => {
            let tool = &TOOL_APPS[t];
            tool_icons::draw(ctx, icon_x, cy, tile(), tool.label);
            tool.label
        }
        Target::Installed(i) => {
            let Some(name) = ctx.installed_apps.get(i) else { return };
            gen_icon::draw(ctx, icon_x, cy, tile(), name);
            name.as_slice()
        }
        Target::Package(i) => {
            let Some(name) = ctx.pkg_files.get(i) else { return };
            let stem = name.strip_suffix(".nonos").unwrap_or(name.as_str());
            gen_icon::draw(ctx, icon_x, cy, tile(), stem.as_bytes());
            stem.as_bytes()
        }
    };
    paint_label(ctx, label, cx, cy + tile() + label_gap());
}

// The tile label, centred within the cell.
fn paint_label(ctx: &Context, name: &[u8], cx: u32, band_y: u32) {
    let shown = truncate_to_width(valid_str(name), UI_PX, cell_w());
    let x = cx + cell_w().saturating_sub(measure_aa(shown, UI_PX)) / 2;
    let band_h = cell_h().saturating_sub(tile() + label_gap());
    text_aa(ctx, x, top_y_centered(band_y, band_h, UI_PX), shown, LABEL_FG, UI_PX);
}
