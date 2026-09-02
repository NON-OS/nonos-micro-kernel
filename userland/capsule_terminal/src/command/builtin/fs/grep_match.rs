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

//! grep's leaf helpers: the row prefix, the counter, and the matcher.

use alloc::vec::Vec;

use super::grep_scan::Opts;
use crate::term::util::format_u64;

pub(super) fn prefix(path: &[u8], opts: &Opts) -> Vec<u8> {
    let mut row = Vec::new();
    if opts.label {
        row.extend_from_slice(path);
        row.push(b':');
    }
    row
}

pub(super) fn push_num(out: &mut Vec<u8>, v: u64) {
    let mut num = [0u8; 20];
    let n = format_u64(v, &mut num);
    out.extend_from_slice(&num[..n]);
}

pub(super) fn contains(hay: &[u8], needle: &[u8], fold: bool) -> bool {
    if needle.is_empty() || needle.len() > hay.len() {
        return needle.is_empty();
    }
    (0..=hay.len() - needle.len()).any(|i| {
        needle.iter().enumerate().all(|(j, &nb)| {
            if fold {
                hay[i + j].eq_ignore_ascii_case(&nb)
            } else {
                hay[i + j] == nb
            }
        })
    })
}
