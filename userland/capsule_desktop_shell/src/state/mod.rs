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

pub mod apps;
pub mod chrome;
pub mod context;
pub mod indicators;
pub mod menubar;
pub mod notify;
pub mod pkg_prompt;
pub mod scale;
pub mod spotlight;
pub mod taskbar;
pub mod toasts;
pub mod tool_apps;
pub mod tray;

pub use apps::LAUNCHER_APPS;
pub use chrome::TASKBAR_WINDOW_ID;
pub use context::Context;
pub use menubar::{new_menubar_state, MenubarState};
pub use notify::NotifyLevel;
pub use pkg_prompt::PkgInstallPrompt;
pub use spotlight::SpotlightState;
pub use taskbar::{
    collapse_taskbar, expire_taskbar_pulses, expire_taskbar_visibility, mark_taskbar_launch,
    new_taskbar_state, reveal_taskbar, set_taskbar_open, TaskbarState, TASKBAR_NO_ACTIVE,
};
pub use toasts::ToastQueue;
pub use tool_apps::TOOL_APPS;
pub use tray::{TrayEntry, TrayTable};
