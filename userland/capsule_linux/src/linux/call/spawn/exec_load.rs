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

//! Putting the replacing image into a guest that has just been emptied.

use alloc::vec::Vec;

use nonos_libc::mk_foreign_exec;

use super::exec_resolve::Program;
use crate::linux::guest::{Guest, STACK_SIZE, STACK_TOP};
use crate::linux::image;

/// The stack top a guest wakes on. The same one a fresh guest gets,
/// because after exec it is a fresh guest in every way but its pid.

pub fn load_over(guest: &mut Guest, pid: u32, program: &Program, envp: &[Vec<u8>]) -> Option<()> {
    let (loaded, entry, interp_base) = image::program(guest, &program.bytes).ok()?;
    guest.map(STACK_TOP - STACK_SIZE, STACK_SIZE, true, false);
    /*
     * A program invoked with no argv still gets one entry: argv[0] is what a C
     * runtime prints in its own error messages, and a shell that execs without
     * it reports failures against an empty name.
     */
    let args = match program.argv.is_empty() {
        true => alloc::vec![program.path.clone()],
        false => program.argv.clone(),
    };
    let rsp = image::build(guest, STACK_TOP, &loaded, interp_base, &args, envp)?;
    /*
     * The descriptor table survives, which is what makes a shell's redirection
     * work: it wires the pipe, then executes.
     */
    super::exec_clear::shed(guest);
    reset(guest);
    /*
     * Not a start: the caller is parked inside the `execve` it made and is
     * already runnable.
     */
    match mk_foreign_exec(pid, entry, rsp) {
        n if n < 0 => None,
        _ => Some(()),
    }
}

/// State that belongs to the program rather than the process.
fn reset(guest: &mut Guest) {
    guest.brk = crate::linux::guest::BRK_BASE;
    guest.mmap_next = crate::linux::guest::MMAP_BASE;
    guest.fs_base = 0;
}
