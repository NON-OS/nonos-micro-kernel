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

//! Opening a Home document row. The rows hit-tested here are the rows
//! `paint_docs` drew: both sides take their geometry from `docs_list_rect`.

use nonos_app_skeleton::{EventOutcome, InputEvent};

use crate::editor::widget::docrow_hit;

use super::super::app::Editor;
use super::super::screen::Screen;
use super::docs::doc_list;
use super::metrics_pane::{doc_row_h, docs_list_rect};
use super::state::HomeState;

pub(super) fn doc_click(ed: &mut Editor, event: InputEvent) -> EventOutcome {
    let nav = HomeState::load().nav;
    let list = doc_list(ed, nav);
    let (w, h) = (HomeState::painted_width(), HomeState::painted_height());
    let (x, y, rw, total) = docs_list_rect(w, h, list.len());
    let rh = doc_row_h();
    let rows = (total / rh) as usize;
    let hit = docrow_hit((x, y, rw), rh, rows, event.x, event.y);
    let path = match hit.and_then(|i| list.get(i)) {
        Some(d) => d.path.clone(),
        None => return EventOutcome::Idle,
    };
    ed.open_path(&path);
    ed.screen = Screen::Editor;
    EventOutcome::Repaint
}
