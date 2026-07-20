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

use nonos_app_skeleton::clients::vfs::read_file;

use super::super::ensure_pid::ensure_pid;
use super::super::pull::resolve::resolve_host;
use super::{args, request, walk};
use crate::term::cwd::resolve;
use crate::term::state::State;

const MAX: u32 = 64 * 1024 * 1024;

pub fn run(state: &mut State, argv: &[&[u8]]) -> bool {
    let a = match args::parse(argv) {
        Ok(a) => a,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return false;
        }
    };
    let ip = match resolve_host(&a.target.hostname) {
        Ok(ip) => ip,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return false;
        }
    };
    let pid = ensure_pid(state);
    let path = resolve(state.cwd.as_bytes(), &a.src);
    if a.target.is_dir {
        return walk::walk(state, pid, ip, &path, &a.target);
    }
    let body = match read_file(pid, &path, MAX) {
        Ok(b) => b,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return false;
        }
    };
    match request::put(ip, a.target.port, &a.target.host, &a.target.path, &body) {
        Ok(()) => {
            let mut msg = Vec::from(&b"pushed "[..]);
            msg.extend_from_slice(&a.target.path);
            state.scrollback.push_line(&msg);
            true
        }
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            false
        }
    }
}
