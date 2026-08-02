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
//! `git init`

use nonos_git::init;

use crate::command::output::Output;
use crate::term::state::State;

use super::repo::{storage, GIT_DIR};

pub(super) fn run(state: &mut State) {
    let mut s = storage(state);
    let result = init(&mut s, GIT_DIR, "main");
    let mut out = Output::new(&mut state.scrollback);
    match result {
        Ok(()) => out.writeln(b"Initialized empty repository on branch main"),
        Err(nonos_git::RepoError::Exists) => out.writeln(b"git: already a repository"),
        Err(_) => out.writeln(b"git: could not create repository"),
    }
}
