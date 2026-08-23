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


use super::id::IconId;

const ABOUT: &[u8] = include_bytes!("../../../assets/icons/about.a8");
const AUDIO_PLAYER: &[u8] = include_bytes!("../../../assets/icons/audio_player.a8");
const BROWSER: &[u8] = include_bytes!("../../../assets/icons/browser.a8");
const CALC: &[u8] = include_bytes!("../../../assets/icons/calc.a8");
const CLOCK: &[u8] = include_bytes!("../../../assets/icons/clock.a8");
const EDITOR: &[u8] = include_bytes!("../../../assets/icons/editor.a8");
const FILES: &[u8] = include_bytes!("../../../assets/icons/files.a8");
const FS_FILE: &[u8] = include_bytes!("../../../assets/icons/fs_file.a8");
const FS_FOLDER: &[u8] = include_bytes!("../../../assets/icons/fs_folder.a8");
const IMAGE_VIEWER: &[u8] = include_bytes!("../../../assets/icons/image_viewer.a8");
const PROCESSES: &[u8] = include_bytes!("../../../assets/icons/processes.a8");
const SETTINGS: &[u8] = include_bytes!("../../../assets/icons/settings.a8");
const SNAKE: &[u8] = include_bytes!("../../../assets/icons/snake.a8");
const TERMINAL: &[u8] = include_bytes!("../../../assets/icons/terminal.a8");
const VIDEO_PLAYER: &[u8] = include_bytes!("../../../assets/icons/video_player.a8");
const WALLET: &[u8] = include_bytes!("../../../assets/icons/wallet.a8");

/// The 8-bit coverage mask for `id`, square and row-major.
pub fn mask(id: IconId) -> &'static [u8] {
    match id {
        IconId::About => ABOUT,
        IconId::AudioPlayer => AUDIO_PLAYER,
        IconId::Browser => BROWSER,
        IconId::Calc => CALC,
        IconId::Clock => CLOCK,
        IconId::Editor => EDITOR,
        IconId::Files => FILES,
        IconId::FsFile => FS_FILE,
        IconId::FsFolder => FS_FOLDER,
        IconId::ImageViewer => IMAGE_VIEWER,
        IconId::Processes => PROCESSES,
        IconId::Settings => SETTINGS,
        IconId::Snake => SNAKE,
        IconId::Terminal => TERMINAL,
        IconId::VideoPlayer => VIDEO_PLAYER,
        IconId::Wallet => WALLET,
    }
}

/// Side length of `mask(id)`, recovered from its length rather than stored, so
/// regenerating the assets at another resolution needs no code change.
pub fn dim(id: IconId) -> u32 {
    (mask(id).len() as u32).isqrt()
}
