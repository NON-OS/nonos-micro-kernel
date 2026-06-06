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

pub(super) fn emit_call_failed(out: &mut Output<'_>, rc: i64) {
    let mut line = [0u8; 64];
    let mut k = 0;
    k += copy_into(&mut line[k..], b"  market call failed errno=");
    k += format_u64((-rc) as u64, &mut line[k..]);
    out.writeln(&line[..k]);
}
