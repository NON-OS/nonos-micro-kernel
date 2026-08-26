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

use crate::state::apps::LauncherIcon;
use crate::state::Context;
use nonos_toolkit::icons::{mask, IconId};

mod badge;
mod nonos_logo;

const CYAN: u32 = crate::render::palette::ACCENT;

pub fn draw_app_icon(ctx: &Context, x: u32, y: u32, icon: LauncherIcon, size: u32) {
    badge::badge(ctx, x, y, size, icon_bytes(icon), CYAN);
}

/// The same app mark with no tile behind it, for callers that paint their own
/// cell (the dock).
pub fn draw_app_glyph(ctx: &Context, x: u32, y: u32, icon: LauncherIcon, size: u32) {
    badge::glyph(ctx, x, y, size, icon_bytes(icon), CYAN);
}

fn icon_bytes(icon: LauncherIcon) -> &'static [u8] {
    mask(match icon {
        LauncherIcon::Terminal => IconId::Terminal,
        LauncherIcon::FileManager => IconId::Files,
        LauncherIcon::TextEditor => IconId::Editor,
        LauncherIcon::Settings => IconId::Settings,
        LauncherIcon::ProcessManager => IconId::Processes,
        LauncherIcon::About => IconId::About,
        LauncherIcon::Calculator => IconId::Calc,
        LauncherIcon::Clock => IconId::Clock,
        LauncherIcon::Snake => IconId::Snake,
        LauncherIcon::Wallet => IconId::Wallet,
        LauncherIcon::Browser => IconId::Browser,
        LauncherIcon::ImageViewer => IconId::ImageViewer,
        LauncherIcon::AudioPlayer => IconId::AudioPlayer,
        LauncherIcon::VideoPlayer => IconId::VideoPlayer,
    })
}

/// A file/folder mark for the desktop: the folder glyph for directories, the
/// document glyph for regular files. Untiled, so the wallpaper shows through.
pub fn draw_fs_icon(ctx: &Context, x: u32, y: u32, size: u32, is_dir: bool) {
    let id = if is_dir { IconId::FsFolder } else { IconId::FsFile };
    badge::art(ctx, x, y, size, mask(id), CYAN);
}

/// An installed-tool tile from its own 96x96 glyph mask, in the same badge
/// and cyan tile language as the desktop apps.
pub fn draw_tool_icon(ctx: &Context, x: u32, y: u32, size: u32, glyph: &'static [u8]) {
    badge::badge(ctx, x, y, size, glyph, CYAN);
}

/// The real NØNOS logo, drawn as the top-left brand mark on the menu bar.
pub fn draw_logo(ctx: &Context, x: u32, y: u32, size: u32) {
    nonos_logo::paint(ctx, x, y, size, 0);
}
