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

use nonos_libc::{heap_init, mk_debug, mk_exit, mk_foreign_spawn, mk_foreign_start};

use super::guest::Guest;
use super::image;
use super::serve::serve;
use super::source::source;

/// The stack top a guest wakes on, below the personality's own mappings
/// and above everything it maps for itself.
const STACK_TOP: u64 = 0x0000_7FFF_F000;

/// The stack a guest gets, which is also the largest span one peer call
/// will map. Sixty-four kilobytes served a static binary and would have
/// overflowed under an interpreter recursing through a dependency graph,
/// as a fault inside the linker with nothing to read.
const STACK_SIZE: u64 = 1 << 20;

pub fn run() -> ! {
    let _ = heap_init();
    say(b"[LINUX] personality up\n");
    let (path, bytes) = source();
    let pid = mk_foreign_spawn(b"linux");
    if pid < 0 {
        say(b"[LINUX] no guest: refused\n");
        mk_exit(1)
    }
    let mut guest = Guest::new(pid as u32);
    let code = match start(&mut guest, &path, &bytes) {
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

fn start(guest: &mut Guest, path: &[u8], bytes: &[u8]) -> Result<(), &'static [u8]> {
    let (image, entry, interp_base) =
        image::program(guest, bytes).map_err(|_| &b"[LINUX] image refused\n"[..])?;
    guest.map(STACK_TOP - STACK_SIZE, STACK_SIZE, true, false);
    let rsp = image::build(guest, STACK_TOP, &image, interp_base, path)
        .ok_or(&b"[LINUX] stack refused\n"[..])?;
    match mk_foreign_start(guest.pid, entry, rsp) {
        n if n < 0 => Err(&b"[LINUX] start refused\n"[..]),
        _ => Ok(()),
    }
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
