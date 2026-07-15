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

//! One-shot bind diagnostics on the boot console. Real laptops have no
//! serial port, so the framebuffer console is the only place the parsed
//! report layout and the first raw frames can be read back from a running
//! machine; one photograph of these lines pins the exact bit positions the
//! device uses, ending any disagreement between parser and hardware.

use alloc::format;
use alloc::string::String;

pub fn line(s: String) {
    let b = s.as_bytes();
    let len = b.len().min(256);
    let _ = nonos_libc::mk_debug(b.as_ptr(), len);
}

pub fn hex_line(tag: &str, bytes: &[u8]) {
    let mut s = String::from(tag);
    for b in bytes {
        s.push_str(&format!(" {:02x}", b));
    }
    s.push('\n');
    line(s);
}
