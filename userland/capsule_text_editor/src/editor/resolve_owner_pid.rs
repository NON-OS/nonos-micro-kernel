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

use nonos_libc::mk_getpid;

use super::state::State;

pub(super) fn resolve_owner_pid(state: &mut State) -> bool {
    if state.owner_pid == 0 {
        // Authoritative own pid: the vfs server rejects a claimed owner pid that
        // differs from the real sender pid, so resolving it from a service
        // lookup (which can miss) makes every open/read fail as EACCES.
        state.owner_pid = mk_getpid();
    }
    state.owner_pid != 0
}
