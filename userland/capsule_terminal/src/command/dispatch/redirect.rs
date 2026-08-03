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

// Result of scanning a command line for output redirection. `>` writes a
// command's output to a file, `>>` appends. The bytes before the operator
// are the command; the token after it is the destination path.
pub(super) enum Plan<'a> {
    Plain,
    Redirect { cmd_len: usize, append: bool, path: &'a [u8] },
    Error(&'static [u8]),
}

// Scan for an input redirect `< path`, returning the args with the
// `< path` pair removed plus the captured path (None if absent). Output
// redirects and pipes are scanned separately on the returned args.
type Split<'a> = Result<(Vec<&'a [u8]>, Option<&'a [u8]>), &'static [u8]>;

pub(super) fn split_input<'a>(args: &'a [&'a [u8]]) -> Split<'a> {
    let mut out: Vec<&[u8]> = Vec::new();
    let mut path: Option<&[u8]> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == b"<" {
            match args.get(i + 1) {
                Some(p) if !p.is_empty() => {
                    path = Some(p);
                    i += 2;
                    continue;
                }
                _ => return Err(b"redirect: expected a file path after <"),
            }
        }
        out.push(args[i]);
        i += 1;
    }
    Ok((out, path))
}

pub(super) fn split<'a>(args: &'a [&'a [u8]]) -> Plan<'a> {
    let mut i = 0;
    while i < args.len() {
        if args[i] == b">" || args[i] == b">>" {
            let append = args[i] == b">>";
            return match args.get(i + 1) {
                Some(p) if !p.is_empty() => Plan::Redirect { cmd_len: i, append, path: p },
                _ => Plan::Error(b"redirect: expected a file path after >"),
            };
        }
        i += 1;
    }
    Plan::Plain
}
