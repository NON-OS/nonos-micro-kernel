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

use nonos_app_skeleton::{EventOutcome, InputEvent, KEY_DOWN, KEY_ESC, KEY_UP};

use super::preview_paint::VISIBLE_LINES;
use super::state::{Mode, State};

pub fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    let Some(preview) = state.preview.as_mut() else { return EventOutcome::Idle };
    match event.code {
        KEY_ESC => {
            state.mode = Mode::Browse;
            state.status = b"click or Enter to open";
        }
        KEY_UP => {
            preview.scroll = preview.scroll.saturating_sub(1);
        }
        KEY_DOWN => {
            let max = preview.lines.len().saturating_sub(VISIBLE_LINES);
            preview.scroll = preview.scroll.saturating_add(1).min(max);
        }
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
