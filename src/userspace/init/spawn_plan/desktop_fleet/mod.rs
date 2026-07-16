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

//! Desktop capsule fleet: compositor, splash, wm, wallpaper and shell.

mod desktop_enabled;
mod spawn;
mod spawn_boot_splash;
mod spawn_compositor;
mod spawn_early_display;
mod spawn_gui_core;
mod spawn_input_router;
mod spawn_shell;
mod spawn_wallpaper;
mod spawn_wallpaper_catalog;
mod spawn_wm;

pub(super) use spawn::spawn;
pub(super) use spawn_early_display::spawn_early_display;
