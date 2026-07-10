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

//! Incremental find. Case-insensitive substring search that wraps around the
//! buffer, selecting each match so it lands under the caret and is highlighted.

use super::state::State;

impl State {
    /// Move to the next (or previous) match of the query, selecting it. Search
    /// wraps around the end of the buffer.
    pub fn find_next(&mut self, forward: bool) {
        let needle = self.find_buf.as_bytes();
        if needle.is_empty() || needle.len() > self.len {
            return;
        }
        let hay = &self.buf[..self.len];
        let start = if forward {
            self.caret.saturating_add(1).min(self.len)
        } else {
            self.caret.saturating_sub(1)
        };
        if let Some(pos) = search(hay, needle, start, forward) {
            self.sel_anchor = Some(pos);
            self.caret = pos + needle.len();
        }
    }

    /// Refresh the match while typing: search from the current selection start
    /// so refining the query keeps the match under the caret when it still fits.
    pub fn find_incremental(&mut self) {
        let needle = self.find_buf.as_bytes();
        if needle.is_empty() || needle.len() > self.len {
            self.sel_anchor = None;
            return;
        }
        let hay = &self.buf[..self.len];
        let start = self.sel_range().map(|(s, _)| s).unwrap_or(self.caret).min(self.len);
        if let Some(pos) = search(hay, needle, start, true) {
            self.sel_anchor = Some(pos);
            self.caret = pos + needle.len();
        }
    }

    /// Total number of matches, for the find bar's count.
    pub fn find_count(&self) -> usize {
        let needle = self.find_buf.as_bytes();
        if needle.is_empty() {
            return 0;
        }
        let hay = &self.buf[..self.len];
        let mut n = 0;
        let mut i = 0;
        while i + needle.len() <= hay.len() {
            if match_at(hay, needle, i) {
                n += 1;
                i += needle.len();
            } else {
                i += 1;
            }
        }
        n
    }
}

fn match_at(hay: &[u8], needle: &[u8], i: usize) -> bool {
    i + needle.len() <= hay.len()
        && hay[i..i + needle.len()].iter().zip(needle).all(|(&h, &n)| h.eq_ignore_ascii_case(&n))
}

// First match at or after `start` scanning in the given direction, wrapping
// once around the whole buffer.
fn search(hay: &[u8], needle: &[u8], start: usize, forward: bool) -> Option<usize> {
    let last = hay.len().saturating_sub(needle.len());
    if forward {
        (start..=last)
            .find(|&i| match_at(hay, needle, i))
            .or_else(|| (0..start.min(last + 1)).find(|&i| match_at(hay, needle, i)))
    } else {
        (0..=start.min(last))
            .rev()
            .find(|&i| match_at(hay, needle, i))
            .or_else(|| (start.min(last) + 1..=last).rev().find(|&i| match_at(hay, needle, i)))
    }
}
