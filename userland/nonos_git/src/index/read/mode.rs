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

//! The mode word on an index entry.

use crate::tree::Mode;

/// The mode git wrote, refusing anything it would not have.
pub(super) fn mode_from_word(word: u32) -> Option<Mode> {
    match word {
        0o100_644 => Some(Mode::File),
        0o100_755 => Some(Mode::Executable),
        0o120_000 => Some(Mode::Symlink),
        0o160_000 => Some(Mode::Submodule),
        _ => None,
    }
}
