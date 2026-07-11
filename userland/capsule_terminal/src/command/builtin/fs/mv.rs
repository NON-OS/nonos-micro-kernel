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

//! Rename or move a path.

use nonos_app_skeleton::clients::vfs;

use super::{abspath, pid};
use crate::command::output::Output;
use crate::term::state::State;

pub fn mv(state: &mut State, argv: &[&[u8]]) {
    if argv.len() != 3 {
        Output::new(&mut state.scrollback).writeln(b"usage: mv <src> <dst>");
        return;
    }
    let src = abspath(state, argv[1]);
    let dst = abspath(state, argv[2]);
    let owner = pid(state);
    if let Err(e) = vfs::rename(owner, &src, &dst) {
        Output::new(&mut state.scrollback).writeln(e.as_bytes());
    }
}
