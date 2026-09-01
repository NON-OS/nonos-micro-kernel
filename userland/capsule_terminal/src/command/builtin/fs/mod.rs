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
mod glob;
mod grep;
mod grep_match;
mod grep_scan;
mod head;
mod ls;
mod ls_emit;
mod ls_flags;
mod ls_list;
mod ls_long;
mod ls_meta;
mod ls_num;
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
