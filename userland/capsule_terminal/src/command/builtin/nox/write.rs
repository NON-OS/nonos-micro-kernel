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

use alloc::vec::Vec;
use nonos_app_skeleton::clients::vfs::write_file;

use super::ensure_pid::ensure_pid;
use crate::term::cwd::resolve;
use crate::term::state::State;

pub fn run(state: &mut State, args: &[&[u8]]) {
    if args.len() < 2 {
        return state.scrollback.push_line(b"usage: nox write <file> <text>");
    }
    let pid = ensure_pid(state);
    let path = resolve(state.cwd.as_bytes(), args[0]);
    let mut data = Vec::new();
    for (i, word) in args[1..].iter().enumerate() {
        if i > 0 {
            data.push(b' ');
        }
        data.extend_from_slice(word);
    }
    data.push(b'\n');
    match write_file(pid, &path, &data) {
        Ok(()) => state.scrollback.push_line(b"ok"),
        Err(e) => state.scrollback.push_line(e.as_bytes()),
    }
}
