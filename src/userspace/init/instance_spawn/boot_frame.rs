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

extern crate alloc;

use alloc::format;

use crate::ipc::nonos_channel::IpcMessage;
use crate::ipc::nonos_inbox;
use crate::services::registry::lookup_service;

// The app skeleton stays idle until the desktop shell sends it this control
// frame (NCTL, version 1, op "focus self"); only then does an app build its
// window and register a surface. A base app gets the frame when the user
// first clicks its dock icon. A spawned instance was never sent one, so it
// ran but never drew. Delivering the same frame here, attributed to the
// shell that requested the spawn, boots the instance so its window appears.
const NCTL_FOCUS_SELF: [u8; 8] = [b'N', b'C', b'T', b'L', 1, 0, 1, 0];

const DESKTOP_SHELL: &str = "desktop_shell";

/// How long to keep retrying the focus-frame delivery before giving up.
const BOOT_DEADLINE_MS: u64 = 1000;

/// Boot a freshly spawned window instance by delivering the focus frame the
/// app skeleton waits for. The frame is attributed to the desktop shell, the
/// only sender the skeleton accepts, because the shell is what asked for this
/// window. A missing shell just means no window yet, never a fault.
///
/// The instance's `proc.<pid>` inbox is created by its install, which can lag the
/// spawn return by a few scheduler passes on real hardware; a single attempt then
/// drops the frame and the window never draws. That timing is why a second or
/// third window opened reliably under QEMU (steady, fast scheduling) yet not on
/// hardware. Retry until the frame lands or a bounded wall-clock deadline passes,
/// so the instance has real time to register its inbox while the wait stays
/// capped and a genuinely missing instance never wedges the init drain.
pub(super) fn boot(instance_pid: u32) {
    let Some(shell) = lookup_service(DESKTOP_SHELL) else {
        return;
    };
    let from = format!("proc.{}", shell.pid);
    let to = format!("proc.{}", instance_pid);
    let deadline = crate::time::timestamp_millis().saturating_add(BOOT_DEADLINE_MS);
    loop {
        if let Ok(msg) = IpcMessage::new(&from, &to, &NCTL_FOCUS_SELF) {
            if nonos_inbox::try_enqueue_strict(&to, msg).is_ok() {
                return;
            }
        }
        if crate::time::timestamp_millis() >= deadline {
            return;
        }
        crate::sched::yield_now();
    }
}
