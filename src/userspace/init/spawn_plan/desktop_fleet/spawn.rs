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

use super::desktop_enabled::desktop_enabled;
use super::spawn_boot_splash::spawn_boot_splash;
use super::spawn_gui_core::spawn_gui_core;
use super::spawn_shell::spawn_shell;
use super::spawn_wallpaper::spawn_wallpaper;
use super::spawn_wallpaper_catalog::spawn_wallpaper_catalog;
use super::spawn_wm::spawn_wm;

pub(crate) fn spawn() {
    if !desktop_enabled() {
        return;
    }
    spawn_gui_core();
    spawn_boot_splash();
    spawn_wm();
    spawn_wallpaper_catalog();
    spawn_wallpaper();
    spawn_shell();
    super::super::desktop_services::spawn();
}
