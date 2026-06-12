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

use alloc::vec::Vec;

use crate::term::util::format_u64;

// grep [-i] [-v] <pattern>: keep matching lines; -i ignores case, -v inverts.
pub(super) fn grep(args: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let (mut ci, mut inv) = (false, false);
    let mut pat: &[u8] = b"";
    for a in args {
        match *a {
            b"-i" => ci = true,
            b"-v" => inv = true,
            p => pat = p,
        }
    }
    input.into_iter().filter(|l| contains(l, pat, ci) != inv).collect()
}

pub(super) fn sort(mut input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    // Unstable sort: in place, no auxiliary allocation, and equal lines are
    // byte-identical so a stable order would not be observable anyway.
    input.sort_unstable();
    input
}

pub(super) fn uniq(input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = Vec::new();
    for line in input {
        if out.last().map(|prev| prev != &line).unwrap_or(true) {
            out.push(line);
        }
    }
    out
}

pub(super) fn nl(input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut out = Vec::with_capacity(input.len());
    for (i, line) in input.iter().enumerate() {
        let mut num = [0u8; 24];
        let k = format_u64(i as u64 + 1, &mut num);
        let mut row = Vec::with_capacity(k + 2 + line.len());
        row.extend_from_slice(&num[..k]);
        row.extend_from_slice(b"  ");
        row.extend_from_slice(line);
        out.push(row);
    }
    out
}

fn contains(hay: &[u8], needle: &[u8], ci: bool) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > hay.len() {
        return false;
    }
    (0..=hay.len() - needle.len()).any(|i| {
        needle.iter().enumerate().all(|(j, &nb)| {
            if ci {
                hay[i + j].eq_ignore_ascii_case(&nb)
            } else {
                hay[i + j] == nb
            }
        })
    })
}
