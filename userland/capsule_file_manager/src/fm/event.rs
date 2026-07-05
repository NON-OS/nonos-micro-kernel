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

use super::clipboard;
use super::duplicate;
use super::filter;
use super::layout::{FIRST_ROW_Y, ROW_HEIGHT};
use super::preview;
use super::prompt;
use super::refresh::refresh;
use super::scroll::ensure_visible;
use super::selection;
use super::state::{Mode, PromptKind, State};
use super::view::rebuild_view;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.kind == InputKind::ButtonDown {
        if matches!(state.mode, Mode::Browse) {
            let rel = event.y - FIRST_ROW_Y as i32 + 4;
            if rel >= 0 {
                let row = state.scroll + (rel as u32 / ROW_HEIGHT) as usize;
                if row < state.entries.len() {
                    state.cursor = row;
                }
            }
        }
    } else if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    match state.mode {
        Mode::Prompt(_) => return prompt::on_key(state, event),
        Mode::Preview => return preview::on_key(state, event),
        Mode::Filter => return filter::on_key(state, event),
        Mode::Help => return super::help::on_key(state, event),
        Mode::Browse => {}
    }
    match if event.kind == InputKind::ButtonDown { KEY_ENTER } else { event.code } {
        KEY_ESC => EventOutcome::Close,
        KEY_ENTER => {
            if let Some(entry) = state.entries.get(state.cursor).cloned() {
                if entry.is_dir {
                    state.prefix = entry.full_path;
                    state.cursor = 0;
                    state.scroll = 0;
                    selection::clear(state);
                    refresh(state);
                } else {
                    preview::open_preview(state, entry.full_path);
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
                state.cursor = 0;
                state.scroll = 0;
                selection::clear(state);
                refresh(state);
                return EventOutcome::Repaint;
            }
            EventOutcome::Idle
        }
        KEY_UP => {
            state.cursor = state.cursor.saturating_sub(1);
            ensure_visible(state);
            EventOutcome::Repaint
        }
        KEY_DOWN => {
            state.cursor = core::cmp::min(state.cursor.saturating_add(1), state.entries.len().saturating_sub(1));
            ensure_visible(state);
            EventOutcome::Repaint
        }
        KEY_RIGHT => on_event(state, InputEvent { code: KEY_ENTER, ..event }),
        code if code == b'j' as u32 => on_event(state, InputEvent { code: KEY_DOWN, ..event }),
        code if code == b'k' as u32 => on_event(state, InputEvent { code: KEY_UP, ..event }),
        code if code == b'h' as u32 => on_event(state, InputEvent { code: KEY_LEFT, ..event }),
        code if code == b'l' as u32 => on_event(state, InputEvent { code: KEY_ENTER, ..event }),
        code if code == b'n' as u32 => start_prompt(state, PromptKind::NewFile, b"new file: "),
        code if code == b'm' as u32 => start_prompt(state, PromptKind::MkDir, b"mkdir: "),
        code if code == b'r' as u32 => {
            // Pre-fill with the current name so a rename edits rather than retypes.
            state.mode = Mode::Prompt(PromptKind::Rename);
            state.input = state
                .entries
                .get(state.cursor)
                .map(|e| alloc::string::String::from(e.label.trim_end_matches('/')))
                .unwrap_or_default();
            state.status = b"rename to: ";
            EventOutcome::Repaint
        }
        code if code == b'd' as u32 => start_prompt(state, PromptKind::Delete, b"delete? type y + Enter: "),
        code if code == b'c' as u32 => {
            clipboard::yank(state, false);
            EventOutcome::Repaint
        }
        code if code == b'x' as u32 => {
            clipboard::yank(state, true);
            EventOutcome::Repaint
        }
        code if code == b'p' as u32 => {
            clipboard::paste(state);
            EventOutcome::Repaint
        }
        code if code == b's' as u32 => {
            state.sort_mode = state.sort_mode.next();
            rebuild_view(state);
            state.status = b"sorted";
            EventOutcome::Repaint
        }
        code if code == b'o' as u32 => {
            state.sort_rev = !state.sort_rev;
            rebuild_view(state);
            state.status = b"order reversed";
            EventOutcome::Repaint
        }
        code if code == b'/' as u32 => {
            state.mode = Mode::Filter;
            state.status = b"filter: ";
            EventOutcome::Repaint
        }
        code if code == b' ' as u32 => {
            selection::toggle(state);
            state.cursor = core::cmp::min(state.cursor + 1, state.entries.len().saturating_sub(1));
            ensure_visible(state);
            EventOutcome::Repaint
        }
        code if code == b'a' as u32 => {
            selection::select_all(state);
            EventOutcome::Repaint
        }
        code if code == b'y' as u32 => {
            duplicate::duplicate(state);
            EventOutcome::Repaint
        }
        code if code == b'g' as u32 => start_prompt(state, PromptKind::Goto, b"goto: "),
        code if code == b'w' as u32 => {
            super::perms::toggle_readonly(state);
            EventOutcome::Repaint
        }
        code if code == b'?' as u32 => {
            state.mode = Mode::Help;
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}

fn start_prompt(state: &mut State, kind: PromptKind, status: &'static [u8]) -> EventOutcome {
    state.mode = Mode::Prompt(kind);
    state.input = alloc::string::String::new();
    state.status = status;
    EventOutcome::Repaint
}
