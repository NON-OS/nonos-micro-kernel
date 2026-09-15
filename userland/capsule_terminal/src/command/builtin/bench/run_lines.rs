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

//! The two lines around the table: what the counter cost, and what each row is.

use super::format::decimal;
use super::probe_list::Probe;
use crate::command::output::Output;

pub(super) fn blank_line(out: &mut Output<'_>, overhead: u64) {
    let mut line = [b' '; 48];
    let head = b"counter overhead: ";
    line[..head.len()].copy_from_slice(head);
    let mut b = [0u8; 20];
    let n = decimal(overhead, &mut b);
    line[head.len()..head.len() + n].copy_from_slice(&b[..n]);
    out.writeln(&line);
}

pub(super) fn note(out: &mut Output<'_>, p: &Probe) {
    let mut line = [b' '; 96];
    line[..p.name.len()].copy_from_slice(p.name);
    let at = 10;
    let end = (at + p.what.len()).min(line.len());
    line[at..end].copy_from_slice(&p.what[..end - at]);
    out.writeln(&line);
}
