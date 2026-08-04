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

//! Which part a whole word belongs to.

use super::part::{is_operator, Part};

/// Which part the word starting at `start` belongs to.
pub(super) fn word_part(line: &[u8], start: usize, first_word: bool) -> Part {
    if first_word {
        return Part::Command;
    }
    if line.get(start) == Some(&b'-') {
        return Part::Flag;
    }
    // A word carrying a separator names a location. Checking the whole word
    // rather than its first byte means a path is coloured from its first
    // character, not from the slash part way along it.
    let end = line[start..]
        .iter()
        .position(|&c| c == b' ' || c == b'\t' || is_operator(c))
        .map(|n| start + n)
        .unwrap_or(line.len());
    if line[start..end].contains(&b'/') {
        return Part::Path;
    }
    Part::Plain
}
