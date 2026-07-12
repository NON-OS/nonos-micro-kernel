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

//! Print the contents of one or more files.

use nonos_app_skeleton::clients::vfs;

use super::{abspath, pid};
use crate::command::output::Output;
use crate::term::state::State;

const MAX_READ: u32 = 256 * 1024;

pub fn cat(state: &mut State, argv: &[&[u8]]) {
    if argv.len() < 2 {
        Output::new(&mut state.scrollback).writeln(b"cat: missing file");
        return;
    }
    for arg in &argv[1..] {
        let path = abspath(state, arg);
        let owner = pid(state);
        match vfs::read_file(owner, &path, MAX_READ) {
            Ok(bytes) => {
                let mut out = Output::new(&mut state.scrollback);
                for line in bytes.split(|&b| b == b'\n') {
                    out.writeln(line);
                }
            }
            Err(e) => Output::new(&mut state.scrollback).writeln(e.as_bytes()),
        }
    }
}
