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
use nonos_app_skeleton::clients::vfs::{list_paths, stat};

use super::ensure_pid::ensure_pid;
use crate::term::cwd::{dir_prefix, resolve};
use crate::term::state::State;
use crate::term::util::format_u64;

pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    let pid = ensure_pid(state);
    let base = if args.is_empty() {
        state.cwd.as_bytes().to_vec()
    } else {
        resolve(state.cwd.as_bytes(), args[0])
    };
    let prefix = dir_prefix(&base);
    let paths = match list_paths(pid, &prefix) {
        Ok(p) => p,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return false;
        }
    };
    let mut total = 0u64;
    for p in &paths {
        if let Ok((size, false)) = stat(pid, p.as_bytes()) {
            total += size;
        }
    }
    let mut line: Vec<u8> = Vec::new();
    let mut num = [0u8; 24];
    let n = format_u64(total, &mut num);
    line.extend_from_slice(&num[..n]);
    line.extend_from_slice(b" bytes  ");
    line.extend_from_slice(&prefix);
    state.scrollback.push_line(&line);
    true
}
