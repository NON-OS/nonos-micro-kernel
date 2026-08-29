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

//! Run-style edits over the selection. These change `doc` directly rather than
//! the text buffer, so they re-paginate instead of calling `reflow`, which
//! rebuilds `doc` from the text and would throw the new runs away.

use crate::doc::paginate::paginate;
use crate::doc::restyle::set_style;
use crate::doc::style::RunStyle;
use crate::doc::ttf_measure::TtfMeasurer;
use crate::editor::mode::Mode;
use crate::editor::state::State;

impl State {
    pub(in crate::editor) fn restyle_sel(&mut self, f: &dyn Fn(&mut RunStyle)) -> bool {
        let Some((s, e)) = self.sel_range() else {
            return false;
        };
        let (b0, o0) = self.doc_pos(s);
        let (b1, o1) = self.doc_pos(e);
        for b in b0..=b1 {
            let Some(block) = self.doc.blocks.get_mut(b) else {
                break;
            };
            let off = if b == b0 { o0 } else { 0 };
            let end = if b == b1 { o1 } else { block.text.len() };
            set_style(block, off, end.saturating_sub(off), f);
        }
        self.repaginate();
        true
    }

    pub(in crate::editor) fn repaginate(&mut self) {
        if self.mode == Mode::Document {
            self.pages = paginate(&self.doc, &self.page_metrics, &TtfMeasurer);
        }
    }
}
