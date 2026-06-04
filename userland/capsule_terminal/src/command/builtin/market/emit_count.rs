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
use crate::term::util::{copy_into, format_u64};

pub(super) fn emit_count(out: &mut Output<'_>, count: u32) {
    let mut header = [0u8; 64];
    let mut hp = 0;
    hp += copy_into(&mut header[hp..], b"market: ");
    hp += format_u64(count as u64, &mut header[hp..]);
    hp += copy_into(&mut header[hp..], b" listing(s)");
    out.writeln(&header[..hp]);
}
