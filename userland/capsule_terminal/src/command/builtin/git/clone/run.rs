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
//! Running a clone.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_git::clone;
use nonos_tls::rtc_now;

use crate::command::output::Output;
use crate::git::{Https, Remote};
use crate::term::state::State;

use super::super::repo::storage;
use super::fail::fail_with;

/// A clone stops at the tip by default. Whole histories are large and the
/// terminal has no way to show progress across one, so the depth is stated
/// rather than left to run.
const DEPTH: u32 = 1;

pub(in crate::command::builtin::git) fn run(state: &mut State, argv: &[&[u8]]) {
    let Some(url) = argv.first().and_then(|a| core::str::from_utf8(a).ok()) else {
        Output::new(&mut state.scrollback).writeln(b"usage: git clone <https url> [branch]");
        return;
    };
    let Some(remote) = Remote::parse(url) else {
        Output::new(&mut state.scrollback).writeln(b"git clone: only https urls are supported");
        return;
    };
    let branch = argv.get(1).and_then(|a| core::str::from_utf8(a).ok()).unwrap_or("main");

    let into = String::from(remote.name());
    let git_dir = format!("{into}/.git");
    let work_tree = format!("{into}/");
    let mut transport = Https::new(remote, rtc_now());
    let mut s = storage(state);

    match clone(&mut transport, &mut s, &git_dir, &work_tree, branch, DEPTH) {
        Ok(files) => {
            let mut line = Vec::from(&b"Cloned into "[..]);
            line.extend_from_slice(into.as_bytes());
            line.extend_from_slice(b", ");
            line.extend_from_slice(format!("{files}").as_bytes());
            line.extend_from_slice(b" files");
            Output::new(&mut state.scrollback).writeln(&line);
        }
        Err(e) => fail_with(state, "git clone", e),
    }
}
