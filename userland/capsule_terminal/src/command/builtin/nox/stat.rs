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
use nonos_app_skeleton::clients::vfs::stat;

use super::ensure_pid::ensure_pid;
use crate::term::cwd::resolve;
use crate::term::state::State;
use crate::term::util::format_u64;

pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    if args.is_empty() {
        state.scrollback.push_line(b"usage: nox stat <path>");
        return false;
    }
    let pid = ensure_pid(state);
    let path = resolve(state.cwd.as_bytes(), args[0]);
    match stat(pid, &path) {
        Ok((size, is_dir)) => {
            let mut line: Vec<u8> = Vec::new();
            line.extend_from_slice(if is_dir { b"dir   " } else { b"file  " });
            let mut num = [0u8; 24];
            let n = format_u64(size, &mut num);
            line.extend_from_slice(&num[..n]);
            line.extend_from_slice(b" bytes  ");
            line.extend_from_slice(&path);
            state.scrollback.push_line(&line);
            true
        }
        Err(e) => {
            state.scrollback.push_line(e.as_bytes());
            false
        }
    }
}
