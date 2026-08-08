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
//! Why a pack cannot be read.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PackError {
    /// Not a `PACK` file.
    Magic,
    /// A version this does not implement. Only version 2 and 3 are read.
    Version(u32),
    /// The file ended inside the header, an object, or the trailer.
    Truncated,
    /// The trailing SHA-1 does not cover the bytes before it.
    Checksum,
    /// An object header named a type the format does not define.
    ObjectType(u8),
    /// A zlib stream inside the pack did not inflate.
    Corrupt,
    /// A delta named a base that is not in this pack.
    MissingBase,
    /// A delta's instructions ran past the end of its base or target.
    BadDelta,
    /// An object's content does not hash to the id the pack implies.
    IdMismatch,
}
