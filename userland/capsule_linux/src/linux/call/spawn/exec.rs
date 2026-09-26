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

//! `execve`: the same process, a different program.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

use super::exec_args::vector;
use super::exec_clear::clear;
use super::exec_load::load_over;
use super::exec_resolve::resolve;

pub fn execve(guest: &mut Guest, pid: u32, path: u64, argv: u64, envp: u64) -> Answer {
    let Some(name) = crate::linux::file::read_path(guest, path) else {
        return Answer::value(errno::fail(errno::EFAULT));
    };
    /*
     * argv and envp live in the memory that is about to be unmapped, so they
     * are copied out here and not one step later.
     */
    let (Some(args), Some(env)) = (vector(guest, argv), vector(guest, envp)) else {
        return Answer::value(errno::fail(errno::EFAULT));
    };
    /*
     * Found, followed through any `#!` line, and proved at every step, all
     * while the caller still has an address space to be told no in.
     */
    let program = match resolve(&guest.cwd, &name, &args) {
        Ok(p) => p,
        Err(e) => return Answer::value(e),
    };
    super::exec_threads::reap(guest, pid);
    clear(guest);
    match load_over(guest, pid, &program, &env) {
        Some(()) => Answer::Park,
        None => Answer::value(errno::fail(errno::ENOEXEC)),
    }
}
