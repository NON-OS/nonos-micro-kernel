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

//! Launch a capsule-store app from the Launchpad. The kernel's spawn-instance
//! path resolves a service name against a compile-time table of embedded
//! capsules, so it can never reach an app installed at runtime; the installer
//! owns that load instead. The pid it returns is remembered, so a later click
//! raises the running window rather than loading a second copy.

use nonos_libc::mk_ipc_send_to_pid;

use super::launcher_request::focus_frame;
use crate::installer_client::load_by_name;
use crate::state::Context;

pub fn launch(ctx: &mut Context, name: &[u8]) {
    if let Some(pid) = ctx.installed_pids.get(name).copied() {
        if focus(pid) {
            return;
        }
        ctx.installed_pids.remove(name);
    }
    if let Some(pid) = load_by_name(name) {
        ctx.installed_pids.insert(name.to_vec(), pid);
    }
}

/// A send failure means the pid is gone or no longer takes control frames, so
/// the caller falls back to loading the app afresh.
fn focus(pid: u32) -> bool {
    let frame = focus_frame();
    mk_ipc_send_to_pid(pid, frame.as_ptr(), frame.len()) >= 0
}
