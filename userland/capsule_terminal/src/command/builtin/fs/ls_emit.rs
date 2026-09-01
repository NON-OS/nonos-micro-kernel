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

//! Filter, sort and print one directory's entries, returning the subdirectory
//! paths so `-R` can queue them.

use alloc::vec::Vec;

use super::ls_flags::LsFlags;
use super::ls_long::long_row;
use super::ls_meta::{meta, subdirs};
use crate::command::output::Output;
use crate::term::state::State;

pub fn emit(
    state: &mut State,
    base: &[u8],
    names: Vec<Vec<u8>>,
    flags: &LsFlags,
    header: bool,
) -> Vec<Vec<u8>> {
    let kept: Vec<Vec<u8>> =
        names.into_iter().filter(|n| flags.all || n.first() != Some(&b'.')).collect();
    let rows = if flags.needs_meta() { meta(state, base, &kept, flags) } else { Vec::new() };
    let ordered: Vec<Vec<u8>> =
        if flags.needs_meta() { rows.iter().map(|r| r.name.clone()).collect() } else { kept };
    let mut out = Output::new(&mut state.scrollback);
    if header {
        let mut line = base.to_vec();
        line.push(b':');
        out.writeln(&line);
    }
    if flags.long {
        for row in &rows {
            out.writeln(&long_row(row, flags.human));
        }
    } else {
        for name in &ordered {
            out.writeln(name);
        }
    }
    subdirs(base, &ordered)
}
