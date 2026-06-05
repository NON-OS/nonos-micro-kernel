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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    /// Buffer ended before the next field could be read.
    Short,
    /// A length-prefixed field exceeded its declared upper bound.
    TooLarge,
    /// A list count exceeded its declared upper bound.
    TooManyItems,
    /// A UTF-8 string was not valid.
    BadUtf8,
    /// A schema version we do not understand.
    UnsupportedSchema,
    /// Encoded blob is larger than `MAX_INDEX_BLOB`.
    BlobTooLarge,
}
