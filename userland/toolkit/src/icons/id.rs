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


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IconId {
    About,
    AudioPlayer,
    Browser,
    Calc,
    Clock,
    Editor,
    Files,
    FsFile,
    FsFolder,
    ImageViewer,
    Processes,
    Settings,
    Snake,
    Terminal,
    VideoPlayer,
    Wallet,
}

impl IconId {
    pub const ALL: [IconId; 16] = [
        IconId::About,
        IconId::AudioPlayer,
        IconId::Browser,
        IconId::Calc,
        IconId::Clock,
        IconId::Editor,
        IconId::Files,
        IconId::FsFile,
        IconId::FsFolder,
        IconId::ImageViewer,
        IconId::Processes,
        IconId::Settings,
        IconId::Snake,
        IconId::Terminal,
        IconId::VideoPlayer,
        IconId::Wallet,
    ];
}
