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

//! Create one or more directories; -p also creates the parents.

use nonos_app_skeleton::clients::vfs;

use super::{abspath, pid};
use crate::command::flags::{parse, Spec};
use crate::command::output::Output;
use crate::term::state::State;

pub fn mkdir(state: &mut State, argv: &[&[u8]]) {
    let parsed = match parse(&Spec::new(b"mkdir", b"p"), &argv[1..]) {
        Ok(p) => p,
        Err(e) => return Output::new(&mut state.scrollback).writeln(&e),
    };
    if parsed.operands.is_empty() {
        Output::new(&mut state.scrollback).writeln(b"mkdir: missing name");
        return;
    }
    let parents = parsed.has(b'p');
    for arg in parsed.operands {
        let path = abspath(state, arg);
        let owner = pid(state);
        if parents {
            make_parents(owner, &path);
            let _ = vfs::mkdir(owner, &path);
            continue;
        }
        if let Err(e) = vfs::mkdir(owner, &path) {
            Output::new(&mut state.scrollback).writeln(e.as_bytes());
        }
    }
}

fn make_parents(owner: u32, path: &[u8]) {
    for i in 1..path.len() {
        if path[i] == b'/' {
            let _ = vfs::mkdir(owner, &path[..i]);
        }
    }
}
