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

use nonos_app_skeleton::{
    EventOutcome, InputEvent, KEY_BACKSPACE, KEY_DELETE, KEY_DOWN, KEY_END, KEY_ENTER, KEY_ESC,
    KEY_HOME, KEY_LEFT, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_RIGHT, KEY_TAB, KEY_UP, MOD_CTRL,
};

use super::bool_to_outcome::bool_to_outcome;
use super::on_ctrl::on_ctrl;
use super::on_down::on_down;
use super::on_enter::on_enter;
use super::on_printable::on_printable;
use super::on_tab::on_tab;
use super::on_up::on_up;
use crate::term::dimensions::VISIBLE_ROWS;
use crate::term::state::State;

pub fn on_key(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.flags & MOD_CTRL != 0 {
        if let Some(out) = on_ctrl(state, event.code, event.flags) {
            return out;
        }
    }
    if state.fg_running && event.flags & MOD_CTRL == 0 {
        match event.code {
            KEY_ENTER => return super::fg_stdin::forward(state, b'\n'),
            code if (0x20..=0x7E).contains(&code) => {
                return super::fg_stdin::forward(state, code as u8)
            }
            _ => {}
        }
    }
    match event.code {
        // Clears the line. Esc used to close the window, so one stray press
        // took the scrollback, history and working directory with it. Closing
        // is the titlebar's job.
        KEY_ESC => {
            state.line.clear();
            EventOutcome::Repaint
        }
        KEY_ENTER => on_enter(state),
        KEY_BACKSPACE if state.search.is_some() => {
            super::search_edit::search_backspace(state);
            EventOutcome::Repaint
        }
        KEY_BACKSPACE => bool_to_outcome(state.line.backspace()),
        KEY_DELETE => bool_to_outcome(state.line.delete()),
        KEY_LEFT => bool_to_outcome(state.line.move_left()),
        KEY_RIGHT => bool_to_outcome(state.line.move_right()),
        KEY_HOME => {
            state.line.move_home();
            EventOutcome::Repaint
        }
        KEY_END => {
            state.line.move_end();
            EventOutcome::Repaint
        }
        KEY_UP => on_up(state),
        KEY_DOWN => on_down(state),
        KEY_PAGE_UP => {
            state.scrollback.scroll_up(VISIBLE_ROWS - 2);
            EventOutcome::Repaint
        }
        KEY_PAGE_DOWN => {
            state.scrollback.scroll_down(VISIBLE_ROWS - 2);
            EventOutcome::Repaint
        }
        KEY_TAB => on_tab(state),
        code if (0x20..=0x7E).contains(&code) => on_printable(state, code as u8),
        _ => EventOutcome::Idle,
    }
}
