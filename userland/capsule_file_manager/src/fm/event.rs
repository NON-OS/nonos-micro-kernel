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

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind, KEY_BACKSPACE, KEY_ENTER, KEY_ESC, KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT};

use super::refresh::refresh;
use super::state::State;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.kind == InputKind::ButtonDown {
        let row = ((event.y - 60) / 22) as isize;
        if row >= 0 && (row as usize) < state.entries.len() {
            state.cursor = row as usize;
        }
    } else if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    match if event.kind == InputKind::ButtonDown { KEY_ENTER } else { event.code } {
        KEY_ESC => EventOutcome::Close,
        KEY_ENTER => {
            if let Some(entry) = state.entries.get(state.cursor).cloned() {
                if entry.is_dir {
                    state.prefix = entry.full_path;
                    refresh(state);
                } else {
                    state.preview = Some(entry.full_path);
                    state.status = b"file selected";
                }
                return EventOutcome::Repaint;
            }
            EventOutcome::Idle
        }
        KEY_BACKSPACE | KEY_LEFT => {
            if state.prefix != "/" {
                let trim = state.prefix.trim_end_matches('/');
                let cut = trim.rfind('/').unwrap_or(0);
                state.prefix.truncate(if cut == 0 { 1 } else { cut + 1 });
                refresh(state);
                return EventOutcome::Repaint;
            }
            EventOutcome::Idle
        }
        KEY_UP => {
            state.cursor = state.cursor.saturating_sub(1);
            EventOutcome::Repaint
        }
        KEY_DOWN => {
            state.cursor = core::cmp::min(state.cursor.saturating_add(1), state.entries.len().saturating_sub(1));
            EventOutcome::Repaint
        }
        KEY_RIGHT => on_event(state, InputEvent { code: KEY_ENTER, ..event }),
        code if code == b'j' as u32 => on_event(state, InputEvent { code: KEY_DOWN, ..event }),
        code if code == b'k' as u32 => on_event(state, InputEvent { code: KEY_UP, ..event }),
        code if code == b'h' as u32 => on_event(state, InputEvent { code: KEY_LEFT, ..event }),
        code if code == b'l' as u32 => on_event(state, InputEvent { code: KEY_ENTER, ..event }),
        _ => EventOutcome::Idle,
    }
}
