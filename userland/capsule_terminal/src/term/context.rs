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

use crate::term::cwd::strip_home;
use crate::term::util::copy_into;

/// The line that opens a command block: `user@host:path`.
///
/// It is pushed through the scrollback grid rather than drawn, so it is bytes
/// and the block's own start line, which is where the run's time and outcome
/// are right-aligned.
pub fn context_line(user: &[u8], host: &[u8], cwd: &[u8], home: &[u8], out: &mut [u8]) -> usize {
    let mut n = 0;
    n += copy_into(&mut out[n..], user);
    n += copy_into(&mut out[n..], b"@");
    n += copy_into(&mut out[n..], host);
    n += copy_into(&mut out[n..], b":");
    match strip_home(cwd, home) {
        Some(tail) => {
            n += copy_into(&mut out[n..], b"~");
            n += copy_into(&mut out[n..], tail);
        }
        None => n += copy_into(&mut out[n..], cwd),
    }
    n
}
