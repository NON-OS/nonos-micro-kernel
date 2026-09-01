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

//! Print the lines of a file that contain a substring.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;

use super::grep_scan::{scan, Opts};
use super::{abspath, pid};
use crate::command::flags::{parse, Spec};
use crate::command::output::Output;
use crate::term::state::State;

pub fn grep(state: &mut State, argv: &[&[u8]]) {
    let parsed = match parse(&Spec::new(b"grep", b"cinrv"), &argv[1..]) {
        Ok(p) => p,
        Err(e) => return Output::new(&mut state.scrollback).writeln(&e),
    };
    if parsed.operands.len() < 2 {
        Output::new(&mut state.scrollback).writeln(b"usage: grep <pattern> <file>");
        return;
    }
    let pat = parsed.operands[0];
    let recurse = parsed.has(b'r');
    let opts = Opts {
        number: parsed.has(b'n'),
        count: parsed.has(b'c'),
        fold: parsed.has(b'i'),
        invert: parsed.has(b'v'),
        label: recurse,
    };
    let mut targets: Vec<Vec<u8>> = Vec::new();
    for arg in &parsed.operands[1..] {
        if recurse {
            expand(state, arg, &mut targets);
        } else {
            targets.push(arg.to_vec());
        }
    }
    for target in targets {
        scan(state, &target, pat, &opts);
    }
}

fn expand(state: &mut State, arg: &[u8], out: &mut Vec<Vec<u8>>) {
    let mut dir = abspath(state, arg);
    if dir.last() != Some(&b'/') {
        dir.push(b'/');
    }
    let owner = pid(state);
    match vfs::list_paths(owner, &dir) {
        Ok(paths) => {
            for p in paths {
                let bytes = p.into_bytes();
                if bytes.last() != Some(&b'/') {
                    out.push(bytes);
                }
            }
        }
        Err(_) => out.push(arg.to_vec()),
    }
}
