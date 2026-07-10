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

//! List every path under a directory, recursively. The vfs returns the whole
//! subtree for a prefix, so this just prints it.

use nonos_app_skeleton::clients::vfs;

use super::{abspath, pid};
use crate::command::output::Output;
use crate::term::state::State;

pub fn find(state: &mut State, argv: &[&[u8]]) {
    let arg = argv.get(1).copied().unwrap_or(b".");
    let mut dir = abspath(state, arg);
    if dir.last() != Some(&b'/') {
        dir.push(b'/');
    }
    let owner = pid(state);
    match vfs::list_paths(owner, &dir) {
        Ok(paths) => {
            let mut out = Output::new(&mut state.scrollback);
            for p in &paths {
                out.writeln(p.as_bytes());
            }
        }
        Err(e) => Output::new(&mut state.scrollback).writeln(e.as_bytes()),
    }
}
