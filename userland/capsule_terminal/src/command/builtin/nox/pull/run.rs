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
use super::walk::walk;
use super::{args, fetch, store};
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
    if a.target.is_dir {
        store::mkdir(pid, &dest);
        let mut count = 0u32;
        walk(state, pid, ip, &a, &a.target.path, &dest, 0, &mut count);
    } else {
        one_file(state, pid, ip, &a, &a.target.path, &dest);
    }
    true
}

pub(super) fn one_file(
    state: &mut State,
    pid: u32,
    ip: [u8; 4],
    a: &args::PullArgs,
    path: &[u8],
    dest: &[u8],
) -> bool {
    match fetch::get(ip, a.target.port, &a.target.host, path) {
        Ok(body) => match store::write(pid, dest, &body) {
            Ok(()) => {
                state.scrollback.push_line(dest);
                true
            }
            Err(e) => {
                state.scrollback.push_error(e.as_bytes());
                false
            }
        },
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            false
        }
    }
}
