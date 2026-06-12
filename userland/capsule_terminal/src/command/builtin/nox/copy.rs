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

use nonos_app_skeleton::clients::vfs::{read_file, write_file};

use super::ensure_pid::ensure_pid;
use crate::term::cwd::resolve;
use crate::term::state::State;

const MAX: u32 = 65536;

pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    if args.len() < 2 {
        state.scrollback.push_line(b"usage: nox copy <src> <dst>");
        return false;
    }
    let pid = ensure_pid(state);
    let src = resolve(state.cwd.as_bytes(), args[0]);
    let dst = resolve(state.cwd.as_bytes(), args[1]);
    let data = match read_file(pid, &src, MAX) {
        Ok(d) => d,
        Err(e) => {
            state.scrollback.push_line(e.as_bytes());
            return false;
        }
    };
    match write_file(pid, &dst, &data) {
        Ok(()) => {
            state.scrollback.push_line(b"ok");
            true
        }
        Err(e) => {
            state.scrollback.push_line(e.as_bytes());
            false
        }
    }
}
