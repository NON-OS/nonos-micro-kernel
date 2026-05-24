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
    EventOutcome, InputEvent, KEY_DOWN, KEY_ESC, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_TAB, KEY_UP,
    MOD_SHIFT,
};

use super::on_arrow_down::on_arrow_down;
use super::on_arrow_up::on_arrow_up;
use super::on_esc::on_esc;
use super::on_page_down::on_page_down;
use super::on_page_up::on_page_up;
use super::on_shift_tab::on_shift_tab;
use super::on_tab::on_tab;
use crate::about::state::State;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    match event.code {
        KEY_ESC => on_esc(),
        KEY_TAB if event.flags & MOD_SHIFT != 0 => on_shift_tab(state),
        KEY_TAB => on_tab(state),
        KEY_UP => on_arrow_up(state),
        KEY_DOWN => on_arrow_down(state),
        KEY_PAGE_UP => on_page_up(state),
        KEY_PAGE_DOWN => on_page_down(state),
        _ => EventOutcome::Idle,
    }
}
