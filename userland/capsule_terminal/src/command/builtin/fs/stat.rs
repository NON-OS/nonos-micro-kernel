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

//! Show a path's type and size.

use nonos_app_skeleton::clients::vfs;

use super::{abspath, pid};
use crate::command::output::Output;
use crate::term::state::State;
use crate::term::util::format_u64;

pub fn stat(state: &mut State, argv: &[&[u8]]) {
    if argv.len() < 2 {
        Output::new(&mut state.scrollback).writeln(b"stat: missing path");
        return;
    }
    let path = abspath(state, argv[1]);
    let owner = pid(state);
    match vfs::stat(owner, &path) {
        Ok((size, is_dir)) => {
            let mut line = alloc::vec::Vec::new();
            line.extend_from_slice(if is_dir { b"dir  " } else { b"file " });
            let mut num = [0u8; 20];
            let n = format_u64(size, &mut num);
            line.extend_from_slice(&num[..n]);
            line.extend_from_slice(if is_dir { b" entries  " } else { b" bytes  " });
            line.extend_from_slice(&path);
            Output::new(&mut state.scrollback).writeln(&line);
        }
        Err(e) => Output::new(&mut state.scrollback).writeln(e.as_bytes()),
    }
}
