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

//! Map between the text buffer's byte offsets and the document model's
//! (block, offset) pairs. Heading blocks drop their `#` prefix, so the two
//! disagree by that much per line. Every offset returned is snapped down to a
//! UTF-8 char boundary: the model slices strings by these, and a slice off a
//! boundary panics, which at CPL=3 kills the capsule.

use super::state::State;

impl State {
    pub(super) fn snap(&self, block: usize, off: usize) -> usize {
        let s = match self.doc.blocks.get(block) {
            Some(b) => b.as_str(),
            None => return 0,
        };
        let mut off = off.min(s.len());
        while off > 0 && !s.is_char_boundary(off) {
            off -= 1;
        }
        off
    }

    fn prefix(&self, block: usize) -> usize {
        match self.doc.blocks.get(block).and_then(|b| b.kind.heading_level()) {
            Some(n) => n as usize + 1,
            None => 0,
        }
    }

    pub(super) fn doc_pos(&self, at: usize) -> (usize, usize) {
        let text = core::str::from_utf8(&self.buf[..self.len]).unwrap_or("");
        let mut base = 0usize;
        for (i, line) in text.split('\n').enumerate() {
            if at <= base + line.len() {
                let off = at.saturating_sub(base).saturating_sub(self.prefix(i));
                return (i, self.snap(i, off));
            }
            base += line.len() + 1;
        }
        (self.doc.blocks.len().saturating_sub(1), 0)
    }

    pub(super) fn doc_byte(&self, block: usize, off: usize) -> usize {
        let text = core::str::from_utf8(&self.buf[..self.len]).unwrap_or("");
        let mut base = 0usize;
        for (i, line) in text.split('\n').enumerate() {
            if i == block {
                let at = base + self.prefix(i) + self.snap(i, off);
                return at.min(base + line.len());
            }
            base += line.len() + 1;
        }
        self.len
    }
}
