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

use super::caller_has_register_right::caller_has_register_right;
use super::owner_has_required::owner_has_required;

pub(in crate::services::registry) fn caller_can_register(owner_pid: u32, required: u64) -> bool {
    if !owner_has_required(owner_pid, required) {
        return false;
    }
    // Kernel-context registration (boot-time trusted services) has no current
    // pid and is allowed. Every capsule with a pid must hold the register right;
    // the old `pid <= 64` shortcut handed unauthenticated registration authority
    // to any capsule spawned early enough to land in that range, which let an
    // ordinary app squat/impersonate a service name.
    match crate::process::current_pid() {
        None => true,
        Some(_) => caller_has_register_right(),
    }
}
