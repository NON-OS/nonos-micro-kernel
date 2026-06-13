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

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ENTER, KEY_TAB};

use crate::wallet::state::{State, SEND_FIELD_AMOUNT, SEND_FIELD_NONCE, SEND_FIELD_TO};

pub fn send_input(state: &mut State, code: u32) -> Option<EventOutcome> {
    if code == KEY_TAB {
        state.send_focus = (state.send_focus + 1) % 3;
        return Some(EventOutcome::Repaint);
    }
    if code == KEY_ENTER {
        return Some(super::sign_eth::sign_eth(state));
    }
    if code == KEY_BACKSPACE {
        return backspace(state);
    }
    match state.send_focus {
        SEND_FIELD_TO => edit_to(state, code),
        SEND_FIELD_AMOUNT => super::edit_amount::edit_amount(state, code),
        SEND_FIELD_NONCE => super::edit_nonce::edit_nonce(state, code),
        _ => None,
    }
}

fn backspace(state: &mut State) -> Option<EventOutcome> {
    if state.send_focus == SEND_FIELD_TO && state.send_to_len > 0 {
        state.send_to_len -= 1;
    } else if state.send_focus == SEND_FIELD_AMOUNT {
        state.send_amount_milli_eth /= 10;
    } else if state.send_focus == SEND_FIELD_NONCE {
        state.send_nonce /= 10;
    }
    Some(EventOutcome::Repaint)
}

fn edit_to(state: &mut State, code: u32) -> Option<EventOutcome> {
    let _ = super::hex_digit::hex_digit(code)?;
    if state.send_to_len < state.send_to_hex.len() {
        state.send_to_hex[state.send_to_len] = code as u8;
        state.send_to_len += 1;
    }
    Some(EventOutcome::Repaint)
}
