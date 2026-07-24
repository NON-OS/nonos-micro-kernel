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

use alloc::string::String;

use super::state::{MAX_PER_SERVICE, PENDING};

pub(in crate::syscall::microkernel::ipc) fn push(
    server_pid: u32,
    caller_inbox: String,
    token: u64,
) -> bool {
    let mut map = PENDING.lock();
    let queue = map.entry(server_pid).or_default();
    if queue.len() < MAX_PER_SERVICE {
        queue.push_back((caller_inbox, token));
        true
    } else {
        false
    }
}
