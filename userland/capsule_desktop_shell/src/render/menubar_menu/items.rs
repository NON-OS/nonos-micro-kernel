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

//! The menu-bar titles and the rows each one drops down. Row order is the
//! action selector the input handler switches on, so the tables and the
//! dispatcher stay in lockstep.

use crate::state::apps::LauncherApp;
use crate::state::{Context, LAUNCHER_APPS, TASKBAR_NO_ACTIVE};

pub(super) const TITLE_COUNT: usize = 5;

const APPS: &[LauncherApp] = &LAUNCHER_APPS;

const APP_ITEMS: [&str; 3] = ["Focus Window", "Show All Apps", "About This System"];
const DESKTOP_ITEMS: [&str; 3] = ["Refresh Desktop", "Show All Apps", "About This System"];
const FILE_ITEMS: [&str; 3] = ["New Folder", "New File", "Open Files"];
const VIEW_ITEMS: [&str; 3] = ["Show Launchpad", "Show Dock", "Refresh Desktop"];
const GO_ITEMS: [&str; 3] = ["Terminal", "Browser", "Settings"];
const HELP_ITEMS: [&str; 2] = ["About This System", "Processes"];

pub fn focused(ctx: &Context) -> Option<usize> {
    if ctx.taskbar.active == TASKBAR_NO_ACTIVE {
        None
    } else {
        Some(ctx.taskbar.active as usize)
    }
}

pub(super) fn title(ctx: &Context, index: usize) -> &'static str {
    match index {
        0 => focused(ctx)
            .and_then(|i| APPS.get(i))
            .and_then(|app| core::str::from_utf8(app.label).ok())
            .unwrap_or("Desktop"),
        1 => "File",
        2 => "View",
        3 => "Go",
        _ => "Help",
    }
}

pub(super) fn rows(ctx: &Context, index: usize) -> &'static [&'static str] {
    match index {
        0 if focused(ctx).is_some() => &APP_ITEMS,
        0 => &DESKTOP_ITEMS,
        1 => &FILE_ITEMS,
        2 => &VIEW_ITEMS,
        3 => &GO_ITEMS,
        _ => &HELP_ITEMS,
    }
}
