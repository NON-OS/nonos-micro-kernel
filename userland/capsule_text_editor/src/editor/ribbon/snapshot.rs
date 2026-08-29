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

//! What the ribbon shows for the current caret, read straight off the document
//! model so a pill always names the value that is really in effect and a toggle
//! lights only when the run under the caret carries it.

use alloc::format;
use alloc::string::{String, ToString};

use super::items::{FONTS, HEADINGS};
use crate::doc::style::{Family, RunStyle};
use crate::editor::state::State;
use crate::editor::theme;

pub(in crate::editor) struct RibbonState {
    pub style: String,
    pub font: String,
    pub size: String,
    pub flags: [bool; 5],
}

pub(in crate::editor) fn ribbon_state(st: &State) -> RibbonState {
    let (b, off) = st.doc_pos(st.caret);
    let block = st.doc.blocks.get(b);
    let level = block.and_then(|x| x.kind.heading_level()).unwrap_or(0);
    let run = block.map(|x| x.style_at(off)).unwrap_or_else(RunStyle::body);
    let accent = theme::active().accent;
    RibbonState {
        style: HEADINGS[level.min(6) as usize].0.to_string(),
        font: FONTS[usize::from(run.family == Family::Mono)].to_string(),
        size: format!("{}", (run.size_px + 0.5) as u32),
        flags: [run.bold, run.italic, run.underline, run.strike, run.color == accent],
    }
}
