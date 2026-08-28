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

//! Rebuild the styled document from the text buffer and re-paginate it. Called
//! from `splice`, the one place every mutation goes through, so undo and redo
//! re-flow without a second path.

use super::mode::Mode;
use super::state::State;
use crate::doc::paginate::paginate;
use crate::doc::text_bridge::doc_from_text;
use crate::doc::ttf_measure::TtfMeasurer;

impl State {
    pub(super) fn reflow(&mut self) {
        if self.mode != Mode::Document {
            return;
        }
        self.doc = doc_from_text(&self.buf[..self.len]);
        self.pages = paginate(&self.doc, &self.page_metrics, &TtfMeasurer);
    }
}
