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

//! What a byte of a command line belongs to, and what colour that makes it.

use crate::term::theme::{ACCENT, DIM, FOREGROUND, PATH};

/// What a byte of the line belongs to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Part {
    /// The command itself, up to the first space.
    Command,
    /// A word beginning with a dash.
    Flag,
    /// Anything inside quotes.
    Quoted,
    /// A word that names a location: it has a separator in it.
    Path,
    /// The characters that join commands together.
    Operator,
    /// Everything else.
    Plain,
}

impl Part {
    pub fn colour(self) -> u32 {
        match self {
            // The command carries the accent because it is the one word that
            // decides what the rest of the line means.
            Part::Command => ACCENT,
            Part::Flag => DIM,
            Part::Quoted => PATH,
            Part::Path => PATH,
            Part::Operator => ACCENT,
            Part::Plain => FOREGROUND,
        }
    }
}

/// Characters that end one command and begin another. A line is read from the
/// start again after any of these, so the word following a pipe is coloured
/// as the command it is.
pub fn is_operator(b: u8) -> bool {
    matches!(b, b'|' | b'>' | b'<' | b'&' | b';')
}
