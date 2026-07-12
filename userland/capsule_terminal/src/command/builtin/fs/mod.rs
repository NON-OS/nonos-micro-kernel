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

//! The filesystem command set: the shell verbs a user expects (pwd, cd, ls,
//! cat, mkdir, touch, rm, mv, cp, stat). Each speaks to the vfs capsule over
//! IPC with the terminal's own pid, so the server's anti-impersonation check
//! passes, and resolves paths against the current working directory.

mod cat;
mod cd;
mod cp;
mod find;
mod grep;
mod head;
mod ls;
mod mkdir;
mod mv;
mod pwd;
mod read_file;
mod rm;
mod rmdir;
mod stat;
mod tail;
mod touch;
mod wc;

pub use cat::cat;
pub use cd::cd;
pub use cp::cp;
pub use find::find;
pub use grep::grep;
pub use head::head;
pub use ls::ls;
pub use mkdir::mkdir;
pub use mv::mv;
pub use pwd::pwd;
pub use rm::rm;
pub use rmdir::rmdir;
pub use stat::stat;
pub use tail::tail;
pub use touch::touch;
pub use wc::wc;

// Parse a `-n N` count with a fallback, returning the count and the file arg.
pub(super) fn head_tail_args<'a>(
    argv: &'a [&'a [u8]],
    default: usize,
) -> (usize, Option<&'a [u8]>) {
    let mut n = default;
    let mut file = None;
    let mut i = 1;
    while i < argv.len() {
        if argv[i] == b"-n" {
            if let Some(v) = argv.get(i + 1) {
                n = parse_usize(v).unwrap_or(default);
                i += 2;
                continue;
            }
        }
        if argv[i].first() != Some(&b'-') {
            file = Some(argv[i]);
        }
        i += 1;
    }
    (n, file)
}

fn parse_usize(b: &[u8]) -> Option<usize> {
    if b.is_empty() {
        return None;
    }
    let mut v: usize = 0;
    for &c in b {
        if !c.is_ascii_digit() {
            return None;
        }
        v = v.checked_mul(10)?.checked_add((c - b'0') as usize)?;
    }
    Some(v)
}

use crate::term::state::State;

// The terminal's real pid, resolved once. The vfs server rejects a claimed
// owner pid that differs from the sender, so this must be our own.
pub(super) fn pid(state: &mut State) -> u32 {
    if state.owner_pid == 0 {
        state.owner_pid = nonos_libc::mk_getpid();
    }
    state.owner_pid
}

// Absolute path for an argument, resolved against the working directory.
pub(super) fn abspath(state: &State, arg: &[u8]) -> alloc::vec::Vec<u8> {
    crate::term::cwd::resolve(state.cwd.as_bytes(), arg)
}
