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

//! Input routing for the Home screen. The nav selection is the only wired
//! control; the search field and the document and template rows are drawn sunk,
//! so a click on one of them stays idle rather than pretending to act.

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use crate::editor::widget::navlist_hit;

use super::super::app::Editor;
use super::metrics::{nav_rect, BODY};
use super::state::{HomeState, NAV_LABELS};

pub(crate) fn home_event(_ed: &mut Editor, event: InputEvent) -> EventOutcome {
    if event.kind != InputKind::ButtonDown {
        return EventOutcome::Idle;
    }
    if let Some(row) = navlist_hit(nav_rect(), NAV_LABELS.len(), BODY, event.x, event.y) {
        return match HomeState::select(row) {
            true => EventOutcome::Repaint,
            false => EventOutcome::Idle,
        };
    }
    EventOutcome::Idle
}
