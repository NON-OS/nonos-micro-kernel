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

use nonos_libc::mk_debug;

/// Say which step of opening a tunnel refused.
///
/// A client is told only that the connect was rejected, which is all it can
/// act on, but the reason matters here: no session, no exit, and a send that
/// failed are three unrelated faults that look identical from the browser.
pub fn open_failed(step: &[u8]) {
    let mut line = [0u8; 64];
    let mut n = 0;
    for &b in b"[SOCKS5] open refused: " {
        line[n] = b;
        n += 1;
    }
    for &b in step {
        if n < line.len() - 2 {
            line[n] = b;
            n += 1;
        }
    }
    line[n] = b'\n';
    mk_debug(line.as_ptr(), n + 1);
}
