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

//! Bring one Linux program up and stay with it until it ends.

use nonos_libc::{heap_init, mk_debug, mk_exit, mk_foreign_spawn};

use super::guest::Guest;
use super::serve::serve;
use super::source::source;
use super::start_guest::start;

pub fn run() -> ! {
    let _ = heap_init();
    say(b"[LINUX] personality up\n");
    if let Some(name) = super::request::install_request() {
        say(b"[LINUX] installing\n");
        let ok = super::install::install(&name);
        say(if ok { b"[LINUX] installed\n" } else { b"[LINUX] install failed\n" });
        mk_exit(if ok { 0 } else { 1 })
    }
    let (path, bytes, origin) = source();
    let pid = mk_foreign_spawn(b"linux");
    if pid < 0 {
        say(b"[LINUX] no guest, errno ");
        let e = (-pid) as u32;
        let digits = [b'0' + (e / 10 % 10) as u8, b'0' + (e % 10) as u8, b'\n'];
        say(&digits);
        mk_exit(1)
    }
    let mut guest = Guest::new(pid as u32);
    let code = match start(&mut guest, &path, &bytes, origin) {
        Ok(()) => {
            say(b"[LINUX] guest running\n");
            serve(&mut guest)
        }
        Err(step) => {
            say(step);
            mk_exit(2)
        }
    };
    say(b"[LINUX] guest exited\n");
    mk_exit(code)
}

pub(super) fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
