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

mod badge;
mod nonos_logo;

// Real anti-aliased line-icons, one per app, rasterized from SVG (48x48 RGBA)
// and tinted at render time in each app's accent colour.
const TERMINAL: &[u8] = include_bytes!("../../assets/app_icons/terminal.rgba");
const FILES: &[u8] = include_bytes!("../../assets/app_icons/files.rgba");
const EDITOR: &[u8] = include_bytes!("../../assets/app_icons/editor.rgba");
const SETTINGS: &[u8] = include_bytes!("../../assets/app_icons/settings.rgba");
const PROCESSES: &[u8] = include_bytes!("../../assets/app_icons/processes.rgba");
const ABOUT: &[u8] = include_bytes!("../../assets/app_icons/about.rgba");
const CALC: &[u8] = include_bytes!("../../assets/app_icons/calc.rgba");
const CLOCK: &[u8] = include_bytes!("../../assets/app_icons/clock.rgba");
const SNAKE: &[u8] = include_bytes!("../../assets/app_icons/snake.rgba");
const WALLET: &[u8] = include_bytes!("../../assets/app_icons/wallet.rgba");
const BROWSER: &[u8] = include_bytes!("../../assets/app_icons/browser.rgba");
const IMAGE_VIEWER: &[u8] = include_bytes!("../../assets/app_icons/image_viewer.rgba");
const AUDIO_PLAYER: &[u8] = include_bytes!("../../assets/app_icons/audio_player.rgba");
const VIDEO_PLAYER: &[u8] = include_bytes!("../../assets/app_icons/video_player.rgba");

// One brand accent for every app: NØNOS cyan on near-black tiles. No rainbow.
const CYAN: u32 = 0xFF66E6FF;

pub fn draw_app_icon(ctx: &Context, x: u32, y: u32, icon: LauncherIcon, size: u32) {
    let glyph: &[u8] = match icon {
        LauncherIcon::Terminal => TERMINAL,
        LauncherIcon::FileManager => FILES,
        LauncherIcon::TextEditor => EDITOR,
        LauncherIcon::Settings => SETTINGS,
        LauncherIcon::ProcessManager => PROCESSES,
        LauncherIcon::About => ABOUT,
        LauncherIcon::Calculator => CALC,
        LauncherIcon::Clock => CLOCK,
        LauncherIcon::Snake => SNAKE,
        LauncherIcon::Wallet => WALLET,
        LauncherIcon::Browser => BROWSER,
        LauncherIcon::ImageViewer => IMAGE_VIEWER,
        LauncherIcon::AudioPlayer => AUDIO_PLAYER,
        LauncherIcon::VideoPlayer => VIDEO_PLAYER,
    };
    badge::badge(ctx, x, y, size, glyph, CYAN);
}

/// A file/folder tile for the desktop: the files glyph for directories, the
/// document (editor) glyph for regular files. Same cyan tile language as apps.
pub fn draw_fs_icon(ctx: &Context, x: u32, y: u32, size: u32, is_dir: bool) {
    let glyph: &[u8] = if is_dir { FILES } else { EDITOR };
    badge::badge(ctx, x, y, size, glyph, CYAN);
}

/// An installed-tool tile from its own 48x48 glyph mask, in the same badge
/// and cyan tile language as the desktop apps.
pub fn draw_tool_icon(ctx: &Context, x: u32, y: u32, size: u32, glyph: &'static [u8]) {
    badge::badge(ctx, x, y, size, glyph, CYAN);
}

/// The real NØNOS logo, drawn as the top-left brand mark on the menu bar.
pub fn draw_logo(ctx: &Context, x: u32, y: u32, size: u32) {
    nonos_logo::paint(ctx, x, y, size, 0);
}
