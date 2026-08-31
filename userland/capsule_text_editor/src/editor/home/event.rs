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

//! Input routing for the Home screen. The nav rows with a store behind them,
//! the document rows and the "View all" link are wired; the search field, the
//! template rows and the storeless nav rows are drawn sunk and stay idle.

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use crate::editor::widget::navlist_hit;

use super::super::app::Editor;
use super::doc_click::doc_click;
use super::metrics::{nav_rect, BODY};
use super::state::{HomeState, NAV_LABELS};

pub(crate) fn home_event(ed: &mut Editor, event: InputEvent) -> EventOutcome {
    if event.kind != InputKind::ButtonDown {
        return EventOutcome::Idle;
    }
    if let Some(row) = navlist_hit(nav_rect(), NAV_LABELS.len(), BODY, event.x, event.y) {
        return select_outcome(row);
    }
    if HomeState::view_all_hit(event.x, event.y) {
        return select_outcome(0);
    }
    doc_click(ed, event)
}

fn select_outcome(row: usize) -> EventOutcome {
    match HomeState::select(row) {
        true => EventOutcome::Repaint,
        false => EventOutcome::Idle,
    }
}
