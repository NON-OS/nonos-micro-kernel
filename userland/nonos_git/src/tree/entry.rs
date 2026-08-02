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

//! One entry in a tree: a mode, a name and the object it points at.

extern crate alloc;

use alloc::string::String;

use crate::oid::ObjectId;

/// The file modes git records. Git stores a small fixed set, not the full unix
/// mode, so this is an enum rather than a raw number: an unknown mode in a tree
/// is a corrupt tree, not a file to guess about.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    /// A non-executable file.
    File,
    /// An executable file.
    Executable,
    /// A symbolic link; the blob holds the target path.
    Symlink,
    /// A subdirectory, pointing at another tree.
    Directory,
    /// A commit id embedded in a tree: a submodule.
    Submodule,
}

impl Mode {
    /// The octal text git writes before the name. Note trees are `40000`, with
    /// no leading zero, which is why this is a table and not a format call.
    pub const fn as_bytes(self) -> &'static [u8] {
        match self {
            Mode::File => b"100644",
            Mode::Executable => b"100755",
            Mode::Symlink => b"120000",
            Mode::Directory => b"40000",
            Mode::Submodule => b"160000",
        }
    }

    /// Parse the mode field, rejecting anything git would not write.
    pub fn from_bytes(bytes: &[u8]) -> Option<Mode> {
        match bytes {
            b"100644" => Some(Mode::File),
            b"100755" => Some(Mode::Executable),
            b"120000" => Some(Mode::Symlink),
            b"40000" | b"040000" => Some(Mode::Directory),
            b"160000" => Some(Mode::Submodule),
            _ => None,
        }
    }

    /// Whether this entry names another tree, which decides how the sort order
    /// treats the name.
    pub const fn is_dir(self) -> bool {
        matches!(self, Mode::Directory)
    }
}

/// A single tree entry.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TreeEntry {
    pub mode: Mode,
    pub name: String,
    pub id: ObjectId,
}
