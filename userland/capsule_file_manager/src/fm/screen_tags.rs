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

use nonos_app_skeleton::PaintBuffer;

use super::chip::chip;
use super::file_color::color;
use super::file_kind::kind_of_name;
use super::layout::{CONTENT_X, HEADER_H, PAD_X};
use super::recents_group::parent_of;
use super::screen_row::{empty_state, screen_row, section_label};
use super::sidebar_rows::base_label;
use super::state::State;
use super::tags_geom::{chip_slots, tag_names};
use super::tags_rows::tag_lines;

/// Chips come from `chip_slots` and rows from `tag_lines`, the same two passes
/// `screen_hit` searches, so every pill and row on this surface is clickable
/// exactly where it was drawn.
pub fn paint_tags(state: &State, fb: &mut PaintBuffer) {
    let x = CONTENT_X + PAD_X;
    let w = fb.width.saturating_sub(CONTENT_X + PAD_X * 2);
    if tag_names(&state.tags).is_empty() {
        empty_state(fb, x, HEADER_H + 40, w, "No tags yet", "Tag a file to gather it here.");
        return;
    }
    section_label(fb, x, HEADER_H + 16, "TAGS");
    for slot in chip_slots(state) {
        chip(fb, slot.x, slot.y, slot.name.as_str(), slot.name == state.tag_filter);
    }
    for line in &tag_lines(state) {
        let path = line.path.as_str();
        let title = base_label(path);
        let row = (title.as_str(), parent_of(path), line.meta.as_str());
        screen_row(fb, x, line.y, w, row, line.dir, color(kind_of_name(path)));
    }
}
