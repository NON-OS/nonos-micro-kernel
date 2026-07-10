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

//! Replace, layered on find. The current match is whatever find has selected;
//! replacing rewrites it as one undoable edit and steps to the next match.
//! Replace-all loops the same primitive from the top of the buffer, so a
//! single undo per match unwinds it (bounded by the match count).

use super::state::State;

impl State {
    // Rewrite the selected match with the replacement text and move to the
    // next occurrence. True when something changed.
    pub fn replace_current(&mut self) -> bool {
        let Some((s, e)) = self.sel_range() else {
            self.find_next(true);
            return false;
        };
        if !self.matches_query(s, e) {
            self.find_next(true);
            return false;
        }
        let rep = self.replace_buf.clone();
        self.sel_anchor = None;
        if !self.apply_edit(s, e - s, rep.as_bytes()) {
            return false;
        }
        self.caret = s + rep.len();
        self.find_next(true);
        true
    }

    // Replace every match in the buffer, from the top. Returns how many were
    // rewritten. The scan restarts after each edit so shifting offsets never
    // land a replacement in the wrong place, and it refuses queries that the
    // replacement re-contains only by bounding the pass count.
    pub fn replace_all(&mut self) -> usize {
        if self.find_buf.is_empty() {
            return 0;
        }
        let bound = self.find_count();
        let mut done = 0;
        self.caret = 0;
        self.sel_anchor = None;
        self.find_incremental();
        while done < bound {
            let Some((s, e)) = self.sel_range() else { break };
            if !self.matches_query(s, e) {
                break;
            }
            let rep = self.replace_buf.clone();
            self.sel_anchor = None;
            if !self.apply_edit(s, e - s, rep.as_bytes()) {
                break;
            }
            self.caret = s + rep.len();
            done += 1;
            // find_next moves the caret to the new match's end, so remember
            // where this replacement finished: a next match starting before it
            // means the search wrapped and the pass is complete.
            let resume = self.caret;
            self.find_next(true);
            if self.sel_range().map(|(ns, _)| ns < resume).unwrap_or(true) {
                break;
            }
        }
        done
    }

    // Whether buf[s..e] equals the find query, ASCII case-insensitive, the
    // same comparison find itself uses.
    fn matches_query(&self, s: usize, e: usize) -> bool {
        let q = self.find_buf.as_bytes();
        e - s == q.len()
            && self.buf[s..e].iter().zip(q.iter()).all(|(a, b)| a.eq_ignore_ascii_case(b))
    }
}
