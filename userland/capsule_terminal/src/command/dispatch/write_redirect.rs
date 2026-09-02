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
use nonos_app_skeleton::clients::vfs::{read_file, write_file};

use crate::term::cwd::resolve;
use crate::term::state::State;

const MAX_APPEND: u32 = 65536;

// Write a captured command's output to a VFS file. `>` overwrites; `>>`
// reads the existing file first and appends. The terminal owner pid is
// resolved once and cached on the state, the same way the file commands
// do it.
pub(super) fn write_redirect(state: &mut State, lines: &[Vec<u8>], append: bool, path_arg: &[u8]) {
    let pid = state.owner_pid;
    let path = resolve(state.cwd.as_bytes(), path_arg);
    let mut data = Vec::new();
    if append {
        if let Ok(existing) = read_file(pid, &path, MAX_APPEND) {
            data.extend_from_slice(&existing);
        }
    }
    for line in lines {
        data.extend_from_slice(line);
        data.push(b'\n');
    }
    match write_file(pid, &path, &data) {
        Ok(()) => {
            let mut msg = Vec::with_capacity(9 + path.len());
            msg.extend_from_slice(b"wrote to ");
            msg.extend_from_slice(&path);
            state.scrollback.push_line(&msg);
        }
        Err(e) => state.scrollback.push_line(e.as_bytes()),
    }
}
