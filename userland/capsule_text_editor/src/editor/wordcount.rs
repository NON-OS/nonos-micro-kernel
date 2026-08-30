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

//! Tools > Word Count rows. The counts come from the shared `doc::counts`
//! helpers over a document rebuilt from the text buffer, so the panel agrees
//! with the model in code mode as well as document mode.

use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::state::State;
use crate::doc::counts::{char_count, word_count};
use crate::doc::kind::BlockKind;
use crate::doc::text_bridge::doc_from_text;

pub(in crate::editor) fn count_rows(state: &State) -> Vec<String> {
    let doc = doc_from_text(&state.buf[..state.len]);
    let paragraphs = doc
        .blocks
        .iter()
        .filter(|b| b.kind != BlockKind::PageBreak && !b.as_str().trim().is_empty())
        .count();
    vec![
        format!("Words: {}", word_count(&doc)),
        format!("Characters: {}", char_count(&doc)),
        format!("Paragraphs: {}", paragraphs),
        String::from("Click anywhere to close"),
    ]
}
