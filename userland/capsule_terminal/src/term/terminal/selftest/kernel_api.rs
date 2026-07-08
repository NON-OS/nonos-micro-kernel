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

use nonos_libc::{mk_kill, mk_stdin_read, mk_wait};

use super::mark;

const ERRNO_PERM: i64 = -1;
const ERRNO_CHILD: i64 = -10;
const SIGTERM: u64 = 15;

pub fn run() {
    mark(b"wait-echild", mk_wait(1, 10) == ERRNO_CHILD);
    mark(b"kill-eperm", mk_kill(1, SIGTERM) == ERRNO_PERM);

    let mut buf = [0u8; 16];
    mark(b"stdin-empty", mk_stdin_read(buf.as_mut_ptr(), buf.len() as u64) == 0);
}
