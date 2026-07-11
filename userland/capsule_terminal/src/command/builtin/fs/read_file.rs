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

//! Shared helper: read a file argument into bytes, or report the error.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;

use super::{abspath, pid};
use crate::command::output::Output;
use crate::term::state::State;

const MAX_READ: u32 = 256 * 1024;

pub(super) fn slurp(state: &mut State, arg: &[u8]) -> Option<Vec<u8>> {
    let path = abspath(state, arg);
    let owner = pid(state);
    match vfs::read_file(owner, &path, MAX_READ) {
        Ok(b) => Some(b),
        Err(e) => {
            Output::new(&mut state.scrollback).writeln(e.as_bytes());
            None
        }
    }
}
