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

//! Byte offsets of the newline-delimited line containing a position. These are
//! text lines, not the wrapped visual lines used for caret paging.

pub(super) fn line_start(buf: &[u8], at: usize) -> usize {
    let at = at.min(buf.len());
    buf[..at].iter().rposition(|&b| b == b'\n').map(|p| p + 1).unwrap_or(0)
}

pub(super) fn line_end(buf: &[u8], at: usize) -> usize {
    let at = at.min(buf.len());
    buf[at..].iter().position(|&b| b == b'\n').map(|p| at + p).unwrap_or(buf.len())
}
