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

//! A cursor over a commit's newline-separated header lines.

/// Walks header lines and can hand back the remaining bytes verbatim once the
/// header ends, which is what keeps the message byte-exact.
pub(super) struct Lines<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Lines<'a> {
    pub(super) fn new(data: &'a [u8]) -> Lines<'a> {
        Lines { data, pos: 0 }
    }

    /// The next line, without its newline. `None` once past the end.
    pub(super) fn next(&mut self) -> Option<&'a [u8]> {
        if self.pos > self.data.len() {
            return None;
        }
        if self.pos == self.data.len() {
            self.pos += 1;
            return Some(&self.data[self.data.len()..]);
        }
        let end = self.data[self.pos..]
            .iter()
            .position(|b| *b == b'\n')
            .map(|p| p + self.pos)
            .unwrap_or(self.data.len());
        let line = &self.data[self.pos..end];
        self.pos = end + 1;
        Some(line)
    }

    /// Everything not yet consumed: the message.
    pub(super) fn rest(&self) -> &'a [u8] {
        if self.pos >= self.data.len() {
            &self.data[self.data.len()..]
        } else {
            &self.data[self.pos..]
        }
    }
}
