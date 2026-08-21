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

use nonos_app_skeleton::{EventOutcome, KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_TAB, KEY_UP};

use crate::settings::state::refresh_wifi::{connect_selected, run_wifi_scan};
use crate::settings::state::State;

use super::next_section::{next_section, prev_section};

const KEY_SPACE: u32 = 0x20;
const KEY_BACKSPACE: u32 = 0x08;
const KEY_DELETE: u32 = 0x7F;

/// Key handling while the Wi-Fi tab is active. With the passphrase editor open,
/// keys type the password, Enter joins and Escape cancels. Otherwise the arrows
/// move through the networks, Enter connects to the selected one (opening the
/// passphrase editor first for a secured network), Space rescans, and Tab or the
/// brackets cycle back to the policy tabs. A scan or a join blocks for a few
/// seconds while the radio works.
pub(super) fn on_event_wifi(state: &mut State, code: u32) -> EventOutcome {
    if state.wifi_pass_active {
        return on_passphrase_key(state, code);
    }
    match code {
        KEY_ESC => EventOutcome::Close,
        KEY_TAB => repaint_after(state, next_section),
        c if c == b']' as u32 => repaint_after(state, next_section),
        c if c == b'[' as u32 => repaint_after(state, prev_section),
        KEY_UP => {
            state.wifi_cursor = state.wifi_cursor.saturating_sub(1);
            EventOutcome::Repaint
        }
        KEY_DOWN => {
            if state.wifi_cursor + 1 < state.wifi_network_count {
                state.wifi_cursor += 1;
            }
            EventOutcome::Repaint
        }
        // Enter (or Space) always scans, matching the panel's "Enter to scan".
        KEY_ENTER | KEY_SPACE => repaint_after(state, run_wifi_scan),
        // A dedicated key connects to the highlighted network, so scanning and
        // joining never fight over the same key.
        c if c == b'c' as u32 || c == b'C' as u32 => repaint_after(state, connect_selected),
        _ => EventOutcome::Idle,
    }
}

// While the passphrase editor is open: type printable characters, delete with
// backspace, join on Enter, cancel on Escape.
fn on_passphrase_key(state: &mut State, code: u32) -> EventOutcome {
    match code {
        KEY_ESC => {
            state.wifi_pass_active = false;
            EventOutcome::Repaint
        }
        KEY_ENTER => repaint_after(state, connect_selected),
        KEY_BACKSPACE | KEY_DELETE => {
            state.wifi_pass.pop();
            EventOutcome::Repaint
        }
        c @ 0x20..=0x7E => {
            state.wifi_pass.push(c as u8);
            EventOutcome::Repaint
        }
        _ => EventOutcome::Idle,
    }
}

fn repaint_after(state: &mut State, f: fn(&mut State)) -> EventOutcome {
    f(state);
    EventOutcome::Repaint
}
