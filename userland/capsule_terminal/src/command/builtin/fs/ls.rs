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

//! List a directory. Shows the immediate children of the target (the working
//! directory by default), directories suffixed with a slash, one per line.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;

use super::{abspath, pid};
use crate::command::output::Output;
use crate::term::state::State;

pub fn ls(state: &mut State, argv: &[&[u8]]) {
    let arg = argv.get(1).copied().unwrap_or(b".");
    let mut dir = abspath(state, arg);
    if dir.last() != Some(&b'/') {
        dir.push(b'/');
    }
    let owner = pid(state);
    let paths = match vfs::list_paths(owner, &dir) {
        Ok(p) => p,
        Err(e) => {
            Output::new(&mut state.scrollback).writeln(e.as_bytes());
            return;
        }
    };
    let mut seen: Vec<Vec<u8>> = Vec::new();
    for p in &paths {
        let bytes = p.as_bytes();
        let Some(rest) = bytes.strip_prefix(dir.as_slice()) else { continue };
        if rest.is_empty() {
            continue;
        }
        // First path segment under the directory is the direct child.
        let cut = rest.iter().position(|&c| c == b'/').unwrap_or(rest.len());
        let mut name = rest[..cut].to_vec();
        if cut < rest.len() || bytes.last() == Some(&b'/') {
            name.push(b'/');
        }
        if !name.is_empty() && !seen.contains(&name) {
            seen.push(name);
        }
    }
    if seen.is_empty() {
        return;
    }
    let mut out = Output::new(&mut state.scrollback);
    for name in &seen {
        out.writeln(name);
    }
}
