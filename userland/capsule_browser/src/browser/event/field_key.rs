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

use alloc::string::{String, ToString};

use nonos_app_skeleton::{EventOutcome, InputEvent, KEY_BACKSPACE, KEY_ENTER, MOD_SHIFT};

use crate::browser::js;
use crate::browser::keymap::printable;
use crate::browser::state::State;

use super::relayout::relayout;
use super::submit_form::submit_form;

const MAX_VALUE_LEN: usize = 512;

// Typing into the focused field edits its value attribute, redraws, and
// fires the input event. Enter submits (a textarea takes a newline).
pub(super) fn field_key(state: &mut State, id: usize, event: InputEvent) -> EventOutcome {
    let is_textarea =
        state.page_dom.as_ref().and_then(|d| d.nodes.get(id)).is_some_and(|n| n.tag == "textarea");
    let mut value: String = state
        .page_dom
        .as_ref()
        .and_then(|d| d.nodes.get(id))
        .and_then(|n| n.attr("value"))
        .unwrap_or("")
        .to_string();
    match event.code {
        KEY_ENTER if !is_textarea => {
            submit_form(state, id);
            return EventOutcome::Repaint;
        }
        KEY_ENTER => value.push('\n'),
        KEY_BACKSPACE => {
            value.pop();
        }
        code => match printable(code, event.flags & MOD_SHIFT != 0) {
            Some(b) if value.len() < MAX_VALUE_LEN => value.push(b as char),
            _ => return EventOutcome::Idle,
        },
    }
    if let Some(dom) = state.page_dom.as_mut() {
        dom.set_attr(id, "value", value);
    }
    if let (Some(dom), Some(world)) = (state.page_dom.as_mut(), state.world.as_mut()) {
        let _ = js::dispatch_event(dom, world, id, "input");
    }
    relayout(state);
    EventOutcome::Repaint
}
