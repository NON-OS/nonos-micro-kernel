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

use nonos_app_skeleton::{EventOutcome, KEY_ENTER};

use super::event_browse::on_browse_key;
use super::event_mouse::select_row;
use super::layout::{content_w, content_x};
use super::list_check::check_hit;
use super::list_cols::{cols, Cols, HEAD_H};
use super::selection::toggle;
use super::state::{SortMode, State};
use super::store_meta::save_meta;
use super::view::rebuild_view;

/// A click inside the detail list, resolved against the one `list_cols` layout
/// the painter drew from: the header strip first, then the checkbox gutter of
/// the row that was hit, and only then the row body.
pub fn list_click(state: &mut State, x: u32, y: u32) -> EventOutcome {
    let c = cols(content_x(), content_w(state.win_w));
    if y >= state.row_top && y < state.row_top + HEAD_H {
        return match col_at(&c, x) {
            Some(mode) => sort_by(state, mode),
            None => EventOutcome::Idle,
        };
    }
    if !select_row(state, y) {
        return EventOutcome::Idle;
    }
    if check_hit(&c, x) {
        toggle(state);
        return EventOutcome::Repaint;
    }
    on_browse_key(state, KEY_ENTER)
}

fn col_at(c: &Cols, x: u32) -> Option<SortMode> {
    c.cols.iter().find(|col| x >= col.x && x < col.x + col.w).map(|col| col.mode)
}

/// Select a sort directly, rather than cycling the way the header pill does.
/// Writes through to `prefs` for the same reason `event_head` does: the drawn
/// caret and the persisted preference must not be able to disagree.
fn sort_by(state: &mut State, mode: SortMode) -> EventOutcome {
    state.sort_mode = mode;
    state.prefs.sort = mode;
    rebuild_view(state);
    save_meta(state);
    EventOutcome::Repaint
}
