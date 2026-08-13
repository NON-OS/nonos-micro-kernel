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
use super::fill::fill_rect;
use super::layout::{bottom_dock_rect, launchpad_slot_x, taskbar_entry_w};
use super::ui_font;
use crate::state::{Context, LAUNCHER_APPS, TASKBAR_NO_ACTIVE};

const ICON_SIZE_LOGICAL: u32 = 40;
const LAUNCHPAD_DOT: u32 = 0xFF9F_B4D6;

fn icon_size() -> u32 {
    ICON_SIZE_LOGICAL * ui_font::scale()
}

// The Launchpad button: the familiar 3x3 grid of cells, sitting in the dock
// slot just past the last app.
fn draw_launchpad_button(ctx: &Context, box_top: u32, box_h: u32) {
    let slot_x = launchpad_slot_x(bottom_dock_rect(ctx.width, ctx.height));
    let x0 = slot_x + (taskbar_entry_w() - icon_size()) / 2;
    let y0 = box_top + (box_h - icon_size()) / 2;
    let cell = icon_size() / 3;
    let dot = cell.saturating_sub(4 * ui_font::scale());
    for row in 0..3 {
        for col in 0..3 {
            let x = x0 + col * cell + 2 * ui_font::scale();
            let y = y0 + row * cell + 2 * ui_font::scale();
            fill_rect(
                ctx.backing_va,
                ctx.stride,
                ctx.width,
                ctx.height,
                x,
                y,
                dot,
                dot,
                LAUNCHPAD_DOT,
            );
        }
    }
}

pub fn paint_bottom_taskbar(ctx: &Context) {
    let dock = bottom_dock_rect(ctx.width, ctx.height);
    let box_top = dock.y + 10 * ui_font::scale();
    let box_h = dock.height - 20 * ui_font::scale();
    let mut x = dock.x + 12 * ui_font::scale();
    for (index, app) in LAUNCHER_APPS.iter().enumerate() {
        let open = ctx.taskbar.open[index];
        let active =
            ctx.taskbar.active != TASKBAR_NO_ACTIVE && ctx.taskbar.active as usize == index;
        let pulsing = ctx.taskbar.pulse_until_ms[index] > 0;
        let bg = if active {
            0x2F66_7F92
        } else if pulsing {
            0x2B5F_7468
        } else if open {
            0x294C_5F70
        } else {
            0x2241_5164
        };
        fill_rect(
            ctx.backing_va,
            ctx.stride,
            ctx.width,
            ctx.height,
            x,
            box_top,
            taskbar_entry_w(),
            box_h,
            bg,
        );
        if open || active || pulsing {
            let indicator = if active { 0xFF76_D98A } else { 0xFF76_C7D7 };
            fill_rect(
                ctx.backing_va,
                ctx.stride,
                ctx.width,
                ctx.height,
                x + 18 * ui_font::scale(),
                box_top + box_h - 3 * ui_font::scale(),
                taskbar_entry_w() - 36 * ui_font::scale(),
                3 * ui_font::scale(),
                indicator,
            );
        }
        let icon_x = x + (taskbar_entry_w() - icon_size()) / 2;
        let icon_y = box_top + (box_h - icon_size()) / 2;
        draw_app_icon(ctx, icon_x, icon_y, app.icon, icon_size());
        x += taskbar_entry_w() + 6 * ui_font::scale();
    }
    draw_launchpad_button(ctx, box_top, box_h);
}
