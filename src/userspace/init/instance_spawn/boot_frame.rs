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

/// Boot a freshly spawned window instance by delivering the focus frame the
/// app skeleton waits for. The frame is attributed to the desktop shell, the
/// only sender the skeleton accepts, because the shell is what asked for this
/// window.
///
/// Delivery is retried until it lands or a wall-clock deadline passes, not a
/// fixed number of yields. The instance's `proc.<pid>` inbox is created by its
/// install and the shell must be up to be the sender, but on real hardware
/// either can lag the spawn return; a fixed count of bare yields resolves in
/// microseconds on a lightly loaded scheduler, so it gave up before the instance
/// had even been scheduled to create its inbox. That is why a second or third
/// window opened reliably under QEMU (steady, fast scheduling) yet not on
/// hardware. Bounding by the monotonic clock (which does not depend on the timer
/// tick) gives the instance real time to appear while still capping the wait so a
/// genuinely missing shell can never wedge the init drain.
const BOOT_DEADLINE_MS: u64 = 1000;

pub(super) fn boot(instance_pid: u32) {
    let to = format!("proc.{}", instance_pid);
    let deadline = crate::time::timestamp_millis().saturating_add(BOOT_DEADLINE_MS);
    while crate::time::timestamp_millis() < deadline {
        if try_deliver(&to) {
            return;
        }
        crate::sched::yield_now();
    }
}

// Deliver the focus frame once. Returns true only when the shell is registered,
// the instance inbox exists, and the frame was accepted, so the caller knows
// whether to keep retrying.
fn try_deliver(to: &str) -> bool {
    let Some(shell) = lookup_service(DESKTOP_SHELL) else {
        return false;
    };
    if !nonos_inbox::exists(to) {
        return false;
    }
    let from = format!("proc.{}", shell.pid);
    match IpcMessage::new(&from, to, &NCTL_FOCUS_SELF) {
        Ok(msg) => nonos_inbox::try_enqueue_strict(to, msg).is_ok(),
        Err(_) => false,
    }
}
