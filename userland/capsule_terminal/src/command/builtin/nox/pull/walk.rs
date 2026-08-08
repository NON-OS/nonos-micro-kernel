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

use super::ctx::Ctx;
use super::run::one_file;
use super::{fetch, progress, recurse, store};
use crate::term::state::State;

const MAX_DEPTH: u32 = 8;
const MAX_FILES: u32 = 512;

pub(super) fn walk(
    state: &mut State,
    ctx: &Ctx<'_>,
    path: &[u8],
    dest: &[u8],
    depth: u32,
    count: &mut u32,
    tally: &mut progress::Tally,
) {
    let (pid, ip, a) = (ctx.pid, ctx.ip, ctx.args);
    if depth >= MAX_DEPTH {
        return;
    }
    let mut conn = None;
    let extra = super::auth::extra_headers(&a.auth, &a.headers);
    let index = match fetch::get_reuse(&mut conn, ip, a.target.port, &a.target.host, path, &extra) {
        Ok(b) => b,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return;
        }
    };
    for entry in recurse::parse_autoindex(&index) {
        if *count >= MAX_FILES {
            state.scrollback.push_error(b"pull: file limit reached");
            break;
        }
        let child_url = join(path, &entry.name);
        let child_dest = join(dest, &entry.name);
        if entry.is_dir {
            store::mkdir(pid, &child_dest);
            walk(state, ctx, &child_url, &child_dest, depth + 1, count, tally);
        } else {
            *count += 1;
            one_file(state, ctx, &mut conn, &child_url, &child_dest, tally);
        }
    }
    if let Some(c) = conn {
        c.close();
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
