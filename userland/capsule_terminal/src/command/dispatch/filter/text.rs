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

use alloc::vec;
use alloc::vec::Vec;

use crate::command::flags::{parse, Spec};
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

// sort [-n] [-r] [-u]: -n orders by leading integer, -r reverses, -u drops
// adjacent duplicates once the order is settled.
pub(super) fn sort(args: &[&[u8]], mut input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let parsed = match parse(&Spec::new(b"sort", b"nru"), args) {
        Ok(p) => p,
        Err(e) => return vec![e],
    };
    if parsed.has(b'n') {
        input.sort_unstable_by(|a, b| numeric_key(a).cmp(&numeric_key(b)).then_with(|| a.cmp(b)));
    } else {
        // Unstable sort: in place, no auxiliary allocation, and equal lines are
        // byte-identical so a stable order would not be observable anyway.
        input.sort_unstable();
    }
    if parsed.has(b'r') {
        input.reverse();
    }
    if parsed.has(b'u') {
        input = uniq(input);
    }
    input
}

fn numeric_key(line: &[u8]) -> i64 {
    let body = line.trim_ascii_start();
    let (neg, digits) = match body.first() {
        Some(b'-') => (true, &body[1..]),
        _ => (false, body),
    };
    let mut v: i64 = 0;
    for &c in digits {
        if !c.is_ascii_digit() {
            break;
        }
        v = v.saturating_mul(10).saturating_add((c - b'0') as i64);
    }
    if neg {
        -v
    } else {
        v
    }
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

// cut -d<delim> -f<n>: emit the n-th delim-separated field of each line
// (1-based; default delimiter space, default field 1). Missing field -> "".
pub(super) fn cut(args: &[&[u8]], input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut delim = b' ';
    let mut field = 1usize;
    for a in args {
        if let Some(d) = a.strip_prefix(b"-d") {
            if let Some(&c) = d.first() {
                delim = c;
            }
        } else if let Some(f) = a.strip_prefix(b"-f") {
            field = field_index(f);
        }
    }
    input
        .into_iter()
        .map(|l| l.split(|&b| b == delim).nth(field - 1).map(<[u8]>::to_vec).unwrap_or_default())
        .collect()
}

fn field_index(digits: &[u8]) -> usize {
    let mut n = 0usize;
    for &c in digits {
        if c.is_ascii_digit() {
            n = n.saturating_mul(10).saturating_add((c - b'0') as usize);
        }
    }
    n.max(1)
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
