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

//! Background profiles for the terminal. Each sets the body background; the
//! translucent ones carry an alpha below 0xFF, which the compositor blends so
//! the desktop shows through the window.

use crate::command::output::Output;
use crate::term::state::State;

// (name, ARGB background). Alpha < 0xFF means translucent.
const PROFILES: &[(&[u8], u32)] = &[
    (b"blackarch", 0xFF07_090B),
    (b"dark", 0xFF18_1A1F),
    (b"black", 0xFF00_0000),
    (b"blue", 0xFF0A_1929),
    (b"matrix", 0xFF00_1200),
    (b"light", 0xFFF5_F5F5),
    (b"glass", 0xB018_1A1F), // ~69% opaque, translucent dark
    (b"smoke", 0x8000_0000), // 50% opaque black
    (b"clear", 0x4018_1A1F), // ~25% opaque, very see-through
];

// The next profile background after `cur`, wrapping. Used by the toolbar's
// theme button to cycle through the profiles with a click.
pub fn next_bg(cur: u32) -> u32 {
    let idx = PROFILES.iter().position(|&(_, bg)| bg == cur).map(|i| i + 1).unwrap_or(0);
    PROFILES[idx % PROFILES.len()].1
}

pub fn run(state: &mut State, argv: &[&[u8]]) {
    match argv.get(1) {
        Some(name) => {
            if let Some(&(_, bg)) = PROFILES.iter().find(|(n, _)| n == name) {
                state.bg = bg;
            } else {
                Output::new(&mut state.scrollback).writeln(b"theme: unknown profile");
                list(state);
            }
        }
        None => list(state),
    }
}

fn list(state: &mut State) {
    let mut line = alloc::vec::Vec::new();
    line.extend_from_slice(b"profiles:");
    for (name, _) in PROFILES {
        line.push(b' ');
        line.extend_from_slice(name);
    }
    Output::new(&mut state.scrollback).writeln(&line);
}
