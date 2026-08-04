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
//! Why a request or response failed.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HttpError {
    /// The response ended before its headers did.
    Incomplete,
    /// The status line was not `HTTP/1.x <code> ...`.
    StatusLine,
    /// A header line had no colon, or a name that is not a token.
    Header,
    /// The body length could not be determined, or did not arrive in full.
    Body,
    /// A chunked body had a malformed size line.
    Chunk,
    /// The response claimed a body larger than the caller allows.
    TooLarge,
    /// The underlying stream failed.
    Io,
}
