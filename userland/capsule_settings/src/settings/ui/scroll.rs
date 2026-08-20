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

use crate::settings::schema::blocks_for;
use crate::settings::schema::rows::Row;
use crate::settings::state::State;

use super::metrics::PANE_PAD_TOP;
use super::walk::{content_h, walk, Item};

pub fn max_scroll(state: &State, view_h: u32) -> u32 {
    content_h(state).saturating_sub(view_h)
}

/// Scroll far enough that the keyboard cursor's row is fully inside the pane,
/// and no further. Keyboard and pointer share the pane's scroll offset, so a
/// separate row-index scroll would fight the wheel.
pub fn ensure_visible(state: &State, scroll: u32, view_h: u32) -> u32 {
    let Some((y, h)) = cursor_rect(state) else { return scroll.min(max_scroll(state, view_h)) };
    let mut next = scroll;
    if y < next + PANE_PAD_TOP {
        next = y.saturating_sub(PANE_PAD_TOP);
    }
    let bottom = y + h;
    if bottom > next + view_h {
        next = bottom - view_h;
    }
    next.min(max_scroll(state, view_h))
}

fn cursor_rect(state: &State) -> Option<(u32, u32)> {
    let blocks = blocks_for(state.section);
    let cursor = state.cursor[state.section.index()];
    let mut index = 0usize;
    let mut found = None;
    walk(state, |y, h, item| {
        let Item::Row(bi, ri) = item else { return };
        if !matches!(blocks[bi].rows[ri], Row::Field(_)) {
            return;
        }
        if index == cursor {
            found = Some((y, h));
        }
        index += 1;
    });
    found
}
