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
//! `git log`

extern crate alloc;

use alloc::vec::Vec;

use nonos_git::log;

use crate::command::output::Output;
use crate::term::state::State;

use super::repo::{storage, GIT_DIR};

/// Bound on how far back one invocation walks.
const MAX: usize = 100;

pub(super) fn run(state: &mut State) {
    let s = storage(state);
    let entries = log(&s, GIT_DIR, MAX);
    let mut out = Output::new(&mut state.scrollback);
    match entries {
        Ok(e) if e.is_empty() => out.writeln(b"git log: no commits yet"),
        Ok(entries) => {
            for e in entries {
                let mut line = Vec::from(&b"commit "[..]);
                line.extend_from_slice(e.id.to_hex().as_bytes());
                out.writeln(&line);

                let mut who = Vec::from(&b"Author: "[..]);
                who.extend_from_slice(e.commit.author.name.as_bytes());
                who.extend_from_slice(b" <");
                who.extend_from_slice(e.commit.author.email.as_bytes());
                who.push(b'>');
                out.writeln(&who);

                let mut msg = Vec::from(&b"    "[..]);
                msg.extend_from_slice(e.commit.message.trim_end().as_bytes());
                out.writeln(&msg);
            }
        }
        Err(_) => out.writeln(b"git log: not a repository"),
    }
}
