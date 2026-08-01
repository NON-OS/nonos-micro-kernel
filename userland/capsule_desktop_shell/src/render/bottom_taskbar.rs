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
use super::layout::{bottom_dock_rect, launchpad_slot_x, TASKBAR_ENTRY_W};
use crate::state::{Context, LAUNCHER_APPS, TASKBAR_NO_ACTIVE};

const ICON_SIZE: u32 = 40;
const LAUNCHPAD_DOT: u32 = 0xFF9F_B4D6;

// The Launchpad button: the familiar 3x3 grid of cells, sitting in the dock
// slot just past the last app.
fn draw_launchpad_button(ctx: &Context, box_top: u32, box_h: u32) {
    let slot_x = launchpad_slot_x(bottom_dock_rect(ctx.width, ctx.height));
    let x0 = slot_x + (TASKBAR_ENTRY_W - ICON_SIZE) / 2;
    let y0 = box_top + (box_h - ICON_SIZE) / 2;
    let cell = ICON_SIZE / 3;
    let dot = cell.saturating_sub(4);
    for row in 0..3 {
        for col in 0..3 {
            let x = x0 + col * cell + 2;
            let y = y0 + row * cell + 2;
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
    let box_top = dock.y + 10;
    let box_h = dock.height - 20;
    let mut x = dock.x + 12;
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
            TASKBAR_ENTRY_W,
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
                x + 18,
                box_top + box_h - 3,
                TASKBAR_ENTRY_W - 36,
                3,
                indicator,
            );
        }
        let icon_x = x + (TASKBAR_ENTRY_W - ICON_SIZE) / 2;
        let icon_y = box_top + (box_h - ICON_SIZE) / 2;
        draw_app_icon(ctx, icon_x, icon_y, app.icon, ICON_SIZE);
        x += TASKBAR_ENTRY_W + 6;
    }
    draw_launchpad_button(ctx, box_top, box_h);
}
