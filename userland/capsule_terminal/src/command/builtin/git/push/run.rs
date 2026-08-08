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
//! Running a push.

extern crate alloc;

use alloc::format;
use alloc::vec::Vec;

use nonos_git::{push, read_head, resolve_head, Head};
use nonos_tls::rtc_now;

use crate::command::output::Output;
use crate::git::{Https, Remote};
use crate::term::state::State;

use super::super::clone::fail_with;
use super::super::repo::{storage, GIT_DIR};

pub(in crate::command::builtin::git) fn run(state: &mut State, argv: &[&[u8]]) {
    let Some(url) = argv.first().and_then(|a| core::str::from_utf8(a).ok()) else {
        Output::new(&mut state.scrollback).writeln(b"usage: git push <https url>");
        return;
    };
    let Some(remote) = Remote::parse(url) else {
        Output::new(&mut state.scrollback).writeln(b"git push: only https urls are supported");
        return;
    };

    let s = storage(state);
    let Some(Head::Branch(branch)) = read_head(&s, GIT_DIR) else {
        Output::new(&mut state.scrollback).writeln(b"git push: HEAD is not on a branch");
        return;
    };
    let Some(head) = resolve_head(&s, GIT_DIR) else {
        Output::new(&mut state.scrollback).writeln(b"git push: nothing committed yet");
        return;
    };

    let full = format!("refs/heads/{branch}");
    let mut transport = Https::new(remote, rtc_now());
    match push(&mut transport, &s, GIT_DIR, &head, &full) {
        Ok(()) => {
            let mut line = Vec::from(&b"Pushed "[..]);
            line.extend_from_slice(full.as_bytes());
            Output::new(&mut state.scrollback).writeln(&line);
        }
        Err(e) => fail_with(state, "git push", e),
    }
}
