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
use crate::jobs::JobTable;
use crate::term::util::{copy_into, format_u64};

// Every job in this shell already runs cooperatively as soon as it's
// submitted (there is no Ctrl-Z stop/resume), so `bg` has nothing to do
// beyond confirming that fact for a valid job id.
pub fn run(out: &mut Output<'_>, jobs: &JobTable, argv: &[&[u8]]) {
    if argv.len() < 2 {
        out.writeln(b"usage: bg <id>");
        return;
    }
    let id = match core::str::from_utf8(argv[1]).ok().and_then(|s| s.parse::<u32>().ok()) {
        Some(id) => id,
        None => {
            out.writeln(b"bg: invalid job id");
            return;
        }
    };
    if jobs.get(id).is_none() {
        out.writeln(b"bg: no such job");
        return;
    }
    let mut line = [0u8; 40];
    let mut n = 0;
    n += copy_into(&mut line[n..], b"job ");
    let mut num = [0u8; 20];
    let nk = format_u64(id as u64, &mut num);
    n += copy_into(&mut line[n..], &num[..nk]);
    n += copy_into(&mut line[n..], b" already running");
    out.writeln(&line[..n]);
}
