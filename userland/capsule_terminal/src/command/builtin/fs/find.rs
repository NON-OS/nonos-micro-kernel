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

//! List every path under a directory, recursively. The vfs returns the whole
//! subtree for a prefix, so this just filters it.

use nonos_app_skeleton::clients::vfs;

use super::glob::{basename, matches};
use super::{abspath, pid};
use crate::command::flags::{parse, Spec};
use crate::command::output::Output;
use crate::term::state::State;

pub fn find(state: &mut State, argv: &[&[u8]]) {
    let spec = Spec::new(b"find", b"").words(&[b"name", b"type"]);
    let parsed = match parse(&spec, &argv[1..]) {
        Ok(p) => p,
        Err(e) => return Output::new(&mut state.scrollback).writeln(&e),
    };
    let want_dir = match parsed.word(b"type") {
        None => None,
        Some(t) if t == b"d".as_slice() => Some(true),
        Some(t) if t == b"f".as_slice() => Some(false),
        Some(_) => {
            Output::new(&mut state.scrollback).writeln(b"find: -type takes f or d");
            return;
        }
    };
    let name = parsed.word(b"name");
    let arg = parsed.operands.first().copied().unwrap_or(b".");
    let mut dir = abspath(state, arg);
    if dir.last() != Some(&b'/') {
        dir.push(b'/');
    }
    let owner = pid(state);
    match vfs::list_paths(owner, &dir) {
        Ok(paths) => {
            let mut out = Output::new(&mut state.scrollback);
            for p in &paths {
                let path = p.as_bytes();
                if name.map(|pat| !matches(pat, basename(path))).unwrap_or(false) {
                    continue;
                }
                if let Some(want) = want_dir {
                    if !matches!(vfs::stat(owner, path), Ok((_, is_dir)) if is_dir == want) {
                        continue;
                    }
                }
                out.writeln(path);
            }
        }
        Err(e) => Output::new(&mut state.scrollback).writeln(e.as_bytes()),
    }
}
