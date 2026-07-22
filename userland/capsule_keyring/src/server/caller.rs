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

// The kernel router stamps proc.<pid> on every capsule-originated message,
// so sender 0 can only mean a kernel-internal delivery. No key operation
// accepts one: trusting a payload-asserted pid there would let any future
// kernel relay path speak for any owner. The asserted pid must equal the pid
// the kernel itself attributes to the message, nothing else.
pub(super) fn resolve_caller(payload_pid: u32, sender_pid: u32) -> Option<u32> {
    if sender_pid == 0 {
        return None;
    }
    if payload_pid == sender_pid {
        return Some(sender_pid);
    }
    None
}
