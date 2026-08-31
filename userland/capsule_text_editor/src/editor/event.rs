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
    EventOutcome, InputEvent, InputKind, KEY_BACKSPACE, KEY_DELETE, KEY_ENTER, KEY_ESC, KEY_TAB,
    MOD_CTRL, MOD_SHIFT,
};

use super::click_caret::click_caret;
use super::follow_caret::follow_caret;
use super::on_ctrl::on_ctrl;
use super::on_nav::on_nav;
use super::path_prompt;
use super::state::State;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    let rows = state.visible_rows;
    let shift = event.flags & MOD_SHIFT != 0;

    // Pointer: press starts (or, with Shift, extends) a selection at the click;
    // dragging extends it; release ends the drag. Two quick presses in the
    // same spot select the word under them.
    if event.kind == InputKind::ButtonDown {
        let double = event.timestamp_ns.saturating_sub(state.last_click_ns) < 400_000_000
            && (event.x - state.last_click_x).abs() <= 3
            && (event.y - state.last_click_y).abs() <= 3;
        state.last_click_ns = event.timestamp_ns;
        state.last_click_x = event.x;
        state.last_click_y = event.y;
        if shift {
            state.begin_sel();
            click_caret(state, event.x, event.y);
        } else {
            click_caret(state, event.x, event.y);
            state.sel_anchor = Some(state.caret);
            if double {
                state.select_word();
            }
        }
        state.selecting = !double;
        follow_caret(state, rows);
        return EventOutcome::Repaint;
    }
    if event.kind == InputKind::PointerAbs && state.selecting {
        click_caret(state, event.x, event.y);
        follow_caret(state, rows);
        return EventOutcome::Repaint;
    }
    if event.kind == InputKind::ButtonUp {
        state.selecting = false;
        return EventOutcome::Idle;
    }
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    if state.prompt.is_some() {
        return path_prompt::on_key(state, event);
    }
    // While the find bar is open, non-Ctrl keys edit the query and step matches.
    if state.find_active && event.flags & MOD_CTRL == 0 {
        super::find_key::find_key(state, event.code);
        follow_caret(state, rows);
        return EventOutcome::Repaint;
    }
    if event.flags & MOD_CTRL != 0 {
        if let Some(outcome) = super::on_ctrl_nav::on_ctrl_nav(state, event.code, rows, shift) {
            return outcome;
        }
        return on_ctrl(state, event.code, shift);
    }
    if let Some(outcome) = on_nav(state, event.code, rows, shift) {
        return outcome;
    }
    // Bracket and quote assistance: wrap a selection, auto-pair an opener, or
    // step over a closer. Only fires on those characters; everything else falls
    // through to normal insertion below.
    if let Some(ch) = char::from_u32(event.code) {
        if let Some(auto) = super::autoclose::autoclose(state, ch) {
            if matches!(auto, super::autoclose::Auto::Inserted) {
                state.status = b"edited";
            }
            follow_caret(state, rows);
            return EventOutcome::Repaint;
        }
    }
    let changed = match event.code {
        KEY_ESC => return EventOutcome::Close,
        // Backspace and Delete remove the selection as a unit when there is one.
        KEY_BACKSPACE => state.delete_sel() || state.backspace(),
        KEY_DELETE => state.delete_sel() || state.delete_forward(),
        KEY_ENTER => {
            state.delete_sel();
            state.insert_newline()
        }
        // Tab indents a multi-line selection; Shift+Tab dedents; otherwise it
        // inserts a soft tab stop over any selection.
        KEY_TAB if shift => state.dedent_selection(),
        KEY_TAB if state.selection_is_multiline() => state.indent_selection(),
        KEY_TAB => insert_replacing(state, b"    "),
        code if is_printable(code) => {
            let mut scratch = [0u8; 4];
            match char::from_u32(code).map(|ch| ch.encode_utf8(&mut scratch).as_bytes()) {
                Some(bytes) => insert_replacing(state, bytes),
                None => false,
            }
        }
        _ => false,
    };
    if changed {
        state.status = b"edited";
        follow_caret(state, rows);
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}

// The input framework reserves 0x1000..=0x12FF for modifier, function and
// navigation keys, so a code in that band is a key press, never text. Anything
// there that no handler above claimed, and every control character, is dropped
// rather than inserted as a glyph.
fn is_printable(code: u32) -> bool {
    if (0x1000..=0x12FF).contains(&code) {
        return false;
    }
    matches!(char::from_u32(code), Some(ch) if !ch.is_control())
}

// Typing over a selection replaces it: drop the selected text, then insert.
fn insert_replacing(state: &mut State, bytes: &[u8]) -> bool {
    state.delete_sel();
    state.insert(bytes)
}
