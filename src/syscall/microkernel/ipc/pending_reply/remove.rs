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

use super::state::PENDING;

pub(in crate::syscall::microkernel::ipc) fn remove(server_pid: u32, caller_inbox: &str) -> bool {
    let mut map = PENDING.lock();
    let Some(queue) = map.get_mut(&server_pid) else {
        return false;
    };
    let Some(pos) = queue.iter().position(|(_, inbox, _)| inbox == caller_inbox) else {
        return false;
    };
    queue.remove(pos);
    if queue.is_empty() {
        map.remove(&server_pid);
    }
    true
}
