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

use super::fill::fill_rect;
use crate::state::Context;
use crate::state::apps::LauncherIcon;

pub fn draw_app_icon(ctx: &Context, x: u32, y: u32, icon: LauncherIcon, size: u32) {
    let bg = 0xFF223245;
    let fg = 0xFFE3EEF9;
    fill_rect(ctx.backing_va, ctx.stride, ctx.width, ctx.height, x, y, size, size, bg);
    match icon {
        LauncherIcon::Terminal => terminal(ctx, x, y, fg),
        LauncherIcon::FileManager => file_manager(ctx, x, y, fg),
        LauncherIcon::TextEditor => text_editor(ctx, x, y, fg),
        LauncherIcon::Settings => settings(ctx, x, y, fg),
        LauncherIcon::ProcessManager => process_manager(ctx, x, y, fg),
        LauncherIcon::About => about(ctx, x, y, fg),
        LauncherIcon::Calculator => calculator(ctx, x, y, fg),
    }
}

fn paint(ctx: &Context, x: u32, y: u32, w: u32, h: u32, argb: u32) {
    fill_rect(ctx.backing_va, ctx.stride, ctx.width, ctx.height, x, y, w, h, argb);
}

fn terminal(ctx: &Context, x: u32, y: u32, fg: u32) {
    paint(ctx, x + 2, y + 3, 12, 1, fg);
    paint(ctx, x + 2, y + 4, 1, 8, fg);
    paint(ctx, x + 13, y + 4, 1, 8, fg);
    paint(ctx, x + 3, y + 11, 10, 1, fg);
    paint(ctx, x + 5, y + 6, 2, 2, fg);
    paint(ctx, x + 7, y + 8, 2, 1, fg);
    paint(ctx, x + 9, y + 9, 2, 1, fg);
    paint(ctx, x + 8, y + 10, 3, 1, fg);
}

fn file_manager(ctx: &Context, x: u32, y: u32, fg: u32) {
    paint(ctx, x + 2, y + 5, 12, 7, fg);
    paint(ctx, x + 4, y + 3, 4, 2, fg);
    paint(ctx, x + 3, y + 6, 10, 1, 0xFF223245);
    paint(ctx, x + 3, y + 8, 8, 1, 0xFF223245);
}

fn text_editor(ctx: &Context, x: u32, y: u32, fg: u32) {
    paint(ctx, x + 3, y + 2, 9, 12, fg);
    paint(ctx, x + 9, y + 2, 3, 3, 0xFF223245);
    paint(ctx, x + 5, y + 5, 5, 1, 0xFF223245);
    paint(ctx, x + 5, y + 7, 4, 1, 0xFF223245);
    paint(ctx, x + 5, y + 9, 5, 1, 0xFF223245);
    paint(ctx, x + 10, y + 9, 3, 1, 0xFF73D6C3);
    paint(ctx, x + 11, y + 10, 2, 1, 0xFF73D6C3);
}

fn settings(ctx: &Context, x: u32, y: u32, fg: u32) {
    paint(ctx, x + 3, y + 4, 10, 1, fg);
    paint(ctx, x + 3, y + 7, 10, 1, fg);
    paint(ctx, x + 3, y + 10, 10, 1, fg);
    paint(ctx, x + 6, y + 3, 2, 3, fg);
    paint(ctx, x + 9, y + 6, 2, 3, fg);
    paint(ctx, x + 5, y + 9, 2, 3, fg);
}

fn process_manager(ctx: &Context, x: u32, y: u32, fg: u32) {
    paint(ctx, x + 3, y + 10, 2, 2, fg);
    paint(ctx, x + 6, y + 7, 2, 5, fg);
    paint(ctx, x + 9, y + 5, 2, 7, fg);
    paint(ctx, x + 12, y + 3, 2, 9, fg);
}

fn about(ctx: &Context, x: u32, y: u32, fg: u32) {
    paint(ctx, x + 4, y + 2, 6, 2, fg);
    paint(ctx, x + 3, y + 4, 2, 6, fg);
    paint(ctx, x + 10, y + 4, 2, 6, fg);
    paint(ctx, x + 4, y + 10, 6, 2, fg);
    paint(ctx, x + 5, y + 5, 2, 2, 0xFF223245);
    paint(ctx, x + 6, y + 6, 2, 2, fg);
    paint(ctx, x + 8, y + 8, 2, 2, fg);
}

fn calculator(ctx: &Context, x: u32, y: u32, fg: u32) {
    paint(ctx, x + 3, y + 2, 10, 12, fg);
    paint(ctx, x + 5, y + 4, 6, 2, 0xFF223245);
    paint(ctx, x + 5, y + 8, 2, 2, 0xFF223245);
    paint(ctx, x + 8, y + 8, 2, 2, 0xFF223245);
    paint(ctx, x + 5, y + 11, 2, 2, 0xFF223245);
    paint(ctx, x + 8, y + 11, 2, 2, 0xFF223245);
}
