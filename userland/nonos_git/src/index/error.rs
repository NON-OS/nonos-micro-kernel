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

//! Why an index could not be read.

/// Every variant is a refusal: a damaged index is never partially believed,
/// since it decides what the next commit contains.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IndexError {
    /// Not a `DIRC` file.
    Magic,
    /// A version this does not implement. Only version 2 is written and read.
    Version(u32),
    /// The file ended inside the header, an entry, or the trailer.
    Truncated,
    /// The trailing SHA-1 does not cover the bytes before it.
    Checksum,
    /// An entry held a mode git would not write, or a path that is empty,
    /// absolute, or walks upward.
    Entry,
}
