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

//! Print the contents of one or more files; -n numbers the lines.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;

use super::{abspath, pid};
use crate::command::flags::{parse, Spec};
use crate::command::output::Output;
use crate::term::state::State;
use crate::term::util::format_u64;

const MAX_READ: u32 = 256 * 1024;

pub fn cat(state: &mut State, argv: &[&[u8]]) {
    let parsed = match parse(&Spec::new(b"cat", b"n"), &argv[1..]) {
        Ok(p) => p,
        Err(e) => return Output::new(&mut state.scrollback).writeln(&e),
    };
    if parsed.operands.is_empty() {
        Output::new(&mut state.scrollback).writeln(b"cat: missing file");
        return;
    }
    let number = parsed.has(b'n');
    let mut seq = 0u64;
    for arg in parsed.operands {
        let path = abspath(state, arg);
        let owner = pid(state);
        match vfs::read_file(owner, &path, MAX_READ) {
            Ok(bytes) => {
                let mut out = Output::new(&mut state.scrollback);
                for line in bytes.split(|&b| b == b'\n') {
                    if !number {
                        out.writeln(line);
                        continue;
                    }
                    seq += 1;
                    out.writeln(&numbered(seq, line));
                }
            }
            Err(e) => Output::new(&mut state.scrollback).writeln(e.as_bytes()),
        }
    }
}

fn numbered(seq: u64, line: &[u8]) -> Vec<u8> {
    let mut num = [0u8; 20];
    let k = format_u64(seq, &mut num);
    let mut row = Vec::with_capacity(k + 2 + line.len());
    row.extend_from_slice(&num[..k]);
    row.extend_from_slice(b"  ");
    row.extend_from_slice(line);
    row
}
