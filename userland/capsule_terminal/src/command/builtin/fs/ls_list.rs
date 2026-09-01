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

//! The immediate children of a directory, in the order the vfs reports them:
//! the first path segment under the prefix, deduplicated, directories suffixed
//! with a slash.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;

use super::pid;
use crate::command::output::Output;
use crate::term::state::State;

pub fn children(state: &mut State, base: &[u8]) -> Option<Vec<Vec<u8>>> {
    let owner = pid(state);
    let paths = match vfs::list_paths(owner, base) {
        Ok(p) => p,
        Err(e) => {
            Output::new(&mut state.scrollback).writeln(e.as_bytes());
            return None;
        }
    };
    let mut seen: Vec<Vec<u8>> = Vec::new();
    for p in &paths {
        let bytes = p.as_bytes();
        let Some(rest) = bytes.strip_prefix(base) else { continue };
        if rest.is_empty() {
            continue;
        }
        let cut = rest.iter().position(|&c| c == b'/').unwrap_or(rest.len());
        let mut name = rest[..cut].to_vec();
        if cut < rest.len() || bytes.last() == Some(&b'/') {
            name.push(b'/');
        }
        if !name.is_empty() && !seen.contains(&name) {
            seen.push(name);
        }
    }
    Some(seen)
}

// Drop dot-prefixed names unless `-a` asked for them, the conventional `ls`
// rule. The names arrive bare, without the directory prefix, so the leading
// byte is the entry's own first character.
pub(super) fn visible(names: Vec<Vec<u8>>, all: bool) -> Vec<Vec<u8>> {
    if all {
        return names;
    }
    names.into_iter().filter(|n| n.first() != Some(&b'.')).collect()
}
