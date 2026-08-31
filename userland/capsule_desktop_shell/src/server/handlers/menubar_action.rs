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

//! What each menu row does. Every arm hands off to the handler that already
//! owns the action, so the menu is a second way in rather than a second copy.

use crate::render::menubar_menu;
use crate::server::desktop;
use crate::server::handlers::{launcher_request, launchpad};
use crate::server::refresh_taskbar::refresh_taskbar;
use crate::state::{reveal_taskbar, Context, LAUNCHER_APPS};
use nonos_libc::mk_time_millis;

pub(super) fn activate(ctx: &mut Context, title: usize, row: usize) {
    match (title, row) {
        (0, 0) => front_window(ctx),
        (0, 1) | (2, 0) => launchpad::open(ctx),
        (0, 2) | (4, 0) => launch(b"app.about"),
        (1, 0) => desktop::create_entry(ctx, false),
        (1, 1) => desktop::create_entry(ctx, true),
        (1, 2) => launch(b"app.file_manager"),
        (2, 1) => show_dock(ctx),
        (2, 2) => refresh_desktop(ctx),
        (3, 0) => launch(b"app.terminal"),
        (3, 1) => launch(b"app.browser"),
        (3, 2) => launch(b"app.settings"),
        (4, 1) => launch(b"app.process_manager"),
        _ => {}
    }
}

fn launch(service: &[u8]) {
    let _ = launcher_request::request_service(service);
}

fn front_window(ctx: &mut Context) {
    let Some(index) = menubar_menu::focused(ctx) else {
        refresh_desktop(ctx);
        return;
    };
    if let Some(app) = LAUNCHER_APPS.get(index) {
        let _ = launcher_request::focus_service(app.service);
    }
}

fn show_dock(ctx: &mut Context) {
    if !ctx.taskbar.visible {
        reveal_taskbar(&mut ctx.taskbar, mk_time_millis());
        refresh_taskbar(ctx);
    }
}

fn refresh_desktop(ctx: &mut Context) {
    desktop::refresh(ctx);
}
