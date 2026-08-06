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

/// The mark that opens an echoed command line, as UTF-8.
///
/// This was a bare 0xD8, which is what the character looks like in Latin-1
/// and is not a character at all in UTF-8: it is the first byte of one. It
/// rendered while the grid held bytes and became a replacement mark the
/// moment the grid held characters.
pub const PROMPT_BYTES: &[u8] = "\u{00D8} ".as_bytes();
