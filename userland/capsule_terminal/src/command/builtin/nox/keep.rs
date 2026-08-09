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

use nonos_app_skeleton::clients::vfs::persist;

use super::ensure_pid::ensure_pid;
use crate::term::cwd::resolve;
use crate::term::state::State;

pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    if args.is_empty() {
        state.scrollback.push_error(b"usage: nox keep <path>");
        return false;
    }
    let pid = ensure_pid(state);
    let path = resolve(state.cwd.as_bytes(), args[0]);
    match persist(pid, &path) {
        Ok(()) => {
            state.scrollback.push_line(b"persisted");
            true
        }
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            false
        }
    }
}
