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

use crate::command::output::Output;
use crate::term::dimensions::COLS;
use crate::term::util::copy_into;

pub(super) fn emit_line(out: &mut Output<'_>, listing: &[u8], name: &[u8], ready: bool) {
    let mut line = [0u8; COLS];
    let mut n = 0;
    n += copy_into(&mut line[n..], if ready { b"  [ready]   " } else { b"  [pending] " });
    n += copy_into(&mut line[n..], name);
    n += copy_into(&mut line[n..], b"  id=");
    n += copy_into(&mut line[n..], listing);
    out.writeln(&line[..n]);
}
