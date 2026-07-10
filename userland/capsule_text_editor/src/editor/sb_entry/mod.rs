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

//! The explorer's inline name entry: typing a name for a new file, a new
//! folder, or a rename, shown as an input bar at the top of the sidebar.

mod commit;
mod key;
mod paint;

pub(in crate::editor) use key::entry_key;
pub(in crate::editor) use paint::paint_entry;

use alloc::string::String;

#[derive(Clone, Copy, PartialEq)]
pub enum EntryOp {
    NewFile,
    NewFolder,
    Rename,
}

pub struct SbEntry {
    pub op: EntryOp,
    // For creation this is the directory the name lands in; for rename it is
    // the full old path.
    pub base: String,
    pub buf: String,
}

// What a key press did to the entry: still typing, committed (tree needs a
// reload), or cancelled. Commit carries the rename pair so open tabs can
// follow the file to its new path.
pub enum EntryOutcome {
    Pending,
    Cancelled,
    Committed { renamed: Option<(String, String)> },
    Failed(&'static str),
}
