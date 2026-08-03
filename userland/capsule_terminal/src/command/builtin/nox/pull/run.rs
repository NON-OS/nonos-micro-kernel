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

use super::super::ensure_pid::ensure_pid;
use super::conn::Conn;
use super::ctx::Ctx;
use super::walk::walk;
use super::{args, fetch, progress, store};
use crate::term::cwd::resolve;
use crate::term::state::State;

pub fn run(state: &mut State, argv: &[&[u8]]) -> bool {
    let a = match args::parse(argv) {
        Ok(a) => a,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return false;
        }
    };
    let ip = match super::resolve::resolve_host(&a.target.hostname) {
        Ok(ip) => ip,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return false;
        }
    };
    let pid = ensure_pid(state);
    let dest = resolve(state.cwd.as_bytes(), &a.dest);
    let mut tally = progress::Tally { ok: 0, skipped: 0, failed: 0 };
    if a.target.is_dir {
        store::mkdir(pid, &dest);
        let mut count = 0u32;
        walk(state, &Ctx { pid, ip, args: &a }, &a.target.path, &dest, 0, &mut count, &mut tally);
    } else {
        let mut conn = None;
        one_file(state, &Ctx { pid, ip, args: &a }, &mut conn, &a.target.path, &dest, &mut tally);
        if let Some(c) = conn {
            c.close();
        }
    }
    state.scrollback.push_line(&progress::summary(&tally));
    true
}

pub(super) fn one_file(
    state: &mut State,
    ctx: &Ctx<'_>,
    conn: &mut Option<Conn>,
    path: &[u8],
    dest: &[u8],
    tally: &mut progress::Tally,
) -> bool {
    let (pid, ip, a) = (ctx.pid, ctx.ip, ctx.args);
    if a.no_clobber && store::exists(pid, dest) {
        tally.skipped += 1;
        return true;
    }
    let extra = super::auth::extra_headers(&a.auth, &a.headers);
    if a.skip_unchanged {
        if let Some(local) = store::size(pid, dest) {
            if fetch::head_reuse(conn, ip, a.target.port, &a.target.host, path, &extra)
                == Some(local as usize)
            {
                tally.skipped += 1;
                return true;
            }
        }
    }
    if !store::has_free_slot(pid) {
        state.scrollback.push_error(b"pull: vfs store full");
        tally.failed += 1;
        return false;
    }
    match fetch::get_reuse(conn, ip, a.target.port, &a.target.host, path, &extra) {
        Ok(body) => {
            if a.verify {
                if let Err(e) = super::verify::check(
                    conn,
                    ip,
                    a.target.port,
                    &a.target.host,
                    path,
                    &extra,
                    &body,
                ) {
                    state.scrollback.push_error(e.as_bytes());
                    tally.failed += 1;
                    return false;
                }
            }
            match store::write(pid, dest, &body) {
                Ok(()) => {
                    state.scrollback.push_line(&progress::file_line(dest, body.len()));
                    tally.ok += 1;
                    true
                }
                Err(e) => {
                    state.scrollback.push_error(e.as_bytes());
                    tally.failed += 1;
                    false
                }
            }
        }
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            tally.failed += 1;
            false
        }
    }
}
