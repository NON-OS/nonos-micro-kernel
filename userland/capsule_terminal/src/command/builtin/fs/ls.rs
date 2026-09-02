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

//! List a directory. Shows the immediate children of each target (the working
//! directory by default), directories suffixed with a slash, one per line.
//! Flags take the usual short-option shape: `-l -a -h -1 -R -t -S`, clustered
//! or separate, with `--` ending the flag run.

use alloc::vec::Vec;

use super::ls_emit::emit;
use super::ls_flags::{parse, LsFlags};
use super::ls_list::{children, visible};
use super::abspath;
use crate::command::output::Output;
use crate::term::state::State;

pub fn ls(state: &mut State, argv: &[&[u8]]) {
    let (flags, operands) = match parse(argv) {
        Ok(parsed) => parsed,
        Err(c) => {
            let mut msg = Vec::from(&b"ls: unrecognized option -- "[..]);
            msg.push(c);
            Output::new(&mut state.scrollback).writeln(&msg);
            return;
        }
    };
    let mut queue: Vec<Vec<u8>> = Vec::new();
    if operands.is_empty() {
        queue.push(dirpath(state, b"."));
    } else {
        for operand in &operands {
            queue.push(dirpath(state, operand));
        }
    }
    let header = flags.recurse || queue.len() > 1;
    walk(state, queue, &flags, header);
}

fn dirpath(state: &State, arg: &[u8]) -> Vec<u8> {
    let mut dir = abspath(state, arg);
    if dir.last() != Some(&b'/') {
        dir.push(b'/');
    }
    dir
}

fn walk(state: &mut State, mut queue: Vec<Vec<u8>>, flags: &LsFlags, header: bool) {
    let mut i = 0;
    while i < queue.len() {
        let dir = queue[i].clone();
        i += 1;
        if header && i > 1 {
            Output::new(&mut state.scrollback).writeln(b"");
        }
        let Some(names) = children(state, &dir).map(|n| visible(n, flags.all)) else { continue };
        let subs = emit(state, &dir, names, flags, header);
        if flags.recurse {
            queue.extend(subs);
        }
    }
}
