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

//! A guest asleep inside the syscall it made.

use super::trap_table::take_answer;

pub(super) fn wait_for_answer(pid: u32) -> u64 {
    loop {
        if let Some(value) = take_answer(pid) {
            return settle(pid, value);
        }
        let token = crate::sched::wake_token(pid);
        if let Some(value) = take_answer(pid) {
            return settle(pid, value);
        }
        crate::sched::sleep_until_unless_woken(pid, u64::MAX, token);
        crate::sched::yield_now();
    }
}

/// Every answer but one is a return value.
fn settle(pid: u32, value: u64) -> u64 {
    if value == super::exec::EXECED {
        super::exec_enter::enter(pid)
    }
    value
}
