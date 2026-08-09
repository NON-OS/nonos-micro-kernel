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

use super::summary::slug;
use super::{args, call, emit, manage};
use crate::term::cwd::resolve;
use crate::term::state::State;

pub(super) const USAGE: &[u8] = b"usage: nox pkg install <path> [--yes] | remove <name> | status";

pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    match args.first().copied() {
        Some(b"install") => install(state, &args[1..]),
        Some(b"remove") => manage::remove(state, &args[1..]),
        Some(b"status") => manage::status(state),
        _ => {
            state.scrollback.push_error(USAGE);
            false
        }
    }
}

// Two-step consent: the bare form only verifies the package and prints what
// it would be granted, and nothing is written until the user repeats the
// command with --yes. The commit carries the digest from the query, so a
// package swapped in between the two steps is rejected rather than installed.
fn install(state: &mut State, rest: &[&[u8]]) -> bool {
    let Some((raw, yes)) = args::install(rest) else {
        state.scrollback.push_error(USAGE);
        return false;
    };
    let path = resolve(state.cwd.as_bytes(), raw);
    let s = match call::pkg_query(&path) {
        Ok(s) => s,
        Err(status) => {
            emit::error(state, status);
            return false;
        }
    };
    emit::summary(state, &s);
    if !yes {
        state.scrollback.push_line(b"run again with --yes to install");
        return true;
    }
    match call::pkg_commit(&path, &s.digest) {
        Ok(()) => {
            let name = slug(&s.namespace);
            let mut line = Vec::with_capacity(10 + name.len());
            line.extend_from_slice(b"installed ");
            line.extend_from_slice(name);
            state.scrollback.push_line(&line);
            true
        }
        Err(status) => {
            emit::error(state, status);
            false
        }
    }
}
