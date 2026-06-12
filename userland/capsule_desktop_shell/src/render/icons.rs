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
use crate::state::apps::LauncherIcon;
use crate::state::Context;

mod about;
mod calculator;
mod constants;
mod file_manager;
mod paint;
mod process_manager;
mod settings;
mod snake;
mod terminal;
mod text_editor;

pub fn draw_app_icon(ctx: &Context, x: u32, y: u32, icon: LauncherIcon, size: u32) {
    fill_rect(
        ctx.backing_va,
        ctx.stride,
        ctx.width,
        ctx.height,
        x,
        y,
        size,
        size,
        constants::ICON_BG,
    );
    match icon {
        LauncherIcon::Terminal => terminal::terminal(ctx, x, y, size),
        LauncherIcon::FileManager => file_manager::file_manager(ctx, x, y, size),
        LauncherIcon::TextEditor => text_editor::text_editor(ctx, x, y, size),
        LauncherIcon::Settings => settings::settings(ctx, x, y, size),
        LauncherIcon::ProcessManager => process_manager::process_manager(ctx, x, y, size),
        LauncherIcon::About => about::about(ctx, x, y, size),
        LauncherIcon::Calculator => calculator::calculator(ctx, x, y, size),
        LauncherIcon::Snake => snake::snake(ctx, x, y, size),
    }
}
