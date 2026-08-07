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

use nonos_libc::{mk_ipc_send_to_pid, mk_service_lookup, mk_spawn_instance};

use crate::state::apps::LauncherApp;

/// What a dock-launch click actually did, so the caller can surface it on
/// screen. `Queued` means the kernel accepted a new-window spawn; `Focused`
/// means the slot table was full and the running window was raised instead.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LaunchOutcome {
    Queued,
    Focused,
    Failed,
}

const CONTROL_LEN: usize = 8;
const CONTROL_MAGIC: u32 = u32::from_le_bytes(*b"NCTL");
const CONTROL_VERSION: u16 = 1;
const OP_FOCUS_SELF: u16 = 1;

// Services that own a single device-backed session, so a second window
// would fight the first over the same hardware stream rather than give
// the user anything new.
const SINGLE_INSTANCE: [&[u8]; 0] = [];

// Clicking a dock app asks the kernel to spawn another attested instance.
// The kernel queues the request and init performs the spawn in its own
// context, because doing it inline in this shell's syscall corrupted the
// caller. A click with a free instance slot returns ok and a new window
// appears a tick later; when every declared slot is live the kernel returns
// an error and we focus the running instance instead, so a click is never a
// dead end. Single-instance services skip the spawn and always focus.
pub fn request(app: &LauncherApp) -> LaunchOutcome {
    request_service(app.service)
}

/// Launch, or focus if already running, whatever capsule owns `service`. Used
/// by both the dock (a desktop app) and the Launchpad (an installed tool).
pub fn request_service(service: &[u8]) -> LaunchOutcome {
    if !is_single_instance(service) && mk_spawn_instance(service) >= 0 {
        return LaunchOutcome::Queued;
    }
    focus_service(service)
}

/// Focus whatever already owns `service`, spawning nothing.
///
/// The dock uses this when it knows the app is running. Going through
/// `request_service` spawned a fresh window on every click and left a
/// minimized one hidden, which made it unreachable: this message is the only
/// thing that reaches `wm::window_restore`.
pub fn focus_service(service: &[u8]) -> LaunchOutcome {
    let Some(pid) = lookup_pid(service) else { return LaunchOutcome::Failed };
    let frame = focus_frame();
    if mk_ipc_send_to_pid(pid, frame.as_ptr(), frame.len()) >= 0 {
        LaunchOutcome::Focused
    } else {
        LaunchOutcome::Failed
    }
}

fn is_single_instance(service: &[u8]) -> bool {
    SINGLE_INSTANCE.iter().any(|s| *s == service)
}

pub(crate) fn lookup_pid(service: &[u8]) -> Option<u32> {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(service.as_ptr(), service.len(), &mut port, &mut pid);
    if rc < 0 || pid == 0 {
        return None;
    }
    Some(pid)
}

fn focus_frame() -> [u8; CONTROL_LEN] {
    let mut frame = [0u8; CONTROL_LEN];
    frame[0..4].copy_from_slice(&CONTROL_MAGIC.to_le_bytes());
    frame[4..6].copy_from_slice(&CONTROL_VERSION.to_le_bytes());
    frame[6..8].copy_from_slice(&OP_FOCUS_SELF.to_le_bytes());
    frame
}
