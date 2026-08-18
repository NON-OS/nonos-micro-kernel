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

#[derive(Clone, Copy)]
pub enum LauncherIcon {
    Terminal,
    FileManager,
    TextEditor,
    Settings,
    ProcessManager,
    About,
    Calculator,
    Clock,
    Snake,
    Wallet,
    Browser,
    ImageViewer,
    AudioPlayer,
    VideoPlayer,
}

pub struct LauncherApp {
    pub icon: LauncherIcon,
    pub label: &'static [u8],
    pub service: &'static [u8],
}

pub const LAUNCHER_APPS: [LauncherApp; 11] = [
    LauncherApp { icon: LauncherIcon::Terminal, label: b"Terminal", service: b"app.terminal" },
    LauncherApp { icon: LauncherIcon::FileManager, label: b"Files", service: b"app.file_manager" },
    LauncherApp { icon: LauncherIcon::TextEditor, label: b"Editor", service: b"app.text_editor" },
    LauncherApp { icon: LauncherIcon::Settings, label: b"Settings", service: b"app.settings" },
    LauncherApp {
        icon: LauncherIcon::ProcessManager,
        label: b"Processes",
        service: b"app.process_manager",
    },
    LauncherApp { icon: LauncherIcon::About, label: b"About", service: b"app.about" },
    LauncherApp {
        icon: LauncherIcon::Calculator,
        label: b"Calculator",
        service: b"app.calculator",
    },
    LauncherApp { icon: LauncherIcon::Wallet, label: b"Wallet", service: b"app.nonos_wallet" },
    LauncherApp { icon: LauncherIcon::Browser, label: b"Browser", service: b"app.browser" },
    LauncherApp {
        icon: LauncherIcon::AudioPlayer,
        label: b"Music",
        service: b"app.audio_player",
    },
    LauncherApp {
        icon: LauncherIcon::VideoPlayer,
        label: b"Video",
        service: b"app.video_player",
    },
];
