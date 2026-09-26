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

//! The line a slow handler leaves on the console.

use nonos_libc::mk_debug;

/// A handler that outlives its caller's timeout turns every reply into a drop
/// and reads as a dead service. Name the op and the cost.
pub(super) fn report(op: u16, spent: i64) {
    if spent <= 1000 {
        return;
    }
    let mut line = *b"[VFS] slow op 0000 ms 000000";
    for (i, shift) in [(14usize, 12u32), (15, 8), (16, 4), (17, 0)] {
        line[i] = b"0123456789abcdef"[((op as usize) >> shift) & 0xF];
    }
    let ms = spent.min(999_999) as u32;
    let mut v = ms;
    for i in (22..28).rev() {
        line[i] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    let _ = mk_debug(line.as_ptr(), line.len());
}
