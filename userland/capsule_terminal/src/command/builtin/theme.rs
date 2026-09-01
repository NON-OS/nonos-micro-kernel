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

//! Palette profiles for the terminal. Each names a complete `Theme`; the
//! translucent ones carry an alpha below 0xFF, which the compositor blends so
//! the desktop shows through the window.

use crate::command::output::Output;
use crate::term::state::State;
use crate::term::theme::profiles;

const NAMES: &[&[u8]] = &[b"dark", b"dim", b"light", b"abyss"];

pub fn run(state: &mut State, argv: &[&[u8]]) {
    match argv.get(1) {
        Some(name) => match profiles::by_name(name) {
            Some(idx) => state.theme_req = Some(idx),
            None => {
                Output::new(&mut state.scrollback).writeln(b"theme: unknown profile");
                list(state);
            }
        },
        None => list(state),
    }
}

fn list(state: &mut State) {
    let mut line = alloc::vec::Vec::new();
    line.extend_from_slice(b"profiles:");
    for name in NAMES {
        line.push(b' ');
        line.extend_from_slice(name);
    }
    Output::new(&mut state.scrollback).writeln(&line);
}
