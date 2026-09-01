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

//! The per-file half of grep: expand a -r target, then emit or count matches.

use alloc::vec::Vec;

use super::read_file::slurp;
use crate::command::output::Output;
use super::grep_match::{contains, prefix, push_num};
use crate::term::state::State;

pub(super) struct Opts {
    pub number: bool,
    pub count: bool,
    pub fold: bool,
    pub invert: bool,
    pub label: bool,
}

pub(super) fn scan(state: &mut State, path: &[u8], pat: &[u8], opts: &Opts) {
    let Some(bytes) = slurp(state, path) else { return };
    let mut hits = 0u64;
    let mut rows: Vec<Vec<u8>> = Vec::new();
    for (i, line) in bytes.split(|&b| b == b'\n').enumerate() {
        if contains(line, pat, opts.fold) == opts.invert {
            continue;
        }
        hits += 1;
        if opts.count {
            continue;
        }
        let mut row = prefix(path, opts);
        if opts.number {
            push_num(&mut row, i as u64 + 1);
            row.push(b':');
        }
        row.extend_from_slice(line);
        rows.push(row);
    }
    let mut out = Output::new(&mut state.scrollback);
    if opts.count {
        let mut row = prefix(path, opts);
        push_num(&mut row, hits);
        out.writeln(&row);
        return;
    }
    for row in &rows {
        out.writeln(row);
    }
}
