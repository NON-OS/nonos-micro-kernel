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
//! `git status`

extern crate alloc;

use alloc::vec::Vec;

use nonos_git::{read_index, resolve_head};

use crate::command::output::Output;
use crate::term::state::State;

use super::repo::{storage, GIT_DIR};

pub(super) fn run(state: &mut State) {
    let s = storage(state);
    let head = resolve_head(&s, GIT_DIR);
    let staged = read_index(&s, GIT_DIR);
    let mut out = Output::new(&mut state.scrollback);

    let Ok(entries) = staged else {
        out.writeln(b"git status: not a repository");
        return;
    };

    out.writeln(b"On branch main");
    if head.is_none() {
        out.writeln(b"No commits yet");
    }
    if entries.is_empty() {
        out.writeln(b"nothing staged");
        return;
    }
    out.writeln(b"Changes to be committed:");
    for e in entries {
        let mut line = Vec::from(&b"    "[..]);
        line.extend_from_slice(e.path.as_bytes());
        out.writeln(&line);
    }
}
