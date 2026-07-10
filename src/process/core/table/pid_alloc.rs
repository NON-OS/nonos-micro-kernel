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

//! Pure PID-selection arithmetic, with no kernel dependencies so it can be
//! exercised directly by a host proof. `allocate_tid` calls this under the
//! allocation lock; the caller stores `next` back into the counter.

const MAX_ATTEMPTS: u32 = 65536;

/// Choose the next PID to hand out starting from counter value `current`,
/// skipping any PID the `is_active` predicate reports as still in use, and
/// return `(pid, next_counter)`. Returns `None` if no free PID is found within
/// `MAX_ATTEMPTS` probes. The chosen PID is never 0, and the counter wraps to 1
/// at the top of the u32 range rather than reaching 0.
pub(crate) fn choose_pid(mut current: u32, is_active: impl Fn(u32) -> bool) -> Option<(u32, u32)> {
    for _ in 0..MAX_ATTEMPTS {
        let next = if current >= u32::MAX - 1 { 1 } else { current + 1 };
        let pid = if current == 0 { 1 } else { current };
        if !is_active(pid) {
            return Some((pid, next));
        }
        current = next;
    }
    None
}
