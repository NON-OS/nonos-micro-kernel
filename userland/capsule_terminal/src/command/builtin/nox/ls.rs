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

use nonos_app_skeleton::clients::vfs::list_paths;

use super::children::children;
use super::ensure_pid::ensure_pid;
use crate::term::cwd::{dir_prefix, resolve};
use crate::term::state::State;

pub fn run(state: &mut State, args: &[&[u8]]) {
    let pid = ensure_pid(state);
    let base = if args.is_empty() {
        state.cwd.as_bytes().to_vec()
    } else {
        resolve(state.cwd.as_bytes(), args[0])
    };
    let prefix = dir_prefix(&base);
    let prefix_str = match core::str::from_utf8(&prefix) {
        Ok(s) => s,
        Err(_) => return state.scrollback.push_line(b"nox ls: bad path"),
    };
    match list_paths(pid, &prefix) {
        Ok(paths) => {
            let kids = children(prefix_str, &paths);
            if kids.is_empty() {
                state.scrollback.push_line(b"(empty)");
            }
            for k in &kids {
                state.scrollback.push_line(k.as_bytes());
            }
        }
        Err(e) => state.scrollback.push_line(e.as_bytes()),
    }
}
