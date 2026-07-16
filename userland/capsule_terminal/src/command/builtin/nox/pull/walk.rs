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

use alloc::vec::Vec;

use super::args::PullArgs;
use super::run::one_file;
use super::{fetch, recurse, store};
use crate::term::state::State;

const MAX_DEPTH: u32 = 8;
const MAX_FILES: u32 = 512;

pub(super) fn walk(
    state: &mut State,
    pid: u32,
    ip: [u8; 4],
    a: &PullArgs,
    path: &[u8],
    dest: &[u8],
    depth: u32,
    count: &mut u32,
) {
    if depth >= MAX_DEPTH {
        return;
    }
    let index = match fetch::get(ip, a.target.port, &a.target.host, path) {
        Ok(b) => b,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return;
        }
    };
    for entry in recurse::parse_autoindex(&index) {
        if *count >= MAX_FILES {
            state.scrollback.push_error(b"pull: file limit reached");
            return;
        }
        let child_url = join(path, &entry.name);
        let child_dest = join(dest, &entry.name);
        if entry.is_dir {
            store::mkdir(pid, &child_dest);
            walk(state, pid, ip, a, &child_url, &child_dest, depth + 1, count);
        } else {
            *count += 1;
            one_file(state, pid, ip, a, &child_url, &child_dest);
        }
    }
}

fn join(base: &[u8], name: &[u8]) -> Vec<u8> {
    let mut p = base.to_vec();
    if p.last() != Some(&b'/') {
        p.push(b'/');
    }
    p.extend_from_slice(name);
    p
}
