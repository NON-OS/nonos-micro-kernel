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
    EventOutcome, InputEvent, InputKind, KEY_BACKSPACE, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_LEFT,
    KEY_RIGHT, KEY_TAB, KEY_UP,
};
use nonos_policy_proto::Category;

use crate::settings::manifest::{HEIGHT, WIDTH};
use crate::settings::paint::{
    layout::{BODY_TOP, HEADER_H, ROW_H, STATUS_H, TAB_H, VALUE_LEFT},
    visible_rows::visible_rows,
};
use crate::settings::state::{
    cursor_down, cursor_up, edit_cancel, State, focused_count::focused_count,
    set_category::set_category, track_scroll::track_scroll,
};

use super::adjust::adjust;
use super::commit_string::commit_string;
use super::next_category::next_category;
use super::prev_category::prev_category;
use super::push_text_char::push_text_char;
use super::toggle_or_inc::toggle_or_inc;

const KEY_SPACE: u32 = 0x20;
const TAB_WIDTH: u32 = WIDTH / 3;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.kind == InputKind::ButtonDown {
        return on_pointer(state, event.x, event.y);
    }
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    if state.editing {
        return on_event_editing(state, event.code);
    }
    on_event_browsing(state, event.code)
}

fn on_event_editing(state: &mut State, code: u32) -> EventOutcome {
    match code {
        KEY_ESC => {
            edit_cancel(state);
            EventOutcome::Repaint
        }
        KEY_ENTER => {
            commit_string(state);
            EventOutcome::Repaint
        }
        KEY_BACKSPACE => {
            state.edit.pop();
            EventOutcome::Repaint
        }
        c => {
            if push_text_char(state, c) {
                EventOutcome::Repaint
            } else {
                EventOutcome::Idle
            }
        }
    }
}

fn on_event_browsing(state: &mut State, code: u32) -> EventOutcome {
    match code {
        KEY_ESC => EventOutcome::Close,
        KEY_TAB => {
            next_category(state);
            EventOutcome::Repaint
        }
        KEY_UP => {
            cursor_up(state);
            EventOutcome::Repaint
        }
        KEY_DOWN => {
            cursor_down(state);
            EventOutcome::Repaint
        }
        KEY_LEFT => {
            adjust(state, -1);
            EventOutcome::Repaint
        }
        KEY_RIGHT => {
            adjust(state, 1);
            EventOutcome::Repaint
        }
        KEY_SPACE | KEY_ENTER => {
            toggle_or_inc(state);
            EventOutcome::Repaint
        }
        c if c == b'[' as u32 => {
            prev_category(state);
            EventOutcome::Repaint
        }
        c if c == b']' as u32 => {
            next_category(state);
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}

fn on_pointer(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if x < 0 || y < 0 {
        return EventOutcome::Idle;
    }
    let (x, y) = (x as u32, y as u32);
    if y >= HEADER_H && y < HEADER_H + TAB_H {
        set_category(
            state,
            match core::cmp::min(x / TAB_WIDTH, 2) {
                0 => Category::User,
                1 => Category::Identity,
                _ => Category::Kernel,
            },
        );
        return EventOutcome::Repaint;
    }
    if y < BODY_TOP || y >= HEIGHT.saturating_sub(STATUS_H) {
        return EventOutcome::Idle;
    }
    let row = ((y - BODY_TOP) / ROW_H) as usize;
    let cat = state.category as usize;
    let idx = state.scroll_top[cat] + row;
    if row >= visible_rows() || idx >= focused_count(state.category) {
        return EventOutcome::Idle;
    }
    state.cursor[cat] = idx;
    track_scroll(state);
    if x < VALUE_LEFT {
        return EventOutcome::Repaint;
    }
    if x < VALUE_LEFT + 96 {
        adjust(state, -1);
    } else if x > WIDTH.saturating_sub(120) {
        adjust(state, 1);
    } else {
        toggle_or_inc(state);
    }
    EventOutcome::Repaint
}
