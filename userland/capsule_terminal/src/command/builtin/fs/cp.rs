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

//! Copy a path. A directory needs -r.

use nonos_app_skeleton::clients::vfs;

use super::{abspath, pid};
use crate::command::flags::{parse, Spec};
use crate::command::output::Output;
use crate::term::state::State;

pub fn cp(state: &mut State, argv: &[&[u8]]) {
    let parsed = match parse(&Spec::new(b"cp", b"r"), &argv[1..]) {
        Ok(p) => p,
        Err(e) => return Output::new(&mut state.scrollback).writeln(&e),
    };
    if parsed.operands.len() != 2 {
        Output::new(&mut state.scrollback).writeln(b"usage: cp [-r] <src> <dst>");
        return;
    }
    let src = abspath(state, parsed.operands[0]);
    let dst = abspath(state, parsed.operands[1]);
    let owner = pid(state);
    if let Err(e) = vfs::copy(owner, &src, &dst, parsed.has(b'r')) {
        Output::new(&mut state.scrollback).writeln(e.as_bytes());
    }
}
