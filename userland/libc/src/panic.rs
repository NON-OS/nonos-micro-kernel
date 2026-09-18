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

//! A capsule panic ends the process with 134, the SIGABRT convention used
//! elsewhere in NONOS for unrecoverable user-side faults. Before it does,
//! the location and message go to the kernel log in one line, formatted
//! into a stack buffer so a panic inside the allocator still reports. The
//! kernel drops the line for a capsule without the Debug right; the exit
//! code is then the only trace, and the kernel logs that itself.

use core::fmt::Write;

struct Line {
    buf: [u8; 240],
    len: usize,
}

impl Write for Line {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let room = self.buf.len() - self.len;
        let n = s.len().min(room);
        self.buf[self.len..self.len + n].copy_from_slice(&s.as_bytes()[..n]);
        self.len += n;
        Ok(())
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut line = Line { buf: [0; 240], len: 0 };
    let _ = write!(line, "[PANIC] {info}\n");
    let _ = crate::debug::mk_debug(line.buf.as_ptr(), line.len);
    crate::unistd::mk_exit(134)
}
