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

use nonos_libc::mk_time_millis;

use crate::render::layout::{bottom_dock_rect, launchpad_slot_x, TASKBAR_ENTRY_W};
use crate::server::handlers::launcher_request::{self, LaunchOutcome};
use crate::server::handlers::launchpad;
use crate::server::refresh_taskbar::refresh_taskbar;
use crate::state::{mark_taskbar_launch, Context, NotifyLevel, LAUNCHER_APPS};

pub fn handle(ctx: &mut Context, x: u32, y: u32) {
    let bottom = bottom_dock_rect(ctx.width, ctx.height);
    let mut row_x = bottom.x + 12;
    for (index, app) in LAUNCHER_APPS.iter().enumerate() {
        if x >= row_x
            && x < row_x + TASKBAR_ENTRY_W
            && y >= bottom.y + 10
            && y < bottom.y + bottom.height - 10
        {
            let now = mk_time_millis();
            // Already running: focus it rather than spawn another. Asking to
            // launch first opened a duplicate on every click and left any
            // minimized window hidden behind it.
            let running = ctx.taskbar.open[index];
            let outcome = if running {
                launcher_request::focus_service(app.service)
            } else {
                launcher_request::request(app)
            };
            // Surface the result on screen: with no serial port a dock click is
            // otherwise silent, so a toast says which branch it took.
            match outcome {
                LaunchOutcome::Queued => {
                    ctx.toasts.push(b"opening a new window", NotifyLevel::Info, now);
                    mark_taskbar_launch(&mut ctx.taskbar, index, now);
                }
                LaunchOutcome::Focused if running => {
                    mark_taskbar_launch(&mut ctx.taskbar, index, now);
                }
                LaunchOutcome::Focused => {
                    ctx.toasts.push(b"window limit reached, focusing", NotifyLevel::Warn, now);
                    mark_taskbar_launch(&mut ctx.taskbar, index, now);
                }
                LaunchOutcome::Failed => {
                    ctx.toasts.push(b"could not open window", NotifyLevel::Error, now);
                }
            }
            refresh_taskbar(ctx);
            return;
        }
        row_x += TASKBAR_ENTRY_W + 6;
    }
    // The trailing dock slot is the Launchpad button.
    let lp = launchpad_slot_x(bottom);
    if x >= lp
        && x < lp + TASKBAR_ENTRY_W
        && y >= bottom.y + 10
        && y < bottom.y + bottom.height - 10
    {
        launchpad::open(ctx);
    }
}
