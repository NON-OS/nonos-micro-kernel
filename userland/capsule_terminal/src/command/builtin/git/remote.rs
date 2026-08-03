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
//! Showing and setting the origin remote.

extern crate alloc;

use alloc::vec::Vec;

use nonos_git::{remote_url, set_remote};

use crate::command::output::Output;
use crate::term::state::State;

use super::repo::{storage, GIT_DIR};

pub(super) fn run(state: &mut State, argv: &[&[u8]]) {
    let url = argv.get(1).and_then(|a| core::str::from_utf8(a).ok());
    match (argv.first().copied(), url) {
        (Some(b"set-url"), Some(url)) => set(state, url),
        (Some(b"get-url"), _) | (None, _) => show(state),
        _ => {
            Output::new(&mut state.scrollback)
                .writeln(b"usage: git remote [get-url | set-url <https url>]");
        }
    }
}

fn show(state: &mut State) {
    let s = storage(state);
    match remote_url(&s, GIT_DIR, "origin") {
        Some(url) => {
            let mut line = Vec::from(&b"origin\t"[..]);
            line.extend_from_slice(url.as_bytes());
            Output::new(&mut state.scrollback).writeln(&line);
        }
        None => Output::new(&mut state.scrollback).writeln(b"git remote: no origin set"),
    }
}

fn set(state: &mut State, url: &str) {
    let mut s = storage(state);
    match set_remote(&mut s, GIT_DIR, "origin", url) {
        Ok(()) => Output::new(&mut state.scrollback).writeln(b"origin set"),
        Err(_) => Output::new(&mut state.scrollback).writeln(b"git remote: cannot write config"),
    }
}
