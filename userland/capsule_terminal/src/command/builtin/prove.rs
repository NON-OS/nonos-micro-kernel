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

//! `prove`: run a proeve program and prove its execution with the in-kernel
//! money-grade STARK, in process. With no argument it proves a built-in demo;
//! with a path it reads that file from the VFS and proves it. The program is
//! compiled, run on the VM, laid out for the step AIR, proven, and the proof
//! verified, all here in the capsule. A false claim in the program has no proof
//! and is reported as such rather than crashing.

use nonos_app_skeleton::clients::vfs;
use nonos_prove::{prove_source, RunError};

use super::fs::{abspath, pid};
use crate::command::output::Output;
use crate::term::dimensions::COLS;
use crate::term::state::State;
use crate::term::util::{copy_into, format_u64};

const MAX_READ: u32 = 64 * 1024;

// A small program that touches add, multiply, and an assertion:
//   a = 3; b = 5; s = a + b (= 8); p = s * s (= 64); assert p == 64.
const DEMO: &str = "let a = 3; let b = 5; let s = a + b; let p = s * s; assert p - 64;";

pub fn run(state: &mut State, argv: &[&[u8]]) {
    if argv.len() >= 2 {
        let path = abspath(state, argv[1]);
        let owner = pid(state);
        match vfs::read_file(owner, &path, MAX_READ) {
            Ok(bytes) => match core::str::from_utf8(&bytes) {
                Ok(src) => report(state, src),
                Err(_) => {
                    Output::new(&mut state.scrollback).writeln(b"prove: file is not valid UTF-8")
                }
            },
            Err(e) => Output::new(&mut state.scrollback).writeln(e.as_bytes()),
        }
    } else {
        Output::new(&mut state.scrollback)
            .writeln(b"prove: no file given, proving the demo program");
        report(state, DEMO);
    }
}

// Compile, prove, and verify `src`, printing the outcome and trace shape.
fn report(state: &mut State, src: &str) {
    let mut out = Output::new(&mut state.scrollback);
    out.writeln(b"prove: NONOS verifiable-compute language");
    match prove_source(src) {
        Ok(r) => {
            if r.verified {
                out.writeln(b"  PROVEN and VERIFIED  (money-grade STARK)");
            } else {
                out.writeln(b"  proof did NOT verify");
            }
            let mut line = [0u8; COLS];
            let mut n = 0;
            n += copy_into(&mut line[n..], b"  steps=");
            n += format_u64(r.steps as u64, &mut line[n..]);
            n += copy_into(&mut line[n..], b"  trace=2^");
            n += format_u64(r.log_trace_len as u64, &mut line[n..]);
            n += copy_into(&mut line[n..], b" x ");
            n += format_u64(r.trace_width as u64, &mut line[n..]);
            out.writeln(&line[..n]);
        }
        Err(e) => out.writeln(reason(&e)),
    }
}

fn reason(e: &RunError) -> &'static [u8] {
    match e {
        RunError::Compile(_) => b"  compile error: check the program's syntax and names",
        RunError::Execute(_) => b"  unprovable: a claim in the program is false",
        RunError::Layout(_) => b"  internal error: the trace could not be laid out",
        RunError::ProgramTooLong { .. } => b"  program too long to prove",
    }
}
