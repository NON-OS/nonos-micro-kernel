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

//! Why a zlib/DEFLATE stream is refused.

/// Every variant is a refusal, so a truncated or corrupt object never decodes
/// to usable-looking bytes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InflateError {
    /// Missing the two-byte zlib header or its check failed.
    Header,
    /// Ran out of input mid-stream.
    Truncated,
    /// A reserved block type or an invalid Huffman/back-reference.
    Invalid,
    /// The trailing Adler-32 did not match the decoded data.
    Checksum,
}
