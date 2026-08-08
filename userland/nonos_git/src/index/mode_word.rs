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

//! The mode word an index entry carries.

use crate::index::entry::IndexEntry;
use crate::tree::Mode;

/// The object type in the high bits and the permission in the low ones, which
/// is what `100644` means as a number.
pub(super) fn mode_word(entry: &IndexEntry) -> u32 {
    match entry.mode {
        Mode::File => 0o100_644,
        Mode::Executable => 0o100_755,
        Mode::Symlink => 0o120_000,
        // A tree is never an index entry; a submodule is recorded as a commit.
        Mode::Directory | Mode::Submodule => 0o160_000,
    }
}
