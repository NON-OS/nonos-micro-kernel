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
//! Routing `git <subcommand>`.

use crate::command::output::Output;
use crate::term::state::State;

use super::{add, clone, commit, init, log, push, status};

pub fn run(state: &mut State, argv: &[&[u8]]) {
    let Some(sub) = argv.first() else {
        usage(state);
        return;
    };
    let rest = &argv[1..];
    match *sub {
        b"init" => init::run(state),
        b"clone" => clone::run(state, rest),
        b"push" => push::run(state, rest),
        b"add" => add::run(state, rest),
        b"status" => status::run(state),
        b"log" => log::run(state),
        b"commit" => commit::run(state, message(rest)),
        _ => usage(state),
    }
}

/// The text after `-m`, which is all `commit` takes today.
fn message<'a>(rest: &'a [&'a [u8]]) -> &'a [u8] {
    let mut i = 0;
    while i < rest.len() {
        if rest[i] == b"-m" && i + 1 < rest.len() {
            return rest[i + 1];
        }
        i += 1;
    }
    b""
}

fn usage(state: &mut State) {
    let mut out = Output::new(&mut state.scrollback);
    out.writeln(b"usage: git <init|clone <url>|add|status|commit -m <msg>|log|push <url>>");
}
