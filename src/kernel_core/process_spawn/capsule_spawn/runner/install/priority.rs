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

use crate::process::core::Priority;

/// Capsules on the pointer/keyboard -> compositor -> scanout path.
///
/// Each one parks in the kernel when it has nothing to do (`mk_irq_wait`,
/// `mk_input_event_wait`, `mk_ipc_recv_from` with a timeout), so the band is
/// empty whenever the desktop is idle. `select_by_priority` also skips the
/// running process, so a band holding a single runnable member always falls
/// through to `Normal` on the next switch - a promoted capsule that spun
/// would cost half the CPU, never all of it.
const INTERACTIVE: [&str; 4] =
    ["driver.ps2_kbd0", "input_router", "compositor", "driver.virtio_gpu0"];

/// Scheduling band a freshly installed capsule starts in.
pub(super) fn for_capsule(name: &str) -> Priority {
    if INTERACTIVE.contains(&name) {
        Priority::High
    } else {
        Priority::Normal
    }
}
