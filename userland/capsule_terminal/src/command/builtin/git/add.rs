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
//! `git add <path>...`

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use nonos_git::{add, Mode};

use crate::command::output::Output;
use crate::term::state::State;

use super::repo::{storage, GIT_DIR};

pub(super) fn run(state: &mut State, argv: &[&[u8]]) {
    if argv.is_empty() {
        Output::new(&mut state.scrollback).writeln(b"git add: missing path");
        return;
    }
    for arg in argv {
        let Ok(text) = core::str::from_utf8(arg) else {
            Output::new(&mut state.scrollback).writeln(b"git add: path not utf8");
            continue;
        };
        let mut s = storage(state);
        // The index records paths relative to the work tree, so an absolute
        // argument is trimmed back to one before it is staged.
        let rel = relative(s.root(), text);
        if rel.is_empty() || add(&mut s, GIT_DIR, &rel, Mode::File).is_err() {
            let mut line = Vec::from(&b"git add: cannot stage "[..]);
            line.extend_from_slice(arg);
            Output::new(&mut state.scrollback).writeln(&line);
        }
    }
}

/// `path` expressed relative to `root`, with no leading slash.
fn relative(root: &str, path: &str) -> String {
    let base = root.trim_end_matches('/');
    let trimmed = path.strip_prefix(base).unwrap_or(path);
    String::from(trimmed.trim_start_matches('/'))
}
