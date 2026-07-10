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

//! Change the working directory. With no argument, go to root.

use nonos_app_skeleton::clients::vfs;

use super::{abspath, pid};
use crate::command::output::Output;
use crate::term::state::State;

pub fn cd(state: &mut State, argv: &[&[u8]]) {
    let target = argv.get(1).copied().unwrap_or(b"/");
    let path = abspath(state, target);
    let owner = pid(state);
    if path == b"/" {
        state.cwd.set(path);
        return;
    }
    match vfs::stat(owner, &path) {
        Ok((_, true)) => state.cwd.set(path),
        Ok((_, false)) => Output::new(&mut state.scrollback).writeln(b"cd: not a directory"),
        Err(_) => Output::new(&mut state.scrollback).writeln(b"cd: no such directory"),
    }
}
