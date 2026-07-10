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

//! Host proofs for the text editor's document engine. Each `#[path]` include
//! pulls in the real editor source so the tests pin production editing logic,
//! not a copy. The files are included flat at the crate root: for a root-level
//! module, `super::` resolves to the crate root, so their `super::state::State`
//! style imports line up with the sibling includes below.

extern crate alloc;

#[path = "../../capsule_text_editor/src/editor/autoclose.rs"]
pub mod autoclose;
#[path = "../../capsule_text_editor/src/editor/backspace.rs"]
pub mod backspace;
#[path = "../../capsule_text_editor/src/editor/byte_at.rs"]
pub mod byte_at;
#[path = "../../capsule_text_editor/src/editor/caret_nav.rs"]
pub mod caret_nav;
#[path = "../../capsule_text_editor/src/editor/clamp_scroll.rs"]
pub mod clamp_scroll;
#[path = "../../capsule_text_editor/src/editor/delete.rs"]
pub mod delete;
#[path = "../../capsule_text_editor/src/editor/edit.rs"]
pub mod edit;
#[path = "../../capsule_text_editor/src/editor/find.rs"]
pub mod find;
#[path = "../../capsule_text_editor/src/editor/highlight.rs"]
pub mod highlight;
#[path = "../../capsule_text_editor/src/editor/indent.rs"]
pub mod indent;
#[path = "../../capsule_text_editor/src/editor/insert.rs"]
pub mod insert;
#[path = "../../capsule_text_editor/src/editor/insert_newline.rs"]
pub mod insert_newline;
#[path = "../../capsule_text_editor/src/editor/language.rs"]
pub mod language;
#[path = "../../capsule_text_editor/src/editor/layout.rs"]
pub mod layout;
#[path = "../../capsule_text_editor/src/editor/line_bounds.rs"]
pub mod line_bounds;
#[path = "../../capsule_text_editor/src/editor/line_ops.rs"]
pub mod line_ops;
#[path = "../../capsule_text_editor/src/editor/max_scroll.rs"]
pub mod max_scroll;
#[path = "../../capsule_text_editor/src/editor/position_at.rs"]
pub mod position_at;
#[path = "../../capsule_text_editor/src/editor/replace.rs"]
pub mod replace;
#[path = "../../capsule_text_editor/src/editor/select_word.rs"]
pub mod select_word;
#[path = "../../capsule_text_editor/src/editor/selection.rs"]
pub mod selection;
#[path = "../../capsule_text_editor/src/editor/state.rs"]
pub mod state;
#[path = "../../capsule_text_editor/src/editor/state_new.rs"]
pub mod state_new;
#[path = "../../capsule_text_editor/src/editor/theme.rs"]
pub mod theme;
#[path = "../../capsule_text_editor/src/editor/toggle_comment.rs"]
pub mod toggle_comment;
#[path = "../../capsule_text_editor/src/editor/visual_lines.rs"]
pub mod visual_lines;
#[path = "../../capsule_text_editor/src/editor/word_nav.rs"]
pub mod word_nav;

#[cfg(test)]
mod edit_tests;
#[cfg(test)]
mod feature_tests;
