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

use crate::about::section::SECTIONS;
use crate::about::state::State;
use crate::about::theme::{
    BODY, HEADER_HEIGHT, HEADLINE, TAB_BAR, TAB_BAR_HEIGHT, TAB_HORIZONTAL_PADDING, TAB_SELECTED,
};

const TAB_CELL_WIDTH: u32 = 8;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.fill_rect(0, HEADER_HEIGHT, fb.width, TAB_BAR_HEIGHT, TAB_BAR);
    let mut x = TAB_HORIZONTAL_PADDING;
    for section in SECTIONS {
        let title = section.title();
        let label_w = title.len() as u32 * TAB_CELL_WIDTH;
        let tab_w = label_w + TAB_HORIZONTAL_PADDING * 2;
        let selected = state.section == section;
        if selected {
            fb.fill_rect(x, HEADER_HEIGHT, tab_w, TAB_BAR_HEIGHT, TAB_SELECTED);
        }
        let color = if selected { HEADLINE } else { BODY };
        fb.text(x + TAB_HORIZONTAL_PADDING, HEADER_HEIGHT + 8, title, color);
        x += tab_w;
    }
}
