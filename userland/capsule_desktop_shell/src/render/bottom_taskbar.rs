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

use super::draw_app_icon;
use super::layout::{
    bottom_dock_rect, dock_box_inset, dock_gap, dock_pad, launchpad_slot_x, taskbar_entry_w, Rect,
};
use super::surface::surface;
use super::{palette, ui_font};
use crate::state::{Context, LAUNCHER_APPS, TASKBAR_NO_ACTIVE};

const ICON_SIZE_LOGICAL: u32 = 40;

fn icon_size() -> u32 {
    ICON_SIZE_LOGICAL * ui_font::scale()
}

// The Launchpad button: the familiar 3x3 grid, sitting in the dock slot just
// past the last app.
fn draw_launchpad_button(ctx: &Context, box_top: u32, box_h: u32) {
    let slot_x = launchpad_slot_x(bottom_dock_rect(ctx.width, ctx.height));
    let x0 = slot_x + (taskbar_entry_w() - icon_size()) / 2;
    let y0 = box_top + (box_h - icon_size()) / 2;
    let cell = icon_size() / 3;
    let r = cell.saturating_sub(4 * ui_font::scale()) / 2;
    let mut fb = surface(ctx);
    for row in 0..3 {
        for col in 0..3 {
            let cx = x0 + col * cell + cell / 2;
            let cy = y0 + row * cell + cell / 2;
            fb.circle(cx, cy, r, palette::ACCENT);
        }
    }
}

// The running indicator: a dot under the tile, accented when the app holds
// focus and dimmed when it is merely open.
fn running_dot(ctx: &Context, cx: u32, active: bool) {
    let dock = bottom_dock_rect(ctx.width, ctx.height);
    let cy = dock.y + dock.height - 6 * ui_font::scale();
    let argb = if active { palette::ACCENT } else { palette::TEXT_DIM };
    surface(ctx).circle(cx, cy, 2 * ui_font::scale(), argb);
}

pub fn paint_bottom_taskbar(ctx: &Context) {
    let dock = bottom_dock_rect(ctx.width, ctx.height);
    let box_top = dock.y + dock_box_inset();
    let box_h = dock.height - 2 * dock_box_inset();
    let mut x = dock.x + dock_pad();
    for (index, app) in LAUNCHER_APPS.iter().enumerate() {
        let open = ctx.taskbar.open[index];
        let active =
            ctx.taskbar.active != TASKBAR_NO_ACTIVE && ctx.taskbar.active as usize == index;
        let pulsing = ctx.taskbar.pulse_until_ms[index] > 0;
        let bg = if active {
            palette::TILE_ACTIVE
        } else if pulsing {
            palette::TILE_PULSE
        } else if open {
            palette::TILE_OPEN
        } else {
            palette::TILE_IDLE
        };
        let tile = Rect { x, y: box_top, width: taskbar_entry_w(), height: box_h };
        super::panel::round_fill(ctx, tile, palette::R_TILE, bg);
        if open || active || pulsing {
            running_dot(ctx, x + taskbar_entry_w() / 2, active);
        }
        let icon_x = x + (taskbar_entry_w() - icon_size()) / 2;
        let icon_y = box_top + (box_h - icon_size()) / 2;
        draw_app_icon(ctx, icon_x, icon_y, app.icon, icon_size());
        x += taskbar_entry_w() + dock_gap();
    }
    draw_launchpad_button(ctx, box_top, box_h);
}
