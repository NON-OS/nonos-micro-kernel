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

//! Why a byte slice is not a well-formed commit.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CommitError {
    /// No `tree` line, or it did not come first.
    Tree,
    /// A `parent` line held something that is not an object id.
    Parent,
    /// An `author` or `committer` line was missing or unparseable.
    Signature,
    /// The header was not terminated by a blank line.
    Header,
    /// The message was not valid UTF-8.
    Message,
}
